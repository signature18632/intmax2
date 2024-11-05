use std::sync::Arc;

use ethers::{
    core::k256::{ecdsa::SigningKey, SecretKey},
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{Signer as _, Wallet},
    types::H256,
};

use crate::utils::config::Config;

use super::interface::BlockchainError;

async fn get_provider(rpc_url: &str) -> Result<Provider<Http>, BlockchainError> {
    let provider = Provider::<Http>::try_from(rpc_url)
        .map_err(|_| BlockchainError::InternalError("Failed to parse RPC_URL".to_string()))?;
    Ok(provider)
}

pub async fn get_client(rpc_url: &str) -> Result<Arc<Provider<Http>>, BlockchainError> {
    Ok(Arc::new(get_provider(rpc_url).await?))
}

pub async fn get_wallet(private_key: H256) -> Result<Wallet<SigningKey>, BlockchainError> {
    let key = SecretKey::from_bytes(private_key.as_bytes().into()).unwrap();
    let wallet = Wallet::from(key).with_chain_id(Config::load().chain_id);
    Ok(wallet)
}

pub async fn get_client_with_signer(
    rpc_url: &str,
    private_key: H256,
) -> Result<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>, BlockchainError> {
    let provider = get_provider(rpc_url).await?;
    let wallet = get_wallet(private_key).await?;
    let client = SignerMiddleware::new(provider, wallet);
    Ok(client)
}
