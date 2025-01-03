use ethers::types::H256;
use intmax2_client_sdk::external_api::store_vault_server::StoreVaultServerClient;
use intmax2_interfaces::{
    api::store_vault_server::interface::StoreVaultClientInterface, data::user_data::UserData,
};
use intmax2_zkp::{common::signature::key_set::KeySet, ethereum_types::u256::U256};
use num_bigint::BigUint;
use serde::Deserialize;

#[derive(Deserialize)]
struct EnvVar {
    pub user_pubkey: H256,
    pub store_vault_server_base_url: String,
}

#[tokio::test]
async fn test_store_vault_server() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = envy::from_env::<EnvVar>().unwrap();

    let pubkey: U256 = BigUint::from_bytes_be(config.user_pubkey.as_bytes()).try_into()?;
    let store_vault_server = StoreVaultServerClient::new(&config.store_vault_server_base_url);

    let mut user_data = store_vault_server
        .get_user_data(pubkey)
        .await?
        .map(|data| UserData::decrypt(&data, KeySet::dummy()).unwrap())
        .unwrap_or(UserData::new(pubkey));
    dbg!(&user_data.deposit_lpt);
    user_data.deposit_lpt = 0;
    store_vault_server
        .save_user_data(pubkey, user_data.encrypt(pubkey))
        .await?;
    Ok(())
}
