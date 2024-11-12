use ethers::types::{Address, H256};
use intmax2_core_sdk::{
    client::{client::Client, config::ClientConfig},
    external_api::{
        balance_prover::local::LocalBalanceProver, block_builder::server::server::BlockBuilder,
        block_validity_prover::server::block_validity_prover::BlockValidityProver,
        contract::liquidity_contract::LiquidityContract,
        store_vault_server::server::store_vault_server::StoreVaultServer,
        withdrawal_aggregator::server::WithdrawalAggregatorServer,
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
type W = WithdrawalAggregatorServer;

pub fn get_client() -> anyhow::Result<Client<BC, BB, S, V, B, W>> {
    let contract = BC::new("".to_string(), 1, Address::zero());
    let block_builder = BB::new("http://localhost:4000/v1".to_string());
    let store_vault_server = S::new("http://localhost:4000/v1/".to_string())?;
    let validity_prover = V::new("http://localhost:4000/v1/blockvalidity".to_string())?;
    let balance_prover = B::new()?;
    let withdrawal_aggregator = W::new();

    let config = ClientConfig {
        deposit_timeout: 3600,
        tx_timeout: 60,
        max_tx_query_times: 50,
        tx_query_interval: 1,
    };

    let client: Client<LiquidityContract, BlockBuilder, StoreVaultServer, BlockValidityProver, LocalBalanceProver, WithdrawalAggregatorServer> = Client {
        contract,
        block_builder,
        store_vault_server,
        validity_prover,
        balance_prover,
        withdrawal_aggregator,
        config,
    };

    Ok(client)
}

pub async fn deposit(
    eth_private_key: H256,
    private_key: H256,
    amount: U256,
    token_index: u32,
) -> anyhow::Result<()> {
    let client = get_client()?;
    let key = h256_to_keyset(private_key);
    client
        .deposit(eth_private_key, key, token_index, amount)
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
