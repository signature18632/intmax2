use ethers::types::H256;
use intmax2_core_sdk::{
    client::{client::Client, config::ClientConfig},
    external_api::{
        balance_prover::local::LocalBalanceProver, block_builder::server::server::BlockBuilder,
        block_validity_prover::server::block_validity_prover::BlockValidityProver,
        contract::liquidity_contract::LiquidityContract,
        store_vault_server::server::store_vault_server::StoreVaultServer,
    },
};
use intmax2_zkp::{
    common::{
        generic_address::GenericAddress, salt::Salt, signature::key_set::KeySet, transfer::Transfer,
    },
    ethereum_types::u256::U256,
};
use num_bigint::BigUint;

type BC = LiquidityContract;
type BB = BlockBuilder;
type S = StoreVaultServer;
type V = BlockValidityProver;
type B = LocalBalanceProver;

pub fn get_client() -> anyhow::Result<Client<BC, BB, S, V, B>> {
    let contract = LiquidityContract;
    let block_builder = BB::new();
    let store_vault_server = S::new()?;
    let validity_prover = V::new()?;
    let balance_prover = B::new()?;

    let config = ClientConfig {
        deposit_timeout: 3600,
        tx_timeout: 60,
        max_tx_query_times: 50,
        tx_query_interval: 1,
    };

    let client = Client {
        contract,
        block_builder,
        store_vault_server,
        validity_prover,
        balance_prover,
        config,
    };

    Ok(client)
}

pub async fn deposit(
    rpc_url: &str,
    eth_private_key: H256,
    private_key: H256,
    amount: U256,
    token_index: u32,
) -> anyhow::Result<()> {
    let client = get_client()?;
    let key = h256_to_keyset(private_key);
    client
        .deposit(rpc_url, eth_private_key, key, token_index, amount)
        .await?;
    Ok(())
}

pub async fn tx(private_key: H256, to: U256, amount: U256, token_index: u32) -> anyhow::Result<()> {
    let client = get_client()?;
    let key = h256_to_keyset(private_key);

    let mut rng = rand::thread_rng();
    let salt = Salt::rand(&mut rng);
    let transfer = Transfer {
        recipient: GenericAddress::from_pubkey(to),
        amount,
        token_index,
        salt,
    };
    let memo = client.send_tx_request(key, vec![transfer]).await?;

    // sleep for a while to wait for the block builder to build the block
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    client.finalize_tx(key, &memo).await?;

    Ok(())
}

pub async fn sync(private_key: H256) -> anyhow::Result<()> {
    let client = get_client()?;
    let key = h256_to_keyset(private_key);
    client.sync(key).await?;
    Ok(())
}

pub async fn balance(private_key: H256) -> anyhow::Result<()> {
    let client = get_client()?;
    let key = h256_to_keyset(private_key);
    client.sync(key).await?;

    let user_data = client.get_user_data(key).await?;
    let balances = user_data.balances();
    for (i, leaf) in balances.iter() {
        println!("Token {}: {}", i, leaf.amount);
    }
    Ok(())
}

fn h256_to_keyset(h256: H256) -> KeySet {
    KeySet::new(BigUint::from_bytes_be(h256.as_bytes()).into())
}
