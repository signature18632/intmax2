use async_trait::async_trait;
use ethers::types::H256;
use intmax2_zkp::ethereum_types::{address::Address, bytes32::Bytes32, u256::U256};

#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[async_trait]
pub trait ContractInterface {
    async fn deposit(
        &self,
        rpc_url: &str, // rpc url is given in runtime
        signer_private_key: H256,
        pubkey_salt_hash: Bytes32,
        token_address: Address,
        amount: U256,
    ) -> Result<(), BlockchainError>;
}
