use super::interface::{DataType, SaveDataEntry};
use crate::{data::meta_data::MetaData, utils::signature::Auth};
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSaveDataRequest {
    pub data: Vec<SaveDataEntry>,
    pub auth: Auth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSaveDataResponse {
    pub uuids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserDataRequest {
    pub auth: Auth,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserDataResponse {
    #[serde_as(as = "Option<Base64>")]
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDataAllAfterQuery {
    pub data_type: DataType,
    pub auth: Auth,
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
