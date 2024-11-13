use async_trait::async_trait;
use intmax2_zkp::{
    circuits::validity::validity_pis::ValidityPublicInputs,
    common::{
        trees::{
            block_hash_tree::BlockHashMerkleProof, deposit_tree::DepositMerkleProof,
            sender_tree::SenderLeaf,
        },
        witness::update_witness::UpdateWitness,
    },
    ethereum_types::{bytes32::Bytes32, u256::U256},
};
use plonky2::{field::goldilocks_field::GoldilocksField, plonk::config::PoseidonGoldilocksConfig};
use reqwest_wasm::Client;

use super::types::GetDepositMerkleProofResponse;
use crate::external_api::block_validity_prover::{
    interface::BlockValidityInterface, test_server::types::*,
};
use crate::external_api::common::error::ServerError;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

#[derive(Debug, Clone)]
pub struct TestBlockValidityProver {
    base_url: String,
    client: Client,
}

impl TestBlockValidityProver {
    pub fn new(base_url: String) -> Self {
        TestBlockValidityProver {
            base_url,
            client: Client::new(),
        }
    }

    async fn get_request<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<T, ServerError> {
        let mut url = format!("{}{}", self.base_url, endpoint);

        let response = if let Some(params) = query {
            self.client.get(&url).query(&params).send().await
        } else {
            self.client.get(&url).send().await
        }
        .map_err(|e| ServerError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| ServerError::DeserializationError(e.to_string()))
        } else {
            Err(ServerError::ServerError(response.status().to_string()))
        }
    }

    pub async fn sync(&self) -> Result<(), ServerError> {
        self.get_request::<()>("/block-validity-prover/sync", None)
            .await
    }
}

#[async_trait(?Send)]
impl BlockValidityInterface for TestBlockValidityProver {
    async fn block_number(&self) -> Result<u32, ServerError> {
        let response: GetBlockNumberResponse = self
            .get_request("/block-validity-prover/block-number", None)
            .await?;
        Ok(response.block_number)
    }

    async fn get_account_id(&self, pubkey: U256) -> Result<Option<usize>, ServerError> {
        let query = vec![("pubkey", pubkey.to_string())];
        let response: GetAccountIdResponse = self
            .get_request("/block-validity-prover/get-account-id", Some(query))
            .await?;
        Ok(response.account_id)
    }

    async fn get_update_witness(
        &self,
        pubkey: U256,
        root_block_number: u32,
        leaf_block_number: u32,
        is_prev_account_tree: bool,
    ) -> Result<UpdateWitness<F, C, D>, ServerError> {
        let query = vec![
            ("pubkey", pubkey.to_string()),
            ("rootBlockNumber", root_block_number.to_string()),
            ("leafBlockNumber", leaf_block_number.to_string()),
            ("isPrevAccountTree", is_prev_account_tree.to_string()),
        ];
        let response: GetUpdateWitnessResponse = self
            .get_request("/block-validity-prover/get-update-witness", Some(query))
            .await?;
        Ok(response.update_witness)
    }

    async fn get_deposit_index_and_block_number(
        &self,
        deposit_hash: Bytes32,
    ) -> Result<Option<(usize, u32)>, ServerError> {
        let query = vec![("depositHash", deposit_hash.to_string())];
        let response: GetDepositIndexAndBlockNumberResponse = self
            .get_request(
                "/block-validity-prover/get-deposit-index-and-block-number",
                Some(query),
            )
            .await?;
        Ok(response.deposit_index_and_block_number)
    }

    async fn get_block_number_by_tx_tree_root(
        &self,
        tx_tree_root: Bytes32,
    ) -> Result<Option<u32>, ServerError> {
        let query = vec![("txTreeRoot", tx_tree_root.to_string())];
        let response: GetBlockNumberByTxTreeRootResponse = self
            .get_request(
                "/block-validity-prover/get-block-number-by-tx-tree-root",
                Some(query),
            )
            .await?;
        Ok(response.block_number)
    }

    async fn get_validity_pis(
        &self,
        block_number: u32,
    ) -> Result<Option<ValidityPublicInputs>, ServerError> {
        let query = vec![("blockNumber", block_number.to_string())];
        let response: GetValidityPisResponse = self
            .get_request("/block-validity-prover/get-validity-pis", Some(query))
            .await?;
        Ok(response.validity_pis)
    }

    async fn get_sender_leaves(
        &self,
        block_number: u32,
    ) -> Result<Option<Vec<SenderLeaf>>, ServerError> {
        let query = vec![("blockNumber", block_number.to_string())];
        let response: GetSenderLeavesResponse = self
            .get_request("/block-validity-prover/get-sender-leaves", Some(query))
            .await?;
        Ok(response.sender_leaves)
    }

    async fn get_block_merkle_proof(
        &self,
        root_block_number: u32,
        leaf_block_number: u32,
    ) -> Result<BlockHashMerkleProof, ServerError> {
        let query = vec![
            ("rootBlockNumber", root_block_number.to_string()),
            ("leafBlockNumber", leaf_block_number.to_string()),
        ];
        let response: GetBlockMerkleProofResponse = self
            .get_request("/block-validity-prover/get-block-merkle-proof", Some(query))
            .await?;
        Ok(response.block_merkle_proof)
    }

    async fn get_deposit_merkle_proof(
        &self,
        block_number: u32,
        deposit_index: usize,
    ) -> Result<DepositMerkleProof, ServerError> {
        let query = vec![
            ("blockNumber", block_number.to_string()),
            ("depositIndex", deposit_index.to_string()),
        ];
        let response: GetDepositMerkleProofResponse = self
            .get_request(
                "/block-validity-prover/get-deposit-merkle-proof",
                Some(query),
            )
            .await?;
        Ok(response.deposit_merkle_proof)
    }
}
