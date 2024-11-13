use intmax2_core_sdk::external_api::{
    balance_prover::local::LocalBalanceProver, block_builder::local::LocalBlockBuilder,
    block_validity_prover::local::LocalBlockValidityProver, contract::local::LocalContract,
    store_vault_server::local::LocalStoreVaultServer,
    withdrawal_aggregator::local::LocalWithdrawalAggregator,
};

pub struct State {
    contract: LocalContract,
    store_vault_server: LocalStoreVaultServer,
    validity_prover: LocalBlockValidityProver,
    block_builder: LocalBlockBuilder,
    balance_prover: LocalBalanceProver,
    withdrawal_aggregator: LocalWithdrawalAggregator,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let contract = LocalContract::new();
        let store_vault_server = LocalStoreVaultServer::new();
        let validity_prover = LocalBlockValidityProver::new(contract.0.clone());
        let block_builder = LocalBlockBuilder::new(
            contract.0.clone(),
            validity_prover.inner_block_validity_prover.clone(),
        );
        let balance_prover = LocalBalanceProver::new()?;
        let withdrawal_aggregator = LocalWithdrawalAggregator::new()?;
        Ok(Self {
            contract,
            store_vault_server,
            validity_prover,
            block_builder,
            balance_prover,
            withdrawal_aggregator,
        })
    }
}
