use intmax2_zkp::common::{transfer::Transfer, tx::Tx};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct JsTransfer {
    pub is_withdrawal: bool,
    pub recipient: String, // hex string
    pub token_index: u32,
    pub amount: String, // 10 base string
    pub salt: String,   // hex string
}

impl JsTransfer {
    pub fn from_transfer(transfer: &Transfer) -> Self {
        let is_withdrawal = !transfer.recipient.is_pubkey;
        let recipient = if is_withdrawal {
            transfer.recipient.to_address().unwrap().to_string()
        } else {
            transfer.recipient.to_pubkey().unwrap().to_string()
        };

        Self {
            is_withdrawal,
            recipient,
            token_index: transfer.token_index,
            amount: transfer.amount.to_string(),
            salt: transfer.salt.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct JsTx {
    pub transfer_tree_root: String, // hex string
    pub nonce: u32,
}

impl JsTx {
    pub fn from_tx(tx: &Tx) -> Self {
        Self {
            transfer_tree_root: tx.transfer_tree_root.to_string(),
            nonce: tx.nonce,
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct JsBlockProposal {
    
}
