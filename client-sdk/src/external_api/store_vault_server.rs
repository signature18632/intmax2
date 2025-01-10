use crate::utils::signature::{bytes_to_hex, sign_message, verify_signature};
use async_trait::async_trait;
use intmax2_interfaces::{
    api::{
        error::ServerError,
        store_vault_server::{
            interface::{DataType, StoreVaultClientInterface},
            types::{
                AuthInfoForGetData, AuthInfoForSaveData, BatchSaveDataRequest,
                BatchSaveDataResponse, GetBalanceProofQuery, GetBalanceProofResponse,
                GetDataAllAfterRequestWithSignature, GetDataAllAfterResponse,
                GetUserDataRequestWithSignature, GetUserDataResponse, SaveBalanceProofRequest,
                SaveDataRequestWithSignature, SaveDataResponse,
            },
        },
    },
    data::meta_data::MetaData,
};
use intmax2_zkp::{
    common::signature::key_set::KeySet, ethereum_types::u256::U256,
    utils::poseidon_hash_out::PoseidonHashOut,
};
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};

use super::utils::query::{get_request, post_request};

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
    async fn save_balance_proof(
        &self,
        pubkey: U256,
        proof: &ProofWithPublicInputs<F, C, D>,
    ) -> Result<(), ServerError> {
        let request = SaveBalanceProofRequest {
            pubkey,
            balance_proof: proof.clone(),
        };
        post_request::<_, ()>(
            &self.base_url,
            "/store-vault-server/save-balance-proof",
            Some(&request),
        )
        .await
    }

    async fn get_balance_proof(
        &self,
        pubkey: U256,
        block_number: u32,
        private_commitment: PoseidonHashOut,
    ) -> Result<Option<ProofWithPublicInputs<F, C, D>>, ServerError> {
        let query = GetBalanceProofQuery {
            pubkey,
            block_number,
            private_commitment,
        };
        let response: GetBalanceProofResponse = get_request(
            &self.base_url,
            "/store-vault-server/get-balance-proof",
            Some(query),
        )
        .await?;
        Ok(response.balance_proof)
    }

    /// The signer is required for the API below:
    /// - /tx/save
    async fn save_data(
        &self,
        data_type: DataType,
        pubkey: U256,
        encrypted_data: &[u8],
        signer: Option<KeySet>,
    ) -> Result<String, ServerError> {
        let auth = if let Some(key) = signer {
            let auth = self.generate_auth_info_for_posting(key, encrypted_data.to_vec())?;

            Some(auth)
        } else {
            None
        };

        let request = SaveDataRequestWithSignature {
            pubkey,
            data: encrypted_data.to_vec(),
            auth,
        };
        let response: SaveDataResponse = post_request(
            &self.base_url,
            &format!("/store-vault-server/{}/save", data_type),
            Some(&request),
        )
        .await?;
        Ok(response.uuid)
    }

    /// The signer is required for the API below:
    /// - /tx/save
    async fn save_data_batch(
        &self,
        data_type: DataType,
        data: Vec<(U256, Vec<u8>)>,
    ) -> Result<Vec<String>, ServerError> {
        if data_type == DataType::Tx {
            panic!("Batch save is not supported for tx data");
        }
        let request = BatchSaveDataRequest { requests: data };
        let response: BatchSaveDataResponse = post_request(
            &self.base_url,
            &format!("/store-vault-server/{}/batch-save", data_type),
            Some(&request),
        )
        .await?;
        Ok(response.uuids)
    }

    async fn get_data(
        &self,
        _data_type: DataType,
        _uuid: &str,
        _signer: KeySet,
    ) -> Result<Option<(MetaData, Vec<u8>)>, ServerError> {
        unimplemented!()
    }

    async fn get_data_batch(
        &self,
        _data_type: DataType,
        _uuids: &[String],
    ) -> Result<Vec<Option<(MetaData, Vec<u8>)>>, ServerError> {
        unimplemented!()
    }

    async fn get_data_all_after(
        &self,
        data_type: DataType,
        signer: KeySet,
        timestamp: u64,
    ) -> Result<Vec<(MetaData, Vec<u8>)>, ServerError> {
        let auth = self.generate_auth_info_for_fetching(signer).await?;
        let request = GetDataAllAfterRequestWithSignature { timestamp, auth };
        let response: GetDataAllAfterResponse = post_request(
            &self.base_url,
            &format!("/store-vault-server/{}/get-all-after", data_type),
            Some(&request),
        )
        .await?;
        Ok(response.data)
    }

    async fn save_user_data(
        &self,
        signer: KeySet,
        encrypted_data: Vec<u8>,
    ) -> Result<(), ServerError> {
        let auth = self.generate_auth_info_for_posting(signer, encrypted_data.clone())?;
        let request = SaveDataRequestWithSignature {
            pubkey: signer.pubkey,
            data: encrypted_data,
            auth: Some(auth),
        };
        post_request::<_, ()>(
            &self.base_url,
            "/store-vault-server/save-user-data",
            Some(&request),
        )
        .await
    }

    async fn get_user_data(&self, signer: KeySet) -> Result<Option<Vec<u8>>, ServerError> {
        let auth = self.generate_auth_info_for_fetching(signer).await?;
        let request = GetUserDataRequestWithSignature { auth };
        let response: GetUserDataResponse = post_request(
            &self.base_url,
            "/store-vault-server/get-user-data",
            Some(&request),
        )
        .await?;
        Ok(response.data)
    }
}

impl StoreVaultServerClient {
    async fn generate_auth_info_for_fetching(
        &self,
        key: KeySet,
    ) -> Result<AuthInfoForGetData, ServerError> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let challenge = [timestamp.to_be_bytes().to_vec(), vec![0; 24]].concat();
        let challenge_hex = bytes_to_hex(&challenge);
        let signature = sign_message(key.privkey, challenge.clone()).unwrap();
        Ok(AuthInfoForGetData {
            signature,
            pubkey: key.pubkey,
            challenge: challenge_hex,
        })
    }

    fn generate_auth_info_for_posting(
        &self,
        key: KeySet,
        data: Vec<u8>,
    ) -> Result<AuthInfoForSaveData, ServerError> {
        let signature = sign_message(key.privkey, data.clone()).unwrap();
        debug_assert!(verify_signature(signature.clone(), key.pubkey, data).is_ok());
        Ok(AuthInfoForSaveData { signature })
    }
}
