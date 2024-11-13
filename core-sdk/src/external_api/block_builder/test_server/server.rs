use async_trait::async_trait;
use intmax2_zkp::{
    common::{block_builder::BlockProposal, signature::flatten::FlatG2, tx::Tx},
    ethereum_types::u256::U256,
};
use reqwest_wasm::Client;

use crate::external_api::block_builder::{
    interface::{BlockBuilderInterface, FeeProof},
    test_server::types::{
        PostSignatureRequest, QueryProposalRequest, QueryProposalResponse, TxRequestRequest,
    },
};
use crate::external_api::common::error::ServerError;

#[derive(Debug, Clone)]
pub struct TestBlockBuilder {
    base_url: String,
    client: Client,
}

impl TestBlockBuilder {
    pub fn new(base_url: String) -> Self {
        TestBlockBuilder {
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

    async fn get_request<U: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<U, ServerError> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .client
            .get(&url)
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
impl BlockBuilderInterface for TestBlockBuilder {
    async fn send_tx_request(
        &self,
        _block_builder_url: &str,
        pubkey: U256,
        tx: Tx,
        _fee_proof: Option<FeeProof>,
    ) -> Result<(), ServerError> {
        let request = TxRequestRequest { pubkey, tx };
        self.post_request::<_, ()>("/block-builder/tx-request", &request)
            .await
    }

    async fn query_proposal(
        &self,
        _block_builder_url: &str,
        pubkey: U256,
        tx: Tx,
    ) -> Result<Option<BlockProposal>, ServerError> {
        let request = QueryProposalRequest { pubkey, tx };
        let response: QueryProposalResponse = self
            .post_request("/block-builder/query-proposal", &request)
            .await?;
        Ok(response.block_proposal)
    }

    async fn post_signature(
        &self,
        _block_builder_url: &str,
        pubkey: U256,
        tx: Tx,
        signature: FlatG2,
    ) -> Result<(), ServerError> {
        let request = PostSignatureRequest {
            pubkey,
            tx,
            signature,
        };
        self.post_request::<_, ()>("/block-builder/post-signature", &request)
            .await
    }
}

impl TestBlockBuilder {
    pub async fn construct_block(&self) -> Result<(), ServerError> {
        self.get_request::<()>("/block-builder/construct-block")
            .await
    }

    pub async fn post_block(&self) -> Result<(), ServerError> {
        self.get_request::<()>("/block-builder/post-block").await
    }

    pub async fn post_empty_block(&self) -> Result<(), ServerError> {
        self.get_request::<()>("/block-builder/post-empty-block")
            .await
    }
}
