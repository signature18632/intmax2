use intmax2_interfaces::{
    api::store_vault_server::{
        interface::{DataType, StoreVaultClientInterface},
        types::DataWithMetaData,
    },
    data::{
        encryption::Encryption, meta_data::MetaData, user_data::ProcessStatus,
        validation::Validation,
    },
};
use intmax2_zkp::common::signature::key_set::KeySet;
use itertools::Itertools;

use super::error::StrategyError;

pub async fn fetch_decrypt_validate<S: StoreVaultClientInterface, T: Encryption + Validation>(
    store_vault_server: &S,
    key: KeySet,
    data_type: DataType,
    process_status: &ProcessStatus,
) -> Result<Vec<(MetaData, T)>, StrategyError> {
    // fetch pending data
    let encrypted_pending_data_with_meta = store_vault_server
        .get_data_batch(key, data_type, &process_status.pending_uuids)
        .await?;

    // fetch unprocessed data
    let encrypted_unprocessed_data_with_meta = store_vault_server
        .get_data_sequence(key, data_type, &process_status.last_processed_meta_data)
        .await?;

    // decrypt
    let data_with_meta = encrypted_pending_data_with_meta
        .into_iter()
        .chain(encrypted_unprocessed_data_with_meta.into_iter())
        .unique_by(|data_with_meta| data_with_meta.meta.uuid.clone()) // remove duplicates
        .filter_map(|data_with_meta| {
            let DataWithMetaData { meta, data } = data_with_meta;
            if process_status.processed_uuids.contains(&meta.uuid) {
                log::warn!("{} {} is already processed", data_type, meta.uuid);
                return None;
            }
            match T::decrypt(&data, key) {
                Ok(data) => match data.validate(key) {
                    Ok(_) => Some((meta, data)),
                    Err(e) => {
                        log::warn!("failed to validate {}: {}", data_type, e);
                        None
                    }
                },
                Err(e) => {
                    log::warn!("failed to decrypt {}: {}", data_type, e);
                    None
                }
            }
        })
        .collect::<Vec<_>>();
    Ok(data_with_meta)
}
