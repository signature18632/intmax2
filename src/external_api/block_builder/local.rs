use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use intmax2_zkp::{
    common::{
        block_builder::{BlockProposal, UserSignature},
        signature::flatten::FlatG2,
        tx::Tx,
    },
    ethereum_types::u256::U256,
    mock::{
        block_builder::BlockBuilder as InnerBlockBuilder,
        block_validity_prover::BlockValidityProver, contract::MockContract,
    },
};
use plonky2::{field::goldilocks_field::GoldilocksField, plonk::config::PoseidonGoldilocksConfig};

use crate::external_api::common::error::ServerError;

use super::interface::{BlockBuilderInterface, FeeProof};

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

pub struct LocalBlockBuilder {
    pub contract: Arc<Mutex<MockContract>>,
    pub validity_prover: Arc<BlockValidityProver<F, C, D>>,
    pub inner_block_builder: Arc<Mutex<InnerBlockBuilder>>,
    pub state: Arc<Mutex<State>>,
}

pub struct State {
    is_registration_block: Option<bool>,
    txs: Vec<(U256, Tx)>,
    proposals: Option<Vec<BlockProposal>>,
    signatures: Vec<UserSignature>,
}

impl State {
    pub fn is_empty(&self) -> bool {
        self.is_registration_block.is_none() && self.txs.is_empty() && self.proposals.is_none()
    }

    pub fn clear(&mut self) {
        self.is_registration_block = None;
        self.txs.clear();
        self.proposals = None;
        self.signatures.clear();
    }
}

impl LocalBlockBuilder {
    pub fn construct_block(&self) -> Result<(), ServerError> {
        let is_registration_block = self.state.lock().unwrap().is_registration_block.unwrap();
        let txs = self.state.lock().unwrap().txs.clone();

        let mut contract = self.contract.lock().unwrap();
        let validity_prover = &self.validity_prover;
        let proposals = self
            .inner_block_builder
            .lock()
            .unwrap()
            .propose(
                &mut *contract,
                &*validity_prover,
                is_registration_block,
                txs,
            )
            .map_err(|e| ServerError::InternalError(format!("Block construction {:?}", e)))?;

        self.state.lock().unwrap().proposals = Some(proposals);
        Ok(())
    }

    pub fn post_block(&self) -> Result<(), ServerError> {
        let mut contract = self.contract.lock().unwrap();
        let validity_prover = &self.validity_prover;
        let signatures = self.state.lock().unwrap().signatures.clone();
        self.inner_block_builder
            .lock()
            .unwrap()
            .post_block(&mut *contract, &validity_prover, signatures)
            .map_err(|e| ServerError::InternalError(format!("Post block {:?}", e)))?;
        self.state.lock().unwrap().clear();
        Ok(())
    }
}

#[async_trait]
impl BlockBuilderInterface for LocalBlockBuilder {
    async fn initialize_tx(
        &self,
        pubkey: U256,
        tx: Tx,
        _fee_proof: FeeProof,
    ) -> Result<(), ServerError> {
        let account_id = self.validity_prover.get_account_id(pubkey);
        let is_registration_block = account_id.is_none();
        if self.state.lock().unwrap().is_registration_block.is_none() {
            self.state.lock().unwrap().is_registration_block = Some(is_registration_block);
        } else if self.state.lock().unwrap().is_registration_block != Some(is_registration_block) {
            return Err(ServerError::InternalError(
                "Cannot mix registration and non-registration txs".to_string(),
            ));
        }
        self.state.lock().unwrap().txs.push((pubkey, tx));
        Ok(())
    }

    async fn query_tx(&self, pubkey: U256, tx: Tx) -> Result<Option<BlockProposal>, ServerError> {
        let tx_index = self
            .state
            .lock()
            .unwrap()
            .txs
            .iter()
            .position(|(p, t)| *p == pubkey && *t == tx)
            .ok_or(ServerError::InternalError("Query tx not found".to_string()))?;
        if self.state.lock().unwrap().proposals.is_none() {
            return Ok(None);
        }
        let proposals = self.state.lock().unwrap().proposals.clone().unwrap();
        Ok(proposals.get(tx_index).cloned())
    }

    async fn post_signature(
        &self,
        pubkey: U256,
        _tx: Tx,
        signature: FlatG2,
    ) -> Result<(), ServerError> {
        let user_signatre = UserSignature { pubkey, signature };
        self.state.lock().unwrap().signatures.push(user_signatre);
        Ok(())
    }
}
