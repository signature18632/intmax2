use ethers::core::{k256::elliptic_curve::sec1::ToEncodedPoint, utils::Anvil};
use intmax2_client_sdk::external_api::store_vault_server::StoreVaultServerClient;
use intmax2_interfaces::{
    api::store_vault_server::interface::StoreVaultClientInterface, data::user_data::UserData,
};
use intmax2_zkp::ethereum_types::u256::U256;
use num_bigint::BigUint;
use serde::Deserialize;

#[derive(Deserialize)]
struct EnvVar {
    pub store_vault_server_base_url: String,
}

#[tokio::test]
async fn reset_user_data() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = envy::from_env::<EnvVar>().unwrap();
    let anvil = Anvil::new().spawn();
    let private_key = &anvil.keys()[1];
    let public_key = private_key.public_key();
    let b = public_key.to_encoded_point(true);
    let compressed_public_key = b.as_bytes();
    let key_without_prefix = &compressed_public_key[1..];

    let user_pubkey: U256 = BigUint::from_bytes_be(key_without_prefix).try_into()?;
    let store_vault_server = StoreVaultServerClient::new(&config.store_vault_server_base_url);
    let user_data = UserData::new(user_pubkey).encrypt(user_pubkey);
    store_vault_server
        .save_user_data(user_pubkey, user_data)
        .await?;
    Ok(())
}
