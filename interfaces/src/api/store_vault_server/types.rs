use intmax2_zkp::{
    common::signature::flatten::FlatG2, ethereum_types::u256::U256,
    utils::poseidon_hash_out::PoseidonHashOut,
};
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};
use serde::{Deserialize, Serialize};

use crate::data::meta_data::MetaData;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveBalanceProofRequest {
    pub pubkey: U256,
    pub balance_proof: ProofWithPublicInputs<F, C, D>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceProofQuery {
    pub pubkey: U256,
    pub block_number: u32,
    pub private_commitment: PoseidonHashOut,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceProofResponse {
    pub balance_proof: Option<ProofWithPublicInputs<F, C, D>>,
}

// #[serde(deny_unknown_fields)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDataRequest {
    pub pubkey: U256,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDataRequestWithSignature {
    pub pubkey: U256,
    pub data: Vec<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<FlatG2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDataResponse {
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSaveDataRequest {
    pub requests: Vec<(U256, Vec<u8>)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSaveDataResponse {
    pub uuids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserDataQuery {
    pub pubkey: U256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserDataResponse {
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDataResponse {
    pub data: Option<(MetaData, Vec<u8>)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchGetDataResponse {
    pub data: Vec<Option<(MetaData, Vec<u8>)>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDataQuery {
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchGetDataQuery {
    pub uuids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDataAllAfterQuery {
    pub pubkey: U256,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDataAllAfterResponse {
    pub data: Vec<(MetaData, Vec<u8>)>,
}

// #[test]
// fn test_json_deserialize() {
//     let json_str = r#"
//         {
//             "pubkey": "0",
//             "data": [],
//             "email": "john@example.com"
//         }
//     "#;

//     let user: SaveDataRequest = serde_json::from_str(json_str).unwrap();

//     println!("pubkey: {}", user.pubkey);
// }
