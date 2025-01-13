use async_trait::async_trait;
use intmax2_interfaces::{
    api::{
        error::ServerError,
        store_vault_server::{
            interface::{DataType, SaveDataEntry, StoreVaultClientInterface},
            types::{
                DataWithMetaData, GetDataAllAfterRequest, GetDataAllAfterResponse,
                GetSenderProofSetRequest, GetSenderProofSetResponse, GetUserDataRequest,
                GetUserDataResponse, SaveDataBatchRequest, SaveDataBatchResponse,
                SaveSenderProofSetRequest, SaveUserDataRequest,
            },
        },
    },
    utils::signature::Signable,
};
use intmax2_zkp::{common::signature::key_set::KeySet, ethereum_types::bytes32::Bytes32};
use plonky2::{field::goldilocks_field::GoldilocksField, plonk::config::PoseidonGoldilocksConfig};

use super::utils::query::post_request;

const TIME_TO_EXPIRY: u64 = 60; // 1 minute

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

#[derive(Debug, Clone)]
pub struct StoreVaultServerClient {
    base_url: String,
}

impl StoreVaultServerClient {
    pub fn new(base_url: &str) -> Self {
        StoreVaultServerClient {
            base_url: base_url.to_string(),
        }
    }
}

#[async_trait(?Send)]
impl StoreVaultClientInterface for StoreVaultServerClient {
    async fn save_user_data(
        &self,
        key: KeySet,
        prev_digest: Option<Bytes32>,
        encrypted_data: &[u8],
    ) -> Result<(), ServerError> {
        let request = SaveUserDataRequest {
            data: encrypted_data.to_vec(),
            prev_digest,
        };
        let request_with_auth = request.sign(key, TIME_TO_EXPIRY);
        post_request::<_, ()>(
            &self.base_url,
            "/store-vault-server/save-user-data",
            Some(&request_with_auth),
        )
        .await?;
        Ok(())
    }

    async fn get_user_data(&self, key: KeySet) -> Result<Option<Vec<u8>>, ServerError> {
        let request = GetUserDataRequest;
        let request_with_auth = request.sign(key, TIME_TO_EXPIRY);
        let response: GetUserDataResponse = post_request(
            &self.base_url,
            "/store-vault-server/get-user-data",
            Some(&request_with_auth),
        )
        .await?;
        Ok(response.data)
    }

    async fn save_sender_proof_set(
        &self,
        ephemeral_key: KeySet,
        encrypted_data: &[u8],
    ) -> Result<(), ServerError> {
        let request = SaveSenderProofSetRequest {
            data: encrypted_data.to_vec(),
        };
        let request_with_auth = request.sign(ephemeral_key, TIME_TO_EXPIRY);
        post_request::<_, ()>(
            &self.base_url,
            "/store-vault-server/save-sender-proof-set",
            Some(&request_with_auth),
        )
        .await?;
        Ok(())
    }

    async fn get_sender_proof_set(&self, ephemeral_key: KeySet) -> Result<Vec<u8>, ServerError> {
        let request = GetSenderProofSetRequest;
        let request_with_auth = request.sign(ephemeral_key, TIME_TO_EXPIRY);
        let response: GetSenderProofSetResponse = post_request(
            &self.base_url,
            "/store-vault-server/get-sender-proof-set",
            Some(&request_with_auth),
        )
        .await?;
        Ok(response.data)
    }

    async fn save_data_batch(
        &self,
        key: KeySet,
        entries: &[SaveDataEntry],
    ) -> Result<Vec<String>, ServerError> {
        let request = SaveDataBatchRequest {
            data: entries.to_vec(),
        };
        let request_with_auth = request.sign(key, TIME_TO_EXPIRY);
        let response: SaveDataBatchResponse = post_request(
            &self.base_url,
            "/store-vault-server/save-data-batch",
            Some(&request_with_auth),
        )
        .await?;
        Ok(response.uuids)
    }

    async fn get_data_all_after(
        &self,
        data_type: DataType,
        key: KeySet,
        timestamp: u64,
    ) -> Result<Vec<DataWithMetaData>, ServerError> {
        let request = GetDataAllAfterRequest {
            data_type,
            timestamp,
        };
        let request_with_auth = request.sign(key, TIME_TO_EXPIRY);
        let response: GetDataAllAfterResponse = post_request(
            &self.base_url,
            "/store-vault-server/get-data-all-after",
            Some(&request_with_auth),
        )
        .await?;
        Ok(response.data)
    }
}
