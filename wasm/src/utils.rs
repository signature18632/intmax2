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
