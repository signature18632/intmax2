use intmax2_interfaces::{
    api::{
        store_vault_server::{
            interface::{DataType, StoreVaultClientInterface},
            types::DataWithMetaData,
        },
        validity_prover::interface::ValidityProverClientInterface,
    },
    data::{meta_data::MetaData, tx_data::TxData},
};
use intmax2_zkp::common::signature::key_set::KeySet;
use plonky2::{field::goldilocks_field::GoldilocksField, plonk::config::PoseidonGoldilocksConfig};

use super::error::StrategyError;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

#[derive(Debug, Clone)]
pub struct TxInfo {
    pub settled: Vec<(MetaData, TxData)>,
    pub pending: Vec<(MetaData, TxData)>,
    pub timeout: Vec<(MetaData, TxData)>,
}

pub async fn fetch_tx_info<S: StoreVaultClientInterface, V: ValidityProverClientInterface>(
    store_vault_server: &S,
    validity_prover: &V,
    key: KeySet,
    tx_lpt: u64,
    processed_tx_uuids: &[String],
    tx_timeout: u64,
) -> Result<TxInfo, StrategyError> {
    let mut settled = Vec::new();
    let mut pending = Vec::new();
    let mut timeout = Vec::new();

    let encrypted_data = store_vault_server
        .get_data_all_after(DataType::Tx, key, tx_lpt)
        .await?;
    for DataWithMetaData { meta, data } in encrypted_data {
        if processed_tx_uuids.contains(&meta.uuid) {
            log::info!("Tx {} is already processed", meta.uuid);
            continue;
        }
        match TxData::decrypt(&data, key) {
            Ok(tx_data) => {
                let tx_tree_root = tx_data.tx_tree_root;
                let block_number = validity_prover
                    .get_block_number_by_tx_tree_root(tx_tree_root)
                    .await?;
                if let Some(block_number) = block_number {
                    // set block number
                    let mut meta = meta;
                    meta.block_number = Some(block_number);
                    settled.push((meta, tx_data));
                } else if meta.timestamp + tx_timeout < chrono::Utc::now().timestamp() as u64 {
                    // timeout
                    log::error!("Tx {} is timeout", meta.uuid);
                    timeout.push((meta, tx_data));
                } else {
                    // pending
                    log::info!("Tx {} is pending", meta.uuid);
                    pending.push((meta, tx_data));
                }
            }
            Err(e) => {
                // just ignore the invalid data
                log::error!("failed to decrypt tx data: {}", e);
            }
        };
    }

    // sort by block number
    settled.sort_by_key(|(meta, _)| meta.block_number.unwrap());

    // sort by timestamp
    pending.sort_by_key(|(meta, _)| meta.timestamp);

    Ok(TxInfo {
        settled,
        pending,
        timeout,
    })
}
