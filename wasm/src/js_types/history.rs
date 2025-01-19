use intmax2_client_sdk::client::history::{EntryStatus, HistoryEntry};
use wasm_bindgen::prelude::wasm_bindgen;

use super::{
    common::JsMetaData,
    data::{JsDepositData, JsTransferData, JsTxData},
};

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
#[wasm_bindgen(getter_with_clone)]
pub struct JsDepositEntry {
    pub deposit: JsDepositData,
    pub status: JsEntryStatusWithBlockNumber,
    pub meta: JsMetaData,
}

#[derive(Clone, Debug)]
#[wasm_bindgen(getter_with_clone)]
pub struct JsReceiveEntry {
    pub transfer: JsTransferData,
    pub status: JsEntryStatusWithBlockNumber,
    pub meta: JsMetaData,
}

#[derive(Clone, Debug)]
#[wasm_bindgen(getter_with_clone)]
pub struct JsSendEntry {
    pub tx: JsTxData,
    pub status: JsEntryStatusWithBlockNumber,
    pub meta: JsMetaData,
}

#[derive(Clone, Debug)]
#[wasm_bindgen(getter_with_clone)]
pub struct JsHistoryEntry {
    pub deposit: Option<JsDepositEntry>,
    pub receive: Option<JsReceiveEntry>,
    pub send: Option<JsSendEntry>,
}

impl From<HistoryEntry> for JsHistoryEntry {
    fn from(entry: HistoryEntry) -> Self {
        match entry {
            HistoryEntry::Deposit {
                deposit,
                status,
                meta,
            } => Self {
                deposit: Some(JsDepositEntry {
                    deposit: deposit.into(),
                    status: status.into(),
                    meta: meta.into(),
                }),
                receive: None,
                send: None,
            },
            HistoryEntry::Receive {
                transfer,
                status,
                meta,
            } => Self {
                deposit: None,
                receive: Some(JsReceiveEntry {
                    transfer: transfer.into(),
                    status: status.into(),
                    meta: meta.into(),
                }),
                send: None,
            },
            HistoryEntry::Send { tx, status, meta } => Self {
                deposit: None,
                receive: None,
                send: Some(JsSendEntry {
                    tx: tx.into(),
                    status: status.into(),
                    meta: meta.into(),
                }),
            },
        }
    }
}
