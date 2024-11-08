use plonky2::{
    field::{extension::Extendable, goldilocks_field::GoldilocksField},
    hash::hash_types::RichField,
    plonk::config::{GenericConfig, PoseidonGoldilocksConfig},
};

use intmax2_zkp::{
    common::signature::key_set::KeySet,
    mock::data::{meta_data::MetaData, transfer_data::TransferData},
};

use crate::{
    client::error::ClientError,
    external_api::{
        block_validity_prover::interface::BlockValidityInterface,
        store_vault_server::interface::StoreVaultInterface,
    },
};

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

#[derive(Debug, Clone)]
pub struct WithdrawalInfo<F, C, const D: usize>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
{
    pub settled: Vec<(MetaData, TransferData<F, C, D>)>,
    pub pending: Vec<MetaData>,
    pub rejected: Vec<MetaData>,
}

pub async fn fetch_withdrawal_info<S: StoreVaultInterface, V: BlockValidityInterface>(
    store_vault_server: &S,
    validity_prover: &V,
    key: KeySet,
    withdrwal_lpt: u64,
    tx_timeout: u64,
) -> Result<WithdrawalInfo<F, C, D>, ClientError> {
    let mut settled = Vec::new();
    let mut pending = Vec::new();
    let mut rejected = Vec::new();

    let encrypted_data = store_vault_server
        .get_withdrawal_data_all_after(key.pubkey, withdrwal_lpt)
        .await?;
    for (meta, encrypted_data) in encrypted_data {
        match TransferData::decrypt(&encrypted_data, key) {
            Ok(transfer_data) => {
                let tx_tree_root = transfer_data.tx_data.tx_tree_root;
                let block_number = validity_prover
                    .get_block_number_by_tx_tree_root(tx_tree_root)
                    .await?;
                if let Some(block_number) = block_number {
                    // set block number
                    let mut meta = meta;
                    meta.block_number = Some(block_number);
                    settled.push((meta, transfer_data));
                } else {
                    if meta.timestamp + tx_timeout < chrono::Utc::now().timestamp() as u64 {
                        // timeout
                        log::error!("Withdrawal {} is timeouted", meta.uuid);
                        rejected.push(meta);
                    } else {
                        // pending
                        log::info!("Withdrawal {} is pending", meta.uuid);
                        pending.push(meta);
                    }
                }
            }
            Err(e) => {
                log::error!("failed to decrypt withdrawal data: {}", e);
                rejected.push(meta);
            }
        }
    }

    Ok(WithdrawalInfo {
        settled,
        pending,
        rejected,
    })
}
