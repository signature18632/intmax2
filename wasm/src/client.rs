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

type BB = BlockBuilder;
type S = StoreVaultServer;
type V = BlockValidityProver;
type B = BalanceProver;
type W = WithdrawalAggregatorServer;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    store_vault_server_url: String,
    block_validity_prover_url: String,
    balance_prover_url: String,
    withdrawal_aggregator_url: String,
    config: ClientConfig,
}

impl Config {
    pub fn new(
        store_vault_server_url: String,
        block_validity_prover_url: String,
        balance_prover_url: String,
        withdrawal_aggregator_url: String,
        deposit_timeout: u64,
        tx_timeout: u64,
        max_tx_query_times: usize,
        tx_query_interval: u64,
    ) -> Self {
        let config = ClientConfig {
            deposit_timeout,
            tx_timeout,
            max_tx_query_times,
            tx_query_interval,
        };
        Config {
            store_vault_server_url,
            block_validity_prover_url,
            balance_prover_url,
            withdrawal_aggregator_url,
            config,
        }
    }
}

pub fn get_client(config: Config) -> anyhow::Result<Client<BB, S, V, B, W>> {
    let block_builder = BB::new();
    let store_vault_server = S::new(config.store_vault_server_url)?;
    let validity_prover = V::new(config.block_validity_prover_url)?;
    let balance_prover = B::new(config.balance_prover_url);
    let withdrawal_aggregator = W::new(config.withdrawal_aggregator_url);

    let config = ClientConfig {
        deposit_timeout: 3600,
        tx_timeout: 60,
        max_tx_query_times: 50,
        tx_query_interval: 1,
    };

    let client = Client {
        block_builder,
        store_vault_server,
        validity_prover,
        balance_prover,
        withdrawal_aggregator,
        config,
    };

    Ok(client)
}
