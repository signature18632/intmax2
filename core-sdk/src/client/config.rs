#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub deposit_timeout: u64,
    pub tx_timeout: u64,
    pub max_tx_query_times: usize,
    pub tx_query_interval: u64,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            deposit_timeout: 0,
            tx_timeout: 0,
            max_tx_query_times: 1,
            tx_query_interval: 0,
        }
    }
}
