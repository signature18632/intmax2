use async_trait::async_trait;
use ethers::types::H256;
use intmax2_zkp::ethereum_types::{bytes32::Bytes32, u256::U256};
use reqwest_wasm::Client;

use crate::external_api::common::error::ServerError;
use crate::external_api::contract::interface::{BlockchainError, ContractInterface};

use super::types::DepositNativeTokenRequest;

#[derive(Debug, Clone)]
pub struct TestContract {
    base_url: String,
    client: Client,
}

impl TestContract {
    pub fn new(base_url: String) -> Self {
        TestContract {
            base_url,
            client: Client::new(),
        }
    }

    async fn post_request<T: serde::Serialize, U: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<U, ServerError> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .client
            .post(&url)
            .json(body)
            .send()
            .await
            .map_err(|e| ServerError::NetworkError(e.to_string()))?;

        if response.status().is_success() {
            response
                .json::<U>()
                .await
                .map_err(|e| ServerError::DeserializationError(e.to_string()))
        } else {
            Err(ServerError::ServerError(response.status().to_string()))
        }
    }
}

#[async_trait(?Send)]
impl ContractInterface for TestContract {
    async fn deposit_native_token(
        &self,
        _signer_private_key: H256,
        pubkey_salt_hash: Bytes32,
        amount: U256,
    ) -> Result<(), BlockchainError> {
        let request = DepositNativeTokenRequest {
            pubkey_salt_hash,
            amount,
        };

        // Note: In a real implementation, you would use the signer_private_key to sign the transaction.
        // For this test implementation, we're ignoring it as the server is handling the signing.

        self.post_request::<_, ()>("/contract/deposit-native-token", &request)
            .await
            .map_err(|e| match e {
                ServerError::ServerError(msg) if msg.contains("Insufficient funds") => {
                    BlockchainError::InsufficientFunds(msg)
                }
                ServerError::ServerError(msg) => BlockchainError::TransactionFailed(msg),
                _ => BlockchainError::InternalError(e.to_string()),
            })
    }
}
