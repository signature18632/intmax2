use super::interface::SaveDataEntry;
use crate::data::meta_data::MetaData;
use intmax2_zkp::{common::signature::flatten::FlatG2, ethereum_types::u256::U256};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSaveDataRequest {
    pub data: Vec<SaveDataEntry>,
    pub pubkey: U256,
    pub expiry: u64,
    pub signature: FlatG2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSaveDataResponse {
    pub uuids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserDataRequestWithSignature {
    pub expiry: u64,
    pub signature: FlatG2,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserDataResponse {
    pub is_exist: bool,
    #[serde_as(as = "Base64")]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDataAllAfterQuery {
    pub pubkey: U256,
    pub timestamp: u64,
    pub expiry: u64,
    pub signature: FlatG2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDataAllAfterResponse {
    pub data: Vec<DataWithMetaData>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataWithMetaData {
    pub meta_data: MetaData,
    #[serde_as(as = "Base64")]
    pub data: Vec<u8>,
}
