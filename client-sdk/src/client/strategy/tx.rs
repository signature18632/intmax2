use super::{common::fetch_decrypt_validate, error::StrategyError};
use intmax2_interfaces::{
    api::{
        store_vault_server::interface::{DataType, StoreVaultClientInterface},
        validity_prover::interface::ValidityProverClientInterface,
    },
    data::{
        meta_data::{MetaData, MetaDataWithBlockNumber},
        tx_data::TxData,
        user_data::ProcessStatus,
    },
};
use intmax2_zkp::common::signature::key_set::KeySet;

#[derive(Debug, Clone)]
pub struct TxInfo {
    pub settled: Vec<(MetaDataWithBlockNumber, TxData)>,
    pub pending: Vec<(MetaData, TxData)>,
    pub timeout: Vec<(MetaData, TxData)>,
}

pub async fn fetch_tx_info<S: StoreVaultClientInterface, V: ValidityProverClientInterface>(
    store_vault_server: &S,
    validity_prover: &V,
    key: KeySet,
    tx_status: &ProcessStatus,
    tx_timeout: u64,
) -> Result<TxInfo, StrategyError> {
    let mut settled = Vec::new();
    let mut pending = Vec::new();
    let mut timeout = Vec::new();

    let data_with_meta =
        fetch_decrypt_validate::<_, TxData>(store_vault_server, key, DataType::Tx, tx_status)
            .await?;
    for (meta, tx_data) in data_with_meta {
        let tx_tree_root = tx_data.tx_tree_root;
        let block_number = validity_prover
            .get_block_number_by_tx_tree_root(tx_tree_root)
            .await?;
        if let Some(block_number) = block_number {
            let meta = MetaDataWithBlockNumber { meta, block_number };
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
    // sort by block number
    settled.sort_by_key(|(meta, _)| meta.block_number);

    // sort by timestamp
    pending.sort_by_key(|(meta, _)| meta.timestamp);

    Ok(TxInfo {
        settled,
        pending,
        timeout,
    })
}
