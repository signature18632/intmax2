use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockBuilderInfo {
    pub fee: f64,
    pub speed: u32,
    pub url: String,
}
