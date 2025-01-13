use super::interface::{DataType, SaveDataEntry};
use crate::{
    data::meta_data::MetaData,
    utils::signature::{Auth, Signable},
};
use intmax2_zkp::ethereum_types::bytes32::Bytes32;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveUserDataRequest {
    #[serde_as(as = "Base64")]
    pub data: Vec<u8>,
    pub prev_digest: Option<Bytes32>,
    pub auth: Auth,
}

impl Signable for SaveUserDataRequest {
    fn content(&self) -> Vec<u8> {
        bincode::serialize(&(self.data.clone(), self.prev_digest)).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserDataRequest {
    pub auth: Auth,
}

impl Signable for GetUserDataRequest {
    fn content(&self) -> Vec<u8> {
        vec![]
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserDataResponse {
    #[serde_as(as = "Option<Base64>")]
    pub data: Option<Vec<u8>>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveSenderProofSetRequest {
    #[serde_as(as = "Base64")]
    pub data: Vec<u8>,
    pub auth: Auth,
}

impl Signable for SaveSenderProofSetRequest {
    fn content(&self) -> Vec<u8> {
        bincode::serialize(&self.data).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSenderProofSetRequest {
    pub auth: Auth,
}

impl Signable for GetSenderProofSetRequest {
    fn content(&self) -> Vec<u8> {
        vec![]
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSenderProofSetResponse {
    #[serde_as(as = "Base64")]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSaveDataRequest {
    pub data: Vec<SaveDataEntry>,
    pub auth: Auth,
}

impl Signable for BatchSaveDataRequest {
    fn content(&self) -> Vec<u8> {
        bincode::serialize(&self.data).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSaveDataResponse {
    pub uuids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDataAllAfterRequest {
    pub data_type: DataType,
    pub timestamp: u64,
    pub auth: Auth,
}

impl Signable for GetDataAllAfterRequest {
    fn content(&self) -> Vec<u8> {
        bincode::serialize(&(self.data_type, self.timestamp)).unwrap()
    }
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
