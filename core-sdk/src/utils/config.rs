use ethers::types::Address;
use serde::{Deserialize, Serialize};

const CONFIG_BYTE: &'static [u8] = include_bytes!("../../config.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub intmax2_server_base_url: String,

    // Blockchain
    pub chain_id: u64,
    pub liquidity_contract_address: Address,
}

impl Config {
    pub fn load() -> Self {
        serde_json::from_slice(CONFIG_BYTE).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let _config = Config::load();
    }
}
