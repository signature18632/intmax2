use ethers::types::H256;
use intmax2_client_sdk::external_api::store_vault_server::StoreVaultServerClient;
use intmax2_interfaces::{
    api::store_vault_server::interface::StoreVaultClientInterface, data::user_data::UserData,
};
use intmax2_zkp::ethereum_types::u256::U256;
use num_bigint::BigUint;
use serde::Deserialize;

#[derive(Deserialize)]
struct EnvVar {
    pub user_pubkey: H256,
    pub store_vault_server_base_url: String,
}

#[tokio::test]
async fn reset_user_data() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = envy::from_env::<EnvVar>().unwrap();

    let user_pubkey: U256 = BigUint::from_bytes_be(config.user_pubkey.as_bytes()).try_into()?;
    let store_vault_server = StoreVaultServerClient::new(&config.store_vault_server_base_url);
    let user_data = UserData::new(user_pubkey).encrypt(user_pubkey);
    store_vault_server
        .save_user_data(user_pubkey, user_data)
        .await?;
    Ok(())
}
