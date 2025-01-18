use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    pub timestamp: u64,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaDataWithBlockNumber {
    pub meta: MetaData,
    pub block_number: u32,
}
