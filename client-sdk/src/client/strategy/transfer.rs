use super::error::StrategyError;
use intmax2_interfaces::{
    api::{
        store_vault_server::{
            interface::{DataType, StoreVaultClientInterface},
            types::DataWithMetaData,
        },
        validity_prover::interface::ValidityProverClientInterface,
    },
    data::{meta_data::MetaData, sender_proof_set::SenderProofSet, transfer_data::TransferData},
};
use intmax2_zkp::common::signature::key_set::KeySet;
use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub struct TransferInfo {
    pub settled: Vec<(MetaData, TransferData)>,
    pub pending: Vec<(MetaData, TransferData)>,
    pub timeout: Vec<(MetaData, TransferData)>,
}

pub async fn fetch_transfer_info<S: StoreVaultClientInterface, V: ValidityProverClientInterface>(
    store_vault_server: &S,
    validity_prover: &V,
    key: KeySet,
    transfer_lpt: u64,
    processed_transfer_uuids: &[String],
    tx_timeout: u64,
) -> Result<TransferInfo, StrategyError> {
    let mut settled = Vec::new();
    let mut pending = Vec::new();
    let mut timeout = Vec::new();

    let encrypted_data = store_vault_server
        .get_data_all_after(DataType::Transfer, key, transfer_lpt)
        .await?;
    for DataWithMetaData { meta, data } in encrypted_data {
        if processed_transfer_uuids.contains(&meta.uuid) {
            log::info!("Transfer {} is already processed", meta.uuid);
            continue;
        }
        match TransferData::decrypt(&data, key) {
            Ok(transfer_data) => {
                let ephemeral_key =
                    KeySet::new(BigUint::from(transfer_data.sender_proof_set_ephemeral_key).into());
                let encrypted_sender_proof_set = store_vault_server
                    .get_sender_proof_set(ephemeral_key)
                    .await?;
                let sender_proof_set =
                    match SenderProofSet::decrypt(&encrypted_sender_proof_set, ephemeral_key) {
                        Ok(data) => data,
                        Err(e) => {
                            log::error!("failed to decrypt sender proof set: {}", e);
                            continue;
                        }
                    };
                let mut transfer_data = transfer_data;
                transfer_data.set_sender_proof_set(sender_proof_set);

                let tx_tree_root = transfer_data.tx_tree_root;
                let block_number = validity_prover
                    .get_block_number_by_tx_tree_root(tx_tree_root)
                    .await?;
                if let Some(block_number) = block_number {
                    // set block number
                    let mut meta = meta;
                    meta.block_number = Some(block_number);
                    settled.push((meta, transfer_data));
                } else if meta.timestamp + tx_timeout < chrono::Utc::now().timestamp() as u64 {
                    // timeout
                    log::error!("Transfer {} is timeout", meta.uuid);
                    timeout.push((meta, transfer_data));
                } else {
                    // pending
                    log::info!("Transfer {} is pending", meta.uuid);
                    pending.push((meta, transfer_data));
                }
            }
            Err(e) => {
                log::error!("failed to decrypt transfer data: {}", e);
                // ignore this transfer
            }
        };
    }

    // sort by block number
    settled.sort_by_key(|(meta, _)| meta.block_number.unwrap());

    Ok(TransferInfo {
        settled,
        pending,
        timeout,
    })
}
