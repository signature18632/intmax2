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
use intmax2_zkp::ethereum_types::{bytes32::Bytes32, u256::U256, u32limb_trait::U32LimbTrait as _};

use crate::utils::config::Config;

use super::{
    handlers::{handle_contract_call, set_gas_price},
    interface::{BlockchainError, ContractInterface},
    utils::{get_address, get_client, get_client_with_signer},
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

#[derive(Debug, Clone)]
pub struct LiquidityContract;

#[async_trait]
impl ContractInterface for LiquidityContract {
    async fn deposit_native_token(
        &self,
        rpc_url: &str,
        signer_private_key: H256,
        pubkey_salt_hash: Bytes32,
        amount: U256,
    ) -> Result<(), BlockchainError> {
        let contract = get_liquidity_contract_with_signer(rpc_url, signer_private_key).await?;
        let recipient_salt_hash: [u8; 32] = pubkey_salt_hash.to_bytes_be().try_into().unwrap();
        let amount = ethers::types::U256::from_big_endian(&amount.to_bytes_be());
        let mut tx = contract
            .deposit_native_token(recipient_salt_hash)
            .value(amount);
        set_gas_price(&mut tx).await?;
        handle_contract_call(
            tx,
            get_address(signer_private_key),
            "depositer",
            "deposit_native_token",
        )
        .await?;
        Ok(())
    }
}
