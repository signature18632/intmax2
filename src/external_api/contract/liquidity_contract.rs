use std::sync::Arc;

use async_trait::async_trait;
use ethers::{
    contract::abigen,
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::Wallet,
    types::H256,
};
use intmax2_zkp::ethereum_types::{address::Address, bytes32::Bytes32, u256::U256};

use crate::utils::config::Config;

use super::{
    interface::{BlockchainError, ContractInterface},
    utils::{get_client, get_client_with_signer},
};

abigen!(Liquidity, "abi/Liquidity.json",);

async fn get_liquidity_contract(
    rpc_url: &str,
) -> Result<liquidity::Liquidity<Provider<Http>>, BlockchainError> {
    let client = get_client(rpc_url).await?;
    let contract = Liquidity::new(Config::load().liquidity_contract_address, client);
    Ok(contract)
}

async fn get_liquidity_contract_with_signer(
    rpc_url: &str,
    private_key: H256,
) -> Result<
    liquidity::Liquidity<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    BlockchainError,
> {
    let client = get_client_with_signer(rpc_url, private_key).await?;
    let contract = Liquidity::new(Config::load().liquidity_contract_address, Arc::new(client));
    Ok(contract)
}

#[derive(Debug, Clone)]
pub struct LiquidityContract;

#[async_trait]
impl ContractInterface for LiquidityContract {
    async fn deposit(
        &self,
        rpc_url: &str,
        signer_private_key: H256,
        pubkey_salt_hash: Bytes32,
        token_address: Address,
        amount: U256,
    ) -> Result<(), BlockchainError> {
        todo!()
    }
}
