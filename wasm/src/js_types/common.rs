use intmax2_interfaces::api::withdrawal_server::interface::ContractWithdrawal;
use intmax2_zkp::{
    common::{
        generic_address::GenericAddress, transfer::Transfer, tx::Tx,
        withdrawal::get_withdrawal_nullifier,
    },
    ethereum_types::{address::Address, u256::U256, u32limb_trait::U32LimbTrait},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use super::utils::{parse_address, parse_bytes32, parse_salt, parse_u256};

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct JsGenericAddress {
    /// true if pubkey, false if ethereum address
    pub is_pubkey: bool,
    /// hex string of 32 bytes (pubkey) or 20 bytes (ethereum address)
    pub data: String,
}

impl From<GenericAddress> for JsGenericAddress {
    fn from(generic_address: GenericAddress) -> Self {
        let is_pubkey = generic_address.is_pubkey;
        let data = if is_pubkey {
            generic_address.to_pubkey().unwrap().to_hex()
        } else {
            generic_address.to_address().unwrap().to_string()
        };
        Self { is_pubkey, data }
    }
}

impl TryFrom<JsGenericAddress> for GenericAddress {
    type Error = JsError;

    fn try_from(js_generic_address: JsGenericAddress) -> Result<Self, Self::Error> {
        if js_generic_address.is_pubkey {
            let pubkey = U256::from_hex(&js_generic_address.data)
                .map_err(|_| JsError::new("Failed to parse pubkey"))?;
            Ok(GenericAddress::from_pubkey(pubkey))
        } else {
            let address = Address::from_hex(&js_generic_address.data)
                .map_err(|_| JsError::new("Failed to parse address"))?;
            Ok(GenericAddress::from_address(address))
        }
    }
}

#[wasm_bindgen]
impl JsGenericAddress {
    #[wasm_bindgen(constructor)]
    pub fn new(is_pubkey: bool, data: String) -> Result<Self, JsError> {
        // validation
        if is_pubkey {
            U256::from_hex(&data).map_err(|_| JsError::new("Invalid pubkey"))?;
        } else {
            Address::from_hex(&data).map_err(|_| JsError::new("Invalid address"))?;
        }
        Ok(Self { is_pubkey, data })
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

    pub fn to_withdrawal(&self) -> Result<JsContractWithdrawal, JsError> {
        let transfer: Transfer = self.clone().try_into()?;
        if transfer.recipient.is_pubkey {
            return Err(JsError::new("Recipient must be an ethereum address"));
        }
        let recipient = transfer.recipient.to_address().unwrap();
        let nullifier = get_withdrawal_nullifier(&transfer);
        let withdrawal = ContractWithdrawal {
            recipient,
            token_index: transfer.token_index,
            amount: transfer.amount,
            nullifier,
        };
        Ok(withdrawal.into())
    }
}

impl From<Transfer> for JsTransfer {
    fn from(transfer: Transfer) -> JsTransfer {
        Self {
            recipient: transfer.recipient.into(),
            token_index: transfer.token_index,
            amount: transfer.amount.to_string(),
            salt: transfer.salt.to_string(),
        }
    }
}

impl TryFrom<JsTransfer> for Transfer {
    type Error = JsError;

    fn try_from(js_transfer: JsTransfer) -> Result<Transfer, Self::Error> {
        let recipient = js_transfer.recipient.try_into()?;
        let amount =
            parse_u256(&js_transfer.amount).map_err(|_| JsError::new("Failed to parse amount"))?;
        let salt = parse_salt(&js_transfer.salt)?;
        Ok(Transfer {
            recipient,
            token_index: js_transfer.token_index,
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

impl From<Tx> for JsTx {
    fn from(tx: Tx) -> Self {
        Self {
            transfer_tree_root: tx.transfer_tree_root.to_string(),
            nonce: tx.nonce,
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct JsContractWithdrawal {
    pub recipient: String,
    pub token_index: u32,
    pub amount: String,
    pub nullifier: String,
}

impl From<ContractWithdrawal> for JsContractWithdrawal {
    fn from(contract_withdrawal: ContractWithdrawal) -> Self {
        Self {
            recipient: contract_withdrawal.recipient.to_hex(),
            token_index: contract_withdrawal.token_index,
            amount: contract_withdrawal.amount.to_string(),
            nullifier: contract_withdrawal.nullifier.to_hex(),
        }
    }
}

impl TryFrom<JsContractWithdrawal> for ContractWithdrawal {
    type Error = JsError;

    fn try_from(
        js_contract_withdrawal: JsContractWithdrawal,
    ) -> Result<ContractWithdrawal, Self::Error> {
        let recipient = parse_address(&js_contract_withdrawal.recipient)?;
        let amount = parse_u256(&js_contract_withdrawal.amount)?;
        let nullifier = parse_bytes32(&js_contract_withdrawal.nullifier)?;
        Ok(ContractWithdrawal {
            recipient,
            token_index: js_contract_withdrawal.token_index,
            amount,
            nullifier,
        })
    }
}

#[wasm_bindgen]
impl JsContractWithdrawal {
    #[wasm_bindgen(constructor)]
    pub fn new(recipient: String, token_index: u32, amount: String, nullifier: String) -> Self {
        Self {
            recipient,
            token_index,
            amount,
            nullifier,
        }
    }

    pub fn hash(&self) -> Result<String, JsError> {
        let contract_withdrawal: ContractWithdrawal = self.clone().try_into()?;
        let hash = contract_withdrawal.withdrawal_hash().to_hex();
        Ok(hash)
    }
}
