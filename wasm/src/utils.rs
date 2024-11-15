use ethers::types::H256;
use intmax2_core_sdk::client::client::TxRequestMemo;
use intmax2_zkp::{
    common::{block_builder::BlockProposal, signature::key_set::KeySet},
    ethereum_types::{bytes32::Bytes32, u256::U256, u32limb_trait::U32LimbTrait},
};
use num_bigint::BigUint;
use wasm_bindgen::{JsError, JsValue};

pub fn h256_to_bytes32(h256: H256) -> Bytes32 {
    Bytes32::from_bytes_be(h256.as_bytes())
}

pub fn h256_to_keyset(h256: H256) -> KeySet {
    KeySet::new(BigUint::from_bytes_be(h256.as_bytes()).into())
}

pub fn parse_h256(s: &str) -> Result<H256, JsError> {
    let x: H256 = s
        .parse()
        .map_err(|e| JsError::new(&format!("failed to parse h256 {}", e)))?;
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

pub fn tx_request_memo_to_value(memo: &TxRequestMemo) -> JsValue {
    serde_wasm_bindgen::to_value(memo).unwrap()
}

pub fn value_to_tx_request_memo(value: &JsValue) -> Result<TxRequestMemo, JsError> {
    serde_wasm_bindgen::from_value(value.clone())
        .map_err(|e| JsError::new(&format!("failed to parse tx request memo {}", e)))
}

pub fn block_proposal_to_value(proposal: &BlockProposal) -> JsValue {
    serde_wasm_bindgen::to_value(proposal).unwrap()
}

pub fn value_to_block_proposal(value: &JsValue) -> Result<BlockProposal, JsError> {
    serde_wasm_bindgen::from_value(value.clone())
        .map_err(|e| JsError::new(&format!("failed to parse block proposal {}", e)))
}
