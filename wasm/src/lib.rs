use client::{get_client, Config};
use ethers::types::H256;
use intmax2_core_sdk::client::client::{DepositCall, TxRequestMemo};
use intmax2_zkp::{
    common::{
        generic_address::GenericAddress, salt::Salt, signature::key_set::KeySet, transfer::Transfer,
    },
    ethereum_types::u256::U256,
    mock::data::user_data::UserData,
};
use num_bigint::BigUint;

pub mod client;

// Function to take a backup before calling the deposit function of the liquidity contract.
// You can also get the pubkey_salt_hash from the return value.
pub async fn prepare_deposit(
    config: Config,
    private_key: H256,
    amount: U256,
    token_index: u32,
) -> anyhow::Result<DepositCall> {
    let client = get_client(config)?;
    let key = h256_to_keyset(private_key);
    let deposit_call = client.prepare_deposit(key, token_index, amount).await?;
    Ok(deposit_call)
}

// Function to send a tx request to the block builder. The return value contains information to take a backup.
pub async fn send_tx_request(
    config: Config,
    block_builder_url: &str,
    private_key: H256,
    to: U256,
    amount: U256,
    token_index: u32,
) -> anyhow::Result<TxRequestMemo> {
    let client = get_client(config)?;
    let key = h256_to_keyset(private_key);

    let mut rng = rand::thread_rng();
    let salt = Salt::rand(&mut rng);
    let transfer = Transfer {
        recipient: GenericAddress::from_pubkey(to),
        amount,
        token_index,
        salt,
    };
    let memo = client
        .send_tx_request(block_builder_url, key, vec![transfer])
        .await?;
    // client.finalize_tx(block_builder_url, key, &memo).await?;
    Ok(memo)
}

// In this function, query block proposal from the block builder,
// and then send the signed tx tree root to the block builder.
// A backup of the tx is also taken.
// You need to call send_tx_request before calling this function.
pub async fn finalize_tx(
    config: Config,
    block_builder_url: &str,
    private_key: H256,
    tx_request_memo: TxRequestMemo,
) -> anyhow::Result<()> {
    let client = get_client(config)?;
    let key = h256_to_keyset(private_key);
    client
        .finalize_tx(block_builder_url, key, &tx_request_memo)
        .await?;
    Ok(())
}

// Synchronize the user's balance proof. It may take a long time to generate ZKP.
pub async fn sync(config: Config, private_key: H256) -> anyhow::Result<()> {
    let client = get_client(config)?;
    let key = h256_to_keyset(private_key);
    client.sync(key).await?;
    Ok(())
}

// Synchronize the user's withdrawal proof, and send request to the withdrawal aggregator.
// It may take a long time to generate ZKP.
pub async fn sync_withdrawals(config: Config, private_key: H256) -> anyhow::Result<()> {
    let client = get_client(config)?;
    let key = h256_to_keyset(private_key);
    client.sync_withdrawals(key).await?;
    Ok(())
}

// Get the user's data. It is recommended to sync before calling this function.
pub async fn get_user_data(config: Config, private_key: H256) -> anyhow::Result<UserData> {
    let client = get_client(config)?;
    let key = h256_to_keyset(private_key);

    let user_data = client.get_user_data(key).await?;
    Ok(user_data)
}

fn h256_to_keyset(h256: H256) -> KeySet {
    KeySet::new(BigUint::from_bytes_be(h256.as_bytes()).into())
}
