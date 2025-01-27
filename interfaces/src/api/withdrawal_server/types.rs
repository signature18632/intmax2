use intmax2_zkp::ethereum_types::address::Address;
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};
use serde::{Deserialize, Serialize};

use crate::utils::signature::Signable;

use super::interface::{ClaimInfo, Fee, WithdrawalInfo};

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeResponse {
    pub fees: Vec<Fee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestWithdrawalRequest {
    pub single_withdrawal_proof: ProofWithPublicInputs<F, C, D>,
}

impl Signable for RequestWithdrawalRequest {
    fn content(&self) -> Vec<u8> {
        bincode::serialize(&self.single_withdrawal_proof).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestClaimRequest {
    pub single_claim_proof: ProofWithPublicInputs<F, C, D>,
}

impl Signable for RequestClaimRequest {
    fn content(&self) -> Vec<u8> {
        bincode::serialize(&self.single_claim_proof).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalInfoRequest;

impl Signable for GetWithdrawalInfoRequest {
    fn content(&self) -> Vec<u8> {
        vec![]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalInfoResponse {
    pub withdrawal_info: Vec<WithdrawalInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClaimInfoRequest;

impl Signable for GetClaimInfoRequest {
    fn content(&self) -> Vec<u8> {
        vec![]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClaimInfoResponse {
    pub claim_info: Vec<ClaimInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalInfoByRecipientQuery {
    pub recipient: Address,
}
