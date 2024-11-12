use client::{get_client, Config};
use convert::{
    bytes32_to_string, parse_address, parse_h256, parse_u256, tx_request_memo_to_value,
    value_to_tx_request_memo, JsUserData,
};
use ethers::types::H256;
use intmax2_core_sdk::client::client::TxRequestMemo;
use intmax2_zkp::{
    common::{
        generic_address::GenericAddress, salt::Salt, signature::key_set::KeySet, transfer::Transfer,
    },
    ethereum_types::u256::U256,
};
use num_bigint::BigUint;
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

pub mod client;
pub mod convert;

/// Function to take a backup before calling the deposit function of the liquidity contract.
/// You can also get the pubkey_salt_hash from the return value.
#[wasm_bindgen]
pub async fn prepare_deposit(
    config: Config,
    private_key: &str,
    amount: &str,
    token_index: u32,
) -> Result<String, JsError> {
    let private_key = parse_h256(private_key)?;
    let amount = parse_u256(amount)?;

    let client = get_client(config);
    let key: KeySet = h256_to_keyset(private_key);
    let deposit_call = client.prepare_deposit(key, token_index, amount).await?;
    let pubkey_salt_hash = bytes32_to_string(deposit_call.pubkey_salt_hash);
    Ok(pubkey_salt_hash)
}

/// Function to send a tx request to the block builder. The return value contains information to take a backup.
#[wasm_bindgen]
pub async fn send_tx_request(
    config: Config,
    block_builder_url: &str,
    private_key: &str,
    is_withdrawal: bool,
    to: &str, // intmax2 public key (is_withdrawal=false) or ethereum address (is_withdrawal=true)
    amount: &str,
    token_index: u32,
) -> Result<JsValue, JsError> {
    let private_key = parse_h256(private_key)?;
    let amount = parse_u256(amount)?;
    let recipient = if is_withdrawal {
        let to = parse_address(to)?;
        GenericAddress::from_address(to)
    } else {
        let to = h256_to_u256(parse_h256(to)?);
        GenericAddress::from_pubkey(to)
    };

    let client = get_client(config);
    let key = h256_to_keyset(private_key);

    let mut rng = rand::thread_rng();
    let salt = Salt::rand(&mut rng);
    let transfer = Transfer {
        recipient,
        amount,
        token_index,
        salt,
    };
    let memo = client
        .send_tx_request(block_builder_url, key, vec![transfer])
        .await
        .map_err(|e| JsError::new(&format!("failed to send tx request {}", e)))?;

    Ok(tx_request_memo_to_value(&memo))
}

/// In this function, query block proposal from the block builder,
/// and then send the signed tx tree root to the block builder.
/// A backup of the tx is also taken.
/// You need to call send_tx_request before calling this function.
#[wasm_bindgen]
pub async fn finalize_tx(
    config: Config,
    block_builder_url: &str,
    private_key: &str,
    tx_request_memo: &JsValue,
) -> Result<(), JsError> {
    let private_key = parse_h256(private_key)?;
    let tx_request_memo: TxRequestMemo = value_to_tx_request_memo(tx_request_memo)?;

    let client = get_client(config);
    let key = h256_to_keyset(private_key);
    client
        .finalize_tx(block_builder_url, key, &tx_request_memo)
        .await?;
    Ok(())
}

/// Synchronize the user's balance proof. It may take a long time to generate ZKP.
#[wasm_bindgen]
pub async fn sync(config: Config, private_key: &str) -> Result<(), JsError> {
    let private_key = parse_h256(private_key)?;
    let client = get_client(config);
    let key = h256_to_keyset(private_key);
    client.sync(key).await?;
    Ok(())
}

/// Synchronize the user's withdrawal proof, and send request to the withdrawal aggregator.
/// It may take a long time to generate ZKP.
#[wasm_bindgen]
pub async fn sync_withdrawals(config: Config, private_key: &str) -> Result<(), JsError> {
    let private_key = parse_h256(private_key)?;
    let client = get_client(config);
    let key = h256_to_keyset(private_key);
    client.sync_withdrawals(key).await?;
    Ok(())
}

/// Get the user's data. It is recommended to sync before calling this function.
#[wasm_bindgen]
pub async fn get_user_data(config: Config, private_key: &str) -> Result<JsUserData, JsError> {
    let private_key = parse_h256(private_key)?;
    let client = get_client(config);
    let key = h256_to_keyset(private_key);
    let user_data = client.get_user_data(key).await?;
    Ok(JsUserData::from_user_data(&user_data))
}

/// Function to mimic the deposit call of the contract. For development purposes only.
#[wasm_bindgen]
pub async fn mimic_deposit(
    _token_index: u32,
    _pubkey_salt_hash: &str,
    _amount: &str,
) -> Result<(), JsError> {
    todo!()
}

fn h256_to_u256(h256: H256) -> U256 {
    BigUint::from_bytes_be(h256.as_bytes()).try_into().unwrap()
}

fn h256_to_keyset(h256: H256) -> KeySet {
    KeySet::new(BigUint::from_bytes_be(h256.as_bytes()).into())
}
