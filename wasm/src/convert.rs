use ethers::types::{Address, H256};
use intmax2_core_sdk::client::client::TxRequestMemo;
use intmax2_zkp::{
    ethereum_types::{bytes32::Bytes32, u256::U256, u32limb_trait::U32LimbTrait},
    mock::data::user_data::UserData,
};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

pub fn parse_h256(s: &str) -> Result<H256, JsError> {
    let x: H256 = s
        .parse()
        .map_err(|e| JsError::new(&format!("failed to parse h256 {}", e)))?;
    Ok(x)
}

pub fn parse_address(s: &str) -> Result<intmax2_zkp::ethereum_types::address::Address, JsError> {
    let x: Address = s
        .parse()
        .map_err(|e| JsError::new(&format!("failed to parse address {}", e)))?;
    let x = intmax2_zkp::ethereum_types::address::Address::from_bytes_be(&x.0);
    Ok(x)
}

pub fn parse_u256(s: &str) -> Result<U256, JsError> {
    let x: BigUint = s
        .parse()
        .map_err(|e| JsError::new(&format!("failed to parse biguint {}", e)))?;
    let x: U256 = x
        .try_into()
        .map_err(|e| JsError::new(&format!("failed to convert u256 {}", e)))?;
    Ok(x)
}

pub fn bytes32_to_string(x: Bytes32) -> String {
    x.to_hex()
}

pub fn tx_request_memo_to_value(memo: &TxRequestMemo) -> JsValue {
    serde_wasm_bindgen::to_value(memo).unwrap()
}

pub fn value_to_tx_request_memo(value: &JsValue) -> Result<TxRequestMemo, JsError> {
    serde_wasm_bindgen::from_value(value.clone())
        .map_err(|e| JsError::new(&format!("failed to parse tx request memo {}", e)))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct JsUserData {
    pub pubkey: String, // hex string of the user public key

    pub block_number: u32,

    pub balances: Vec<TokenBalance>, // token index and balance amount
    pub private_commitment: String,  // hex string of the private commitment

    // The latest unix timestamp of processed (incorporated into the balance proof or rejected)
    // actions
    pub deposit_lpt: u64,
    pub transfer_lpt: u64,
    pub tx_lpt: u64,
    pub withdrawal_lpt: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[wasm_bindgen(getter_with_clone)]
pub struct TokenBalance {
    token_index: u32,
    amount: String,
    is_insufficient: bool,
}

impl JsUserData {
    pub fn from_user_data(user_data: &UserData) -> Self {
        let balances = user_data
            .balances()
            .iter()
            .map(|(token_index, leaf)| {
                let amount = leaf.amount.to_string();
                let is_insufficient = leaf.is_insufficient;
                TokenBalance {
                    token_index: *token_index as u32,
                    amount,
                    is_insufficient,
                }
            })
            .collect();
        // split balances into token indices and balance values

        Self {
            pubkey: user_data.pubkey.to_hex(),
            block_number: user_data.block_number,
            balances,
            private_commitment: user_data
                .full_private_state
                .to_private_state()
                .commitment()
                .to_string(),
            deposit_lpt: user_data.deposit_lpt,
            transfer_lpt: user_data.transfer_lpt,
            tx_lpt: user_data.tx_lpt,
            withdrawal_lpt: user_data.withdrawal_lpt,
        }
    }
}


