use serde::Deserialize;

use common::env::EnvType;

#[derive(Deserialize)]
pub struct Env {
    #[serde(default = "default_env")]
    pub env: EnvType,

    #[serde(default = "default_app_log_level")]
    pub app_log: String,
}

fn default_env() -> EnvType {
    EnvType::Dev
}

fn default_app_log_level() -> String {
    "info".to_string()
}
