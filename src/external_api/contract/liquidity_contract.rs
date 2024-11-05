use std::sync::Arc;

use ethers::{
    contract::abigen,
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::Wallet,
    types::H256,
};

use crate::utils::config::Config;

use super::{
    interface::BlockchainError,
    utils::{get_client, get_client_with_signer},
};

abigen!(Liquidity, "abi/Liquidity.json",);

pub async fn get_liquidity_contract(
    rpc_url: &str,
) -> Result<liquidity::Liquidity<Provider<Http>>, BlockchainError> {
    let client = get_client(rpc_url).await?;
    let contract = Liquidity::new(Config::load().liquidity_contract_address, client);
    Ok(contract)
}

pub async fn get_liquidity_contract_with_signer(
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
