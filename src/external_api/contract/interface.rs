use async_trait::async_trait;
use ethers::types::H256;
use intmax2_zkp::ethereum_types::{address::Address, bytes32::Bytes32, u256::U256};

#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error("Insufficient funds")]
    InsufficientFunds,
}

#[async_trait]
pub trait Contract {
    async fn deposit(
        &self,
        signer_private_key: H256,
        pubkey_salt_hash: Bytes32,
        token_address: Address,
        amount: U256,
    ) -> Result<(), ContractError>;
}
