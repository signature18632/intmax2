use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    pub timestamp: u64,
    pub uuid: String,
}

impl MetaData {
    pub fn set_block_number(self, block_number: u32) -> MetaDataWithBlockNumber {
        MetaDataWithBlockNumber {
            meta: self,
            block_number,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaDataWithBlockNumber {
    pub meta: MetaData,
    pub block_number: u32,
}
