use intmax2_client_sdk::external_api::contract::rollup_contract::RollupContract;
use intmax2_interfaces::api::validity_prover::interface::{AccountInfo, DepositInfo};
use intmax2_zkp::{
    circuits::validity::validity_pis::ValidityPublicInputs,
    common::{
        block::Block,
        trees::{
            account_tree::AccountMembershipProof, block_hash_tree::BlockHashMerkleProof,
            deposit_tree::DepositMerkleProof, sender_tree::SenderLeaf,
        },
        witness::{update_witness::UpdateWitness, validity_witness::ValidityWitness},
    },
    constants::{ACCOUNT_TREE_HEIGHT, BLOCK_HASH_TREE_HEIGHT, DEPOSIT_TREE_HEIGHT},
    ethereum_types::{bytes32::Bytes32, u256::U256, u32limb_trait::U32LimbTrait as _},
    utils::trees::{
        incremental_merkle_tree::IncrementalMerkleProof,
        indexed_merkle_tree::leaf::IndexedMerkleLeaf, merkle_tree::MerkleProof,
    },
};

use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::time::interval;

use super::{error::ValidityProverError, observer::Observer};
use crate::{
    trees::{
        account_tree::HistoricalAccountTree,
        block_tree::HistoricalBlockHashTree,
        deposit_hash_tree::{DepositHash, HistoricalDepositHashTree},
        merkle_tree::sql_merkle_tree::SqlMerkleTree,
        update::{to_block_witness, update_trees},
    },
    Env,
};

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

type ADB = SqlMerkleTree<IndexedMerkleLeaf>;
type BDB = SqlMerkleTree<Bytes32>;
type DDB = SqlMerkleTree<DepositHash>;

const ACCOUNT_DB_TAG: u32 = 1;
const BLOCK_DB_TAG: u32 = 2;
const DEPOSIT_DB_TAG: u32 = 3;

#[derive(Clone)]
pub struct Config {
    pub sync_interval: u64,
}

#[derive(Clone)]
pub struct WitnessGenerator {
    config: Config,
    observer: Observer,
    account_tree: HistoricalAccountTree<ADB>,
    block_tree: HistoricalBlockHashTree<BDB>,
    deposit_hash_tree: HistoricalDepositHashTree<DDB>,
    pool: PgPool,
}

impl WitnessGenerator {
    pub async fn new(env: &Env) -> Result<Self, ValidityProverError> {
        let config = Config {
            sync_interval: env.sync_interval,
        };

        let rollup_contract = RollupContract::new(
            &env.l2_rpc_url,
            env.l2_chain_id,
            env.rollup_contract_address,
            env.rollup_contract_deployed_block_number,
        );
        let observer = Observer::new(
            rollup_contract,
            &env.database_url,
            env.database_max_connections,
            env.database_timeout,
        )
        .await?;

        let pool = PgPoolOptions::new()
            .max_connections(env.database_max_connections)
            .idle_timeout(Duration::from_secs(env.database_timeout))
            .connect(&env.database_url)
            .await?;

        let account_db = SqlMerkleTree::new(&env.database_url, ACCOUNT_DB_TAG, ACCOUNT_TREE_HEIGHT);
        let account_tree = HistoricalAccountTree::initialize(account_db).await?;

        let block_db = SqlMerkleTree::new(&env.database_url, BLOCK_DB_TAG, BLOCK_HASH_TREE_HEIGHT);
        let block_tree = HistoricalBlockHashTree::new(block_db);
        let last_timestamp = block_tree.get_last_timestamp().await?;
        if last_timestamp == 0 {
            let len = block_tree.len(last_timestamp).await?;
            if len == 0 {
                block_tree
                    .push(last_timestamp, Block::genesis().hash())
                    .await?;
            }
        }

        let deposit_db = SqlMerkleTree::new(&env.database_url, DEPOSIT_DB_TAG, DEPOSIT_TREE_HEIGHT);
        let deposit_hash_tree = HistoricalDepositHashTree::new(deposit_db);

        log::info!("block tree len: {}", block_tree.len(last_timestamp).await?);
        log::info!(
            "deposit tree len: {}",
            deposit_hash_tree.len(last_timestamp).await?
        );
        log::info!(
            "account tree len: {}",
            account_tree.len(last_timestamp).await?
        );

        Ok(Self {
            config,
            observer,
            pool,
            account_tree,
            block_tree,
            deposit_hash_tree,
        })
    }

    pub async fn sync_observer(&self) -> Result<(), ValidityProverError> {
        self.observer.sync().await?;
        Ok(())
    }

    pub async fn get_validity_proof(
        &self,
        block_number: u32,
    ) -> Result<Option<ProofWithPublicInputs<F, C, D>>, ValidityProverError> {
        let record = sqlx::query!(
            "SELECT proof FROM validity_proofs WHERE block_number = $1",
            block_number as i32
        )
        .fetch_optional(&self.pool)
        .await?;
        match record {
            Some(r) => {
                let proof: ProofWithPublicInputs<F, C, D> = serde_json::from_value(r.proof)?;
                Ok(Some(proof))
            }
            None => Ok(None),
        }
    }

    pub async fn sync(&self) -> Result<(), ValidityProverError> {
        log::info!("Start sync validity prover");
        self.sync_observer().await?;

        let last_block_number = self.get_last_block_number().await?;
        let next_block_number = self.observer.get_next_block_number().await?;

        for block_number in (last_block_number + 1)..next_block_number {
            log::info!(
                "Sync validity prover: syncing block number {}",
                block_number
            );

            let full_block = self.observer.get_full_block(block_number).await?;

            let deposit_events = self
                .observer
                .get_deposits_between_blocks(block_number)
                .await?;
            for event in deposit_events {
                self.deposit_hash_tree
                    .push(block_number as u64, DepositHash(event.deposit_hash))
                    .await?;
            }
            let deposit_tree_root = self.deposit_hash_tree.get_root(block_number as u64).await?;
            if full_block.block.deposit_tree_root != deposit_tree_root {
                return Err(ValidityProverError::DepositTreeRootMismatch(
                    full_block.block.deposit_tree_root,
                    deposit_tree_root,
                ));
            }

            let block_witness = to_block_witness(
                &full_block,
                block_number as u64,
                &self.account_tree,
                &self.block_tree,
            )
            .await
            .map_err(|e| ValidityProverError::BlockWitnessGenerationError(e.to_string()))?;

            // Caution! This change the state of the account tree and block tree
            // TODO: atomic update
            let validity_witness = update_trees(
                &block_witness,
                block_number as u64,
                &self.account_tree,
                &self.block_tree,
            )
            .await
            .map_err(|e| ValidityProverError::FailedToUpdateTrees(e.to_string()))?;

            // Update database state
            let mut tx = self.pool.begin().await?;
            sqlx::query!(
                "INSERT INTO validity_state (block_number, validity_witness, sender_leaves) VALUES ($1, $2, $3)",
                block_number as i32,
                serde_json::to_value(&validity_witness)?,
                serde_json::to_value(&block_witness.get_sender_tree().leaves())?,
            )
            .execute(tx.as_mut())
            .await?;

            sqlx::query!(
                "INSERT INTO prover_tasks (block_number, assigned, completed) VALUES ($1, FALSE, FALSE)
                 ON CONFLICT (block_number) DO NOTHING",
                block_number as i32
            )
            .execute(tx.as_mut()).await?;

            let tx_tree_root = full_block.signature.tx_tree_root;
            if tx_tree_root != Bytes32::default()
                && validity_witness.to_validity_pis().unwrap().is_valid_block
            {
                sqlx::query!(
                    "INSERT INTO tx_tree_roots (tx_tree_root, block_number) VALUES ($1, $2)
                     ON CONFLICT (tx_tree_root) DO UPDATE SET block_number = $2",
                    tx_tree_root.to_bytes_be(),
                    block_number as i32
                )
                .execute(tx.as_mut())
                .await?;
            }

            tx.commit().await?;
        }
        log::info!("End of sync validity prover");
        Ok(())
    }

    pub async fn get_update_witness(
        &self,
        pubkey: U256,
        root_block_number: u32,
        leaf_block_number: u32,
        is_prev_account_tree: bool,
    ) -> Result<UpdateWitness<F, C, D>, ValidityProverError> {
        let validity_proof = self.get_validity_proof(root_block_number).await?.ok_or(
            ValidityProverError::ValidityProofNotFound(root_block_number),
        )?;

        let block_merkle_proof = self
            .get_block_merkle_proof(root_block_number, leaf_block_number)
            .await?;

        let account_tree_block_number = if is_prev_account_tree {
            root_block_number - 1
        } else {
            root_block_number
        };

        let account_membership_proof = self
            .get_account_membership_proof(account_tree_block_number, pubkey)
            .await?;

        Ok(UpdateWitness {
            is_prev_account_tree,
            validity_proof,
            block_merkle_proof,
            account_membership_proof,
        })
    }

    pub async fn get_account_id(&self, pubkey: U256) -> Result<Option<u64>, ValidityProverError> {
        let last_block_number = self.get_last_block_number().await?;
        let index = self
            .account_tree
            .index(last_block_number as u64, pubkey)
            .await?;
        Ok(index)
    }

    pub async fn get_account_info(&self, pubkey: U256) -> Result<AccountInfo, ValidityProverError> {
        let block_number = self.get_last_block_number().await?;
        let account_id = self.account_tree.index(block_number as u64, pubkey).await?;
        Ok(AccountInfo {
            block_number,
            account_id,
        })
    }

    pub async fn get_deposit_info(
        &self,
        deposit_hash: Bytes32,
    ) -> Result<Option<DepositInfo>, ValidityProverError> {
        let deposit_info = self.observer.get_deposit_info(deposit_hash).await?;
        Ok(deposit_info)
    }

    pub async fn get_block_number_by_tx_tree_root(
        &self,
        tx_tree_root: Bytes32,
    ) -> Result<Option<u32>, ValidityProverError> {
        let record = sqlx::query!(
            "SELECT block_number FROM tx_tree_roots WHERE tx_tree_root = $1",
            tx_tree_root.to_bytes_be()
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|r| r.block_number as u32))
    }

    pub async fn get_validity_pis(
        &self,
        block_number: u32,
    ) -> Result<Option<ValidityPublicInputs>, ValidityProverError> {
        if block_number == 0 {
            return Ok(Some(ValidityPublicInputs::genesis()));
        }
        let record = sqlx::query!(
            r#"
            SELECT validity_witness
            FROM validity_state
            WHERE block_number = $1
            "#,
            block_number as i32,
        )
        .fetch_optional(&self.pool)
        .await?;
        let validity_pis = match record {
            Some(record) => {
                let validity_witness: ValidityWitness =
                    serde_json::from_value(record.validity_witness.clone())?;
                let validity_pis = validity_witness.to_validity_pis()?;
                Some(validity_pis)
            }
            None => None,
        };
        Ok(validity_pis)
    }

    pub async fn get_sender_leaves(
        &self,
        block_number: u32,
    ) -> Result<Option<Vec<SenderLeaf>>, ValidityProverError> {
        let record = sqlx::query!(
            "SELECT sender_leaves FROM validity_state WHERE block_number = $1",
            block_number as i32
        )
        .fetch_optional(&self.pool)
        .await?;

        match record {
            Some(r) => {
                let leaves: Vec<SenderLeaf> = serde_json::from_value(r.sender_leaves)?;
                Ok(Some(leaves))
            }
            None => Ok(None),
        }
    }

    pub async fn get_block_merkle_proof(
        &self,
        root_block_number: u32,
        leaf_block_number: u32,
    ) -> Result<BlockHashMerkleProof, ValidityProverError> {
        if leaf_block_number > root_block_number {
            return Err(ValidityProverError::InputError(
                "leaf_block_number should be smaller than root_block_number".to_string(),
            ));
        }
        let proof = self
            .block_tree
            .prove(root_block_number as u64, leaf_block_number as u64)
            .await?;
        Ok(proof)
    }

    async fn get_account_membership_proof(
        &self,
        block_number: u32,
        pubkey: U256,
    ) -> Result<AccountMembershipProof, ValidityProverError> {
        let proof = self
            .account_tree
            .prove_membership(block_number as u64, pubkey)
            .await?;
        Ok(proof)
    }

    pub async fn get_latest_validity_proof_block_number(&self) -> Result<u32, ValidityProverError> {
        let record = sqlx::query!(
            r#"
            SELECT block_number
            FROM validity_proofs
            ORDER BY block_number DESC
            LIMIT 1
            "#,
        )
        .fetch_optional(&self.pool)
        .await?;
        let block_number = record.map(|r| r.block_number as u32).unwrap_or(0);
        Ok(block_number)
    }

    pub async fn get_last_block_number(&self) -> Result<u32, ValidityProverError> {
        let record =
            sqlx::query!("SELECT MAX(block_number) as last_block_number FROM validity_state")
                .fetch_optional(&self.pool)
                .await?;
        let last_block_number = record
            .map(|r| r.last_block_number) // Option<Option<i32>>
            .flatten() // Option<i32>
            .unwrap_or(0); // i32

        Ok(last_block_number as u32)
    }

    pub async fn get_next_deposit_index(&self) -> Result<u32, ValidityProverError> {
        let deposit_index = self.observer.get_next_deposit_index().await?;
        Ok(deposit_index)
    }

    pub async fn get_deposit_merkle_proof(
        &self,
        block_number: u32,
        deposit_index: u32,
    ) -> Result<DepositMerkleProof, ValidityProverError> {
        let proof = self
            .deposit_hash_tree
            .prove(block_number as u64, deposit_index as u64)
            .await?;
        Ok(IncrementalMerkleProof(MerkleProof {
            siblings: proof.0.siblings,
        }))
    }

    pub fn job(self) {
        let is_syncing = Arc::new(AtomicBool::new(false));
        let is_syncing_clone = is_syncing.clone();
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(self.config.sync_interval));
            loop {
                interval.tick().await;

                // Skip if previous task is still running
                if is_syncing_clone.load(Ordering::SeqCst) {
                    log::warn!("Previous sync task is still running, skipping this interval");
                    continue;
                }

                is_syncing_clone.store(true, Ordering::SeqCst);

                match self.sync().await {
                    Ok(_) => {
                        log::debug!("Sync task completed successfully");
                    }
                    Err(e) => {
                        log::error!("Error in sync task: {:?}", e);
                    }
                }
                // Reset the flag after task completion
                is_syncing_clone.store(false, Ordering::SeqCst);
            }
        });
    }
}
