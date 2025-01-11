use serde::Deserialize;

pub mod app;

#[derive(Deserialize)]
pub struct EnvVar {
    pub validity_prover_base_url: String,
    pub heartbeat_interval: u64,
    pub submit_interval: u64,
}
