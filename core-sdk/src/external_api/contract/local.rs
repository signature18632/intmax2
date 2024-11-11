use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use ethers::types::H256;
use intmax2_zkp::{
    ethereum_types::{bytes32::Bytes32, u256::U256},
    mock::contract::MockContract,
};

use super::interface::{BlockchainError, ContractInterface};

pub struct LocalContract(pub Arc<Mutex<MockContract>>);

impl LocalContract {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(MockContract::new())))
    }
}

#[async_trait(?Send)]
impl ContractInterface for LocalContract {
    async fn deposit_native_token(
        &self,
        _rpc_url: &str,
        _signer_private_key: H256,
        pubkey_salt_hash: Bytes32,
        amount: U256,
    ) -> Result<(), BlockchainError> {
        self.0.lock().unwrap().deposit(pubkey_salt_hash, 0, amount);
        Ok(())
    }
}
