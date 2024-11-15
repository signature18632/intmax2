use intmax2_zkp::mock::data::transfer_data::TransferData;
use intmax2_zkp::mock::data::tx_data::TxData;
use intmax2_zkp::mock::data::user_data::UserData;
use intmax2_zkp::{
    ethereum_types::u32limb_trait::U32LimbTrait as _, mock::data::deposit_data::DepositData,
};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use wasm_bindgen::prelude::wasm_bindgen;

use super::common::{JsTransfer, JsTx};

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct JsUserData {
    pub pubkey: String, // hex string of the user public key

    pub block_number: u32,

    pub balances: Vec<TokenBalance>, // token index and balance amount
    pub private_commitment: String,  // hex string of the private commitment

    // The latest unix timestamp of processed (incorporated into the balance proof or rejected)
    // actions
    pub deposit_lpt: u64,
    pub transfer_lpt: u64,
    pub tx_lpt: u64,
    pub withdrawal_lpt: u64,

    // Uuids of actions that has already been incorporated into the balance proof
    pub processed_deposit_uuids: Vec<String>,
    pub processed_transfer_uuids: Vec<String>,
    pub processed_tx_uuids: Vec<String>,
    pub processed_withdrawal_uuids: Vec<String>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct TokenBalance {
    token_index: u32,
    amount: String,        // 10 base string
    is_insufficient: bool, // insufficient balance flag
}

impl JsUserData {
    pub fn from_user_data(user_data: &UserData) -> Self {
        let balances = user_data
            .balances()
            .iter()
            .map(|(token_index, leaf)| {
                let amount = leaf.amount.to_string();
                let is_insufficient = leaf.is_insufficient;
                TokenBalance {
                    token_index: *token_index as u32,
                    amount,
                    is_insufficient,
                }
            })
            .collect();

        Self {
            pubkey: user_data.pubkey.to_hex(),
            block_number: user_data.block_number,
            balances,
            private_commitment: user_data
                .full_private_state
                .to_private_state()
                .commitment()
                .to_string(),
            deposit_lpt: user_data.deposit_lpt,
            transfer_lpt: user_data.transfer_lpt,
            tx_lpt: user_data.tx_lpt,
            withdrawal_lpt: user_data.withdrawal_lpt,
            processed_deposit_uuids: user_data.processed_deposit_uuids.clone(),
            processed_transfer_uuids: user_data.processed_transfer_uuids.clone(),
            processed_tx_uuids: user_data.processed_tx_uuids.clone(),
            processed_withdrawal_uuids: user_data.processed_withdrawal_uuids.clone(),
        }
    }
}
