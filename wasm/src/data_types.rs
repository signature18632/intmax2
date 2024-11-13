use intmax2_zkp::common::transfer::Transfer;
use intmax2_zkp::common::tx::Tx;
use intmax2_zkp::mock::data::transfer_data::TransferData;
use intmax2_zkp::mock::data::tx_data::TxData;
use intmax2_zkp::{
    ethereum_types::u32limb_trait::U32LimbTrait as _, mock::data::deposit_data::DepositData,
};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use wasm_bindgen::prelude::wasm_bindgen;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

#[wasm_bindgen(getter_with_clone)]
pub struct JsDepositData {
    pub deposit_salt: String,     // hex string
    pub pubkey_salt_hash: String, // hex string
    pub token_index: u32,
    pub amount: String, // 10 base string
}

impl JsDepositData {
    pub fn from_deposit_data(deposit_data: &DepositData) -> Self {
        Self {
            deposit_salt: deposit_data.deposit_salt.to_string(),
            pubkey_salt_hash: deposit_data.deposit.pubkey_salt_hash.to_hex(),
            token_index: deposit_data.deposit.token_index,
            amount: deposit_data.deposit.amount.to_string(),
        }
    }
}

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
pub struct JsTransferData {
    pub sender: String, // hex string
    pub transfer: JsTransfer,
}

impl JsTransferData {
    pub fn from_transfer_data(transfer_data: &TransferData<F, C, D>) -> Self {
        Self {
            sender: transfer_data.sender.to_hex(),
            transfer: JsTransfer::from_transfer(&transfer_data.transfer),
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

#[wasm_bindgen(getter_with_clone)]
pub struct JsTxData {
    pub tx: JsTx,
    pub transfers: Vec<JsTransfer>,
}

impl JsTxData {
    pub fn from_tx_data(tx_data: &TxData<F, C, D>) -> Self {
        let tx = JsTx::from_tx(&tx_data.common.tx);
        let transfers = tx_data
            .spent_witness
            .transfers
            .iter()
            .map(JsTransfer::from_transfer)
            .collect::<Vec<_>>();
        Self { tx, transfers }
    }
}

#[cfg(test)]
mod tests {
    use intmax2_zkp::common::salt::Salt;

    #[test]
    fn generate_salt() {
        let rng = &mut rand::thread_rng();
        let salt = Salt::rand(rng);
        println!("salt: {}", salt.to_string());
    }
}
