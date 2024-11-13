// impl ContractInterface for LocalContract {
//     async fn deposit_native_token(
//         &self,
//         _signer_private_key: H256,
//         pubkey_salt_hash: Bytes32,
//         amount: U256,
//     ) -> Result<(), BlockchainError> {
//         self.0.lock().unwrap().deposit(pubkey_salt_hash, 0, amount);
//         Ok(())
//     }
// }

use intmax2_zkp::ethereum_types::{bytes32::Bytes32, u256::U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositNativeTokenRequest {
    pub pubkey_salt_hash: Bytes32,
    pub amount: U256,
}
