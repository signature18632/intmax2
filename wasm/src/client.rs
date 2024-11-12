use intmax2_core_sdk::{
    client::{client::Client, config::ClientConfig},
    external_api::{
        balance_prover::server::balance_prover::BalanceProver,
        block_builder::server::server::BlockBuilder,
        block_validity_prover::server::block_validity_prover::BlockValidityProver,
        store_vault_server::server::store_vault_server::StoreVaultServer,
        withdrawal_aggregator::server::WithdrawalAggregatorServer,
    },
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

type BB = BlockBuilder;
type S = StoreVaultServer;
type V = BlockValidityProver;
type B = BalanceProver;
type W = WithdrawalAggregatorServer;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct Config {
    pub store_vault_server_url: String,
    pub block_validity_prover_url: String,
    pub balance_prover_url: String,
    pub withdrawal_aggregator_url: String,
    pub deposit_timeout: u64,
    pub tx_timeout: u64,
    pub max_tx_query_times: usize,
    pub tx_query_interval: u64,
}

pub fn get_client(config: Config) -> Client<BB, S, V, B, W> {
    let block_builder = BB::new();
    let store_vault_server = S::new(config.store_vault_server_url);
    let validity_prover = V::new(config.block_validity_prover_url);
    let balance_prover = B::new(config.balance_prover_url);
    let withdrawal_aggregator = W::new(config.withdrawal_aggregator_url);

    let client_config = ClientConfig {
        deposit_timeout: config.deposit_timeout,
        tx_timeout: config.tx_timeout,
        max_tx_query_times: config.max_tx_query_times,
        tx_query_interval: config.tx_query_interval,
    };

    Client {
        block_builder,
        store_vault_server,
        validity_prover,
        balance_prover,
        withdrawal_aggregator,
        config: client_config,
    }
}
