// get balance proof

use intmax2_zkp::ethereum_types::bytes32::Bytes32;
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};
use serde::{Deserialize, Serialize};

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

use crate::external_api::common::{error::ServerError, response::ServerErrorResponse};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum GetBalanceProofResponse {
    Success(GetBalanceProofSuccessResponse),
    Error(ServerErrorResponse),
}

#[derive(Serialize, Deserialize)]
pub struct GetBalanceProofSuccessResponse {
    pub success: bool,
    pub data: GetBalanceProofData,
}

#[derive(Serialize, Deserialize)]
pub struct GetBalanceProofData {
    pub proof: String,
}

pub async fn get_balance_proof(
    base_url: &str,
    pubkey: Bytes32,
    block_number: u32,
    private_commitment: Bytes32,
) -> Result<Option<ProofWithPublicInputs<F, C, D>>, ServerError> {
    let url = format!(
        "{}/balance_proof?user={}&blockNumber={}&privateCommitment={}",
        base_url, pubkey, block_number, private_commitment
    );
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await.map_err(|e| {
        ServerError::NetworkError(format!("Failed to get balance proof from server: {}", e))
    })?;
    if response.status().as_u16() == 404 {
        return Ok(None);
    }
    if response.status().as_u16() == 500 {
        return Err(ServerError::ServerError(format!(
            "Failed to get balance proof from server: {}",
            response.text().await.unwrap()
        )));
    }

    todo!()
}
