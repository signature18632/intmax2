// use ethers::core::{k256::elliptic_curve::sec1::ToEncodedPoint, utils::Anvil};
// use intmax2_client_sdk::external_api::store_vault_server::StoreVaultServerClient;
// use intmax2_interfaces::{
//     api::store_vault_server::interface::StoreVaultClientInterface, data::user_data::UserData,
// };
// use intmax2_zkp::{common::signature::key_set::KeySet, ethereum_types::u256::U256};
// use num_bigint::BigUint;
// use serde::Deserialize;

// #[derive(Deserialize)]
// struct EnvVar {
//     pub store_vault_server_base_url: String,
// }

// #[tokio::test]
// async fn test_store_vault_server() -> anyhow::Result<()> {
//     dotenv::dotenv().ok();
//     let config = envy::from_env::<EnvVar>().unwrap();
//     let anvil = Anvil::new().spawn();
//     let private_key = &anvil.keys()[1];
//     let public_key = private_key.public_key();
//     let b = public_key.to_encoded_point(true);
//     let compressed_public_key = b.as_bytes();
//     let key_without_prefix = &compressed_public_key[1..];

//     let user_pubkey: U256 = BigUint::from_bytes_be(key_without_prefix).try_into()?;
//     let store_vault_server = StoreVaultServerClient::new(&config.store_vault_server_base_url);

//     let mut user_data = store_vault_server
//         .get_user_data(pubkey)
//         .await?
//         .map(|data| UserData::decrypt(&data, KeySet::dummy()).unwrap())
//         .unwrap_or(UserData::new(pubkey));
//     dbg!(&user_data.deposit_lpt);
//     user_data.deposit_lpt = 0;
//     store_vault_server
//         .save_user_data(pubkey, user_data.encrypt(pubkey))
//         .await?;
//     Ok(())
// }
