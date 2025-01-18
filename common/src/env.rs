use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EnvType {
    Local,
    Dev,
    Staging,
    Prod,
}
