use intmax2_interfaces::{
    api::{
        balance_prover::interface::BalanceProverClientInterface,
        block_builder::interface::BlockBuilderClientInterface,
        store_vault_server::interface::StoreVaultClientInterface,
        validity_prover::interface::ValidityProverClientInterface,
        withdrawal_server::interface::WithdrawalServerClientInterface,
    },
    data::{
        deposit_data::DepositData,
        meta_data::{MetaData, MetaDataWithBlockNumber},
        transfer_data::TransferData,
        tx_data::TxData,
        user_data::ProcessStatus,
    },
};
use intmax2_zkp::common::signature::key_set::KeySet;
use serde::{Deserialize, Serialize};

use super::{
    client::Client,
    error::ClientError,
    strategy::{deposit::fetch_deposit_info, transfer::fetch_transfer_info, tx::fetch_tx_info},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EntryStatus {
    Settled(u32),   // Settled at block number but not processed yet
    Processed(u32), // Incorporated into the balance proof
    Pending,        // Not settled yet
    Timeout,        // Timed out
}

impl EntryStatus {
    pub fn from_settled(processed_uuids: &[String], meta: MetaDataWithBlockNumber) -> Self {
        if processed_uuids.contains(&meta.meta.uuid) {
            EntryStatus::Processed(meta.block_number)
        } else {
            EntryStatus::Settled(meta.block_number)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HistoryEntry {
    Deposit {
        deposit: DepositData,
        status: EntryStatus,
        meta: MetaData,
    },
    Receive {
        transfer: TransferData,
        status: EntryStatus,
        meta: MetaData,
    },
    Send {
        tx: TxData,
        status: EntryStatus,
        meta: MetaData,
    },
}

// /// Transfer without salt
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub enum GenericTransfer {
//     Transfer {
//         recipient: U256,
//         token_index: u32,
//         amount: U256,
//     },
//     Withdrawal {
//         recipient: Address,
//         token_index: u32,
//         amount: U256,
//     },
// }

// impl std::fmt::Display for GenericTransfer {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             GenericTransfer::Transfer {
//                 recipient,
//                 token_index,
//                 amount,
//             } => write!(
//                 f,
//                 "Transfer(recipient: {}, token_index: {}, amount: {})",
//                 recipient.to_hex(),
//                 token_index,
//                 amount
//             ),
//             GenericTransfer::Withdrawal {
//                 recipient,
//                 token_index,
//                 amount,
//             } => write!(
//                 f,
//                 "Withdrawal(recipient: {}, token_index: {}, amount: {})",
//                 recipient.to_hex(),
//                 token_index,
//                 amount
//             ),
//         }
//     }
// }

pub async fn fetch_history<
    BB: BlockBuilderClientInterface,
    S: StoreVaultClientInterface,
    V: ValidityProverClientInterface,
    B: BalanceProverClientInterface,
    W: WithdrawalServerClientInterface,
>(
    client: &Client<BB, S, V, B, W>,
    key: KeySet,
) -> Result<Vec<HistoryEntry>, ClientError> {
    let (user_data, _) = client.get_user_data_and_digest(key).await?;

    let mut history = Vec::new();

    // Deposits
    let all_deposit_info = fetch_deposit_info(
        &client.store_vault_server,
        &client.validity_prover,
        &client.liquidity_contract,
        key,
        &ProcessStatus::default(),
        client.config.deposit_timeout,
    )
    .await?;
    for (meta, settled) in all_deposit_info.settled {
        history.push(HistoryEntry::Deposit {
            deposit: settled,
            status: EntryStatus::from_settled(
                &user_data.deposit_status.processed_uuids,
                meta.clone(),
            ),
            meta: meta.meta,
        });
    }
    for (meta, pending) in all_deposit_info.pending {
        history.push(HistoryEntry::Deposit {
            deposit: pending,
            status: EntryStatus::Pending,
            meta,
        });
    }
    for (meta, timeout) in all_deposit_info.timeout {
        history.push(HistoryEntry::Deposit {
            deposit: timeout,
            status: EntryStatus::Timeout,
            meta,
        });
    }

    let all_transfers_info = fetch_transfer_info(
        &client.store_vault_server,
        &client.validity_prover,
        key,
        &ProcessStatus::default(),
        client.config.tx_timeout,
    )
    .await?;
    for (meta, settled) in all_transfers_info.settled {
        history.push(HistoryEntry::Receive {
            transfer: settled,
            status: EntryStatus::from_settled(
                &user_data.transfer_status.processed_uuids,
                meta.clone(),
            ),
            meta: meta.meta,
        });
    }
    for (meta, pending) in all_transfers_info.pending {
        history.push(HistoryEntry::Receive {
            transfer: pending,
            status: EntryStatus::Pending,
            meta: meta.clone(),
        });
    }
    for (meta, timeout) in all_transfers_info.timeout {
        history.push(HistoryEntry::Receive {
            transfer: timeout,
            status: EntryStatus::Timeout,
            meta: meta.clone(),
        });
    }

    let all_tx_info = fetch_tx_info(
        &client.store_vault_server,
        &client.validity_prover,
        key,
        &ProcessStatus::default(),
        client.config.tx_timeout,
    )
    .await?;
    for (meta, settled) in all_tx_info.settled {
        history.push(HistoryEntry::Send {
            tx: settled,
            status: EntryStatus::from_settled(&user_data.tx_status.processed_uuids, meta.clone()),
            meta: meta.meta.clone(),
        });
    }
    for (meta, pending) in all_tx_info.pending {
        history.push(HistoryEntry::Send {
            tx: pending,
            status: EntryStatus::Pending,
            meta,
        });
    }
    for (meta, timeout) in all_tx_info.timeout {
        history.push(HistoryEntry::Send {
            tx: timeout,
            status: EntryStatus::Timeout,
            meta,
        });
    }

    // sort history
    history.sort_by_key(|entry| match entry {
        HistoryEntry::Deposit { meta, .. } => meta.timestamp,
        HistoryEntry::Receive { meta, .. } => meta.timestamp,
        HistoryEntry::Send { meta, .. } => meta.timestamp,
    });

    Ok(history)
}

// fn extract_generic_transfers(tx_data: TxData) -> Vec<GenericTransfer> {
//     let mut transfers = Vec::new();
//     for transfer in tx_data.spent_witness.transfers.iter() {
//         let recipient = transfer.recipient;
//         if !recipient.is_pubkey
//             && recipient.data == U256::default()
//             && transfer.amount == U256::default()
//         {
//             // dummy transfer
//             continue;
//         }
//         if recipient.is_pubkey {
//             transfers.push(GenericTransfer::Transfer {
//                 recipient: recipient.to_pubkey().unwrap(),
//                 token_index: transfer.token_index,
//                 amount: transfer.amount,
//             });
//         } else {
//             transfers.push(GenericTransfer::Withdrawal {
//                 recipient: recipient.to_address().unwrap(),
//                 token_index: transfer.token_index,
//                 amount: transfer.amount,
//             });
//         }
//     }
//     transfers
// }
