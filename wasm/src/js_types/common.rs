use intmax2_zkp::{
    common::{generic_address::GenericAddress, transfer::Transfer, tx::Tx},
    ethereum_types::{address::Address, u256::U256, u32limb_trait::U32LimbTrait},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use super::utils::{parse_salt, parse_u256};

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct JsGenericAddress {
    pub is_pubkey: bool,
    pub data: String, // hex string of 32 bytes (pubkey) or 20 bytes (ethereum address)
}

impl JsGenericAddress {
    pub fn from_generic_address(generic_address: &GenericAddress) -> Self {
        let is_pubkey = generic_address.is_pubkey;
        let data = if is_pubkey {
            generic_address.to_pubkey().unwrap().to_hex()
        } else {
            generic_address.to_address().unwrap().to_string()
        };
        Self { is_pubkey, data }
    }

    pub fn to_generic_address(&self) -> Result<GenericAddress, JsError> {
        if self.is_pubkey {
            let pubkey =
                U256::from_hex(&self.data).map_err(|_| JsError::new("Failed to parse pubkey"))?;
            Ok(GenericAddress::from_pubkey(pubkey))
        } else {
            let address = Address::from_hex(&self.data)
                .map_err(|_| JsError::new("Failed to parse address"))?;
            Ok(GenericAddress::from_address(address))
        }
    }
}

#[wasm_bindgen]
impl JsGenericAddress {
    #[wasm_bindgen(constructor)]
    pub fn new(is_pubkey: bool, data: String) -> Self {
        Self { is_pubkey, data }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct JsTransfer {
    pub recipient: JsGenericAddress,
    pub token_index: u32,
    pub amount: String, // 10 base string
    pub salt: String,   // hex string
}

#[wasm_bindgen]
impl JsTransfer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        recipient: JsGenericAddress,
        token_index: u32,
        amount: String,
        salt: String,
    ) -> Self {
        Self {
            recipient,
            token_index,
            amount,
            salt,
        }
    }
}

impl JsTransfer {
    pub fn from_transfer(transfer: &Transfer) -> Self {
        Self {
            recipient: JsGenericAddress::from_generic_address(&transfer.recipient),
            token_index: transfer.token_index,
            amount: transfer.amount.to_string(),
            salt: transfer.salt.to_string(),
        }
    }

    pub fn to_transfer(&self) -> Result<Transfer, JsError> {
        let recipient = self.recipient.to_generic_address()?;
        let amount =
            parse_u256(&self.amount).map_err(|_| JsError::new("Failed to parse amount"))?;
        let salt = parse_salt(&self.salt)?;
        Ok(Transfer {
            recipient,
            token_index: self.token_index,
            amount,
            salt,
        })
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
