use intmax2_core_sdk::{
    client::client::Client,
    external_api::{
        balance_prover::local::LocalBalanceProver,
        block_builder::server::server::BlockBuilder,
        block_validity_prover::{
            local::LocalBlockValidityProver, server::block_validity_prover::BlockValidityProver,
        },
        contract::liquidity_contract::LiquidityContract,
        store_vault_server::server::store_vault_server::StoreVaultServer,
    },
};

type BC = LiquidityContract;
type BB = BlockBuilder;
type S = StoreVaultServer;
type V = BlockValidityProver;
type B = LocalBalanceProver;

pub fn get_client() -> anyhow::Result<Client<BC, BB, S, V, B>> {
    let contract = LiquidityContract;
    let block_builder = BB::new();
    let store_vault_server = S::new();
    let block_validity_prover = V::new();

    // let local_block_validity_prover = LocalBlockValidityProver::new();
    let balance_prover = B::new();

    todo!()
}
