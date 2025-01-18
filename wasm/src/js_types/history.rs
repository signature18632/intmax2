use intmax2_client_sdk::client::history::{EntryStatus, HistoryEntry};
use intmax2_interfaces::data::meta_data::MetaData;
use wasm_bindgen::prelude::wasm_bindgen;

use super::data::{JsDepositData, JsTransferData, JsTxData};

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum JsEntryStatus {
    Settled,   // Settled at block number but not processed yet
    Processed, // Incorporated into the balance proof
    Pending,   // Not settled yet
    Timeout,   // Timed out
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct JsEntryStatusWithBlockNumber {
    pub status: JsEntryStatus,
    pub block_number: Option<u32>,
}

impl From<EntryStatus> for JsEntryStatusWithBlockNumber {
    fn from(status: EntryStatus) -> Self {
        match status {
            EntryStatus::Settled(b) => Self {
                status: JsEntryStatus::Settled,
                block_number: Some(b),
            },
            EntryStatus::Processed(b) => Self {
                status: JsEntryStatus::Processed,
                block_number: Some(b),
            },
            EntryStatus::Pending => Self {
                status: JsEntryStatus::Pending,
                block_number: None,
            },
            EntryStatus::Timeout => Self {
                status: JsEntryStatus::Timeout,
                block_number: None,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum JsHistoryEntry {
    Deposit {
        deposit: JsDepositData,
        status: EntryStatus,
        meta: MetaData,
    },
    Receive {
        transfer: JsTransferData,
        status: EntryStatus,
        meta: MetaData,
    },
    Send {
        tx: JsTxData,
        status: EntryStatus,
        meta: MetaData,
    },
}

impl From<HistoryEntry> for JsHistoryEntry {
    fn from(entry: HistoryEntry) -> Self {
        match entry {
            HistoryEntry::Deposit {
                deposit,
                status,
                meta,
            } => Self::Deposit {
                deposit: deposit.into(),
                status,
                meta,
            },
            HistoryEntry::Receive {
                transfer,
                status,
                meta,
            } => Self::Receive {
                transfer: transfer.into(),
                status,
                meta,
            },
            HistoryEntry::Send { tx, status, meta } => Self::Send {
                tx: tx.into(),
                status,
                meta,
            },
        }
    }
}
