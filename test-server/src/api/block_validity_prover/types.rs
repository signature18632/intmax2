// #[derive(Clone)]
// pub struct LocalBlockValidityProver {
//     pub contract: Arc<Mutex<MockContract>>,
//     pub inner_block_validity_prover: Arc<Mutex<InnerBlockValidityProver<F, C, D>>>,
// }

// impl LocalBlockValidityProver {
//     // contract is shared state.
//     pub fn new(contract: Arc<Mutex<MockContract>>) -> Self {
//         let block_validity_prover = InnerBlockValidityProver::new();
//         Self {
//             contract,
//             inner_block_validity_prover: Arc::new(Mutex::new(block_validity_prover)),
//         }
//     }

//     pub fn sync(&self) -> anyhow::Result<()> {
//         let contract = self.contract.lock().unwrap();
//         self.inner_block_validity_prover
//             .lock()
//             .unwrap()
//             .sync(&contract)?;
//         Ok(())
//     }
// }

// #[async_trait(?Send)]
// impl BlockValidityInterface for LocalBlockValidityProver {
//     async fn block_number(&self) -> Result<u32, ServerError> {
//         let b = self
//             .inner_block_validity_prover
//             .lock()
//             .unwrap()
//             .block_number();
//         Ok(b)
//     }

//     async fn get_account_id(&self, pubkey: U256) -> Result<Option<usize>, ServerError> {
//         let account_id = self
//             .inner_block_validity_prover
//             .lock()
//             .unwrap()
//             .get_account_id(pubkey);
//         Ok(account_id)
//     }

//     async fn get_update_witness(
//         &self,
//         pubkey: U256,
//         root_block_number: u32,
//         leaf_block_number: u32,
//         is_prev_account_tree: bool,
//     ) -> Result<UpdateWitness<F, C, D>, ServerError> {
//         let update_witness = self
//             .inner_block_validity_prover
//             .lock()
//             .unwrap()
//             .get_update_witness(
//                 pubkey,
//                 root_block_number,
//                 leaf_block_number,
//                 is_prev_account_tree,
//             )
//             .map_err(|e| ServerError::InternalError(e.to_string()))?;
//         Ok(update_witness)
//     }

//     async fn get_deposit_index_and_block_number(
//         &self,
//         deposit_hash: Bytes32,
//     ) -> Result<Option<(usize, u32)>, ServerError> {
//         let deposit_index_and_block_number = self
//             .inner_block_validity_prover
//             .lock()
//             .unwrap()
//             .get_deposit_index_and_block_number(deposit_hash);
//         Ok(deposit_index_and_block_number)
//     }

//     async fn get_block_number_by_tx_tree_root(
//         &self,
//         tx_tree_root: Bytes32,
//     ) -> Result<Option<u32>, ServerError> {
//         let block_number = self
//             .inner_block_validity_prover
//             .lock()
//             .unwrap()
//             .get_block_number_by_tx_tree_root(tx_tree_root);
//         Ok(block_number)
//     }

//     async fn get_validity_pis(
//         &self,
//         block_number: u32,
//     ) -> Result<Option<ValidityPublicInputs>, ServerError> {
//         let validity_pis = self
//             .inner_block_validity_prover
//             .lock()
//             .unwrap()
//             .get_validity_pis(block_number);
//         Ok(validity_pis)
//     }

//     async fn get_sender_leaves(
//         &self,
//         block_number: u32,
//     ) -> Result<Option<Vec<SenderLeaf>>, ServerError> {
//         let sender_leaves = self
//             .inner_block_validity_prover
//             .lock()
//             .unwrap()
//             .get_sender_leaves(block_number);
//         Ok(sender_leaves)
//     }

//     async fn get_block_merkle_proof(
//         &self,
//         root_block_number: u32,
//         leaf_block_number: u32,
//     ) -> Result<BlockHashMerkleProof, ServerError> {
//         let block_merkle_proof = self
//             .inner_block_validity_prover
//             .lock()
//             .unwrap()
//             .get_block_merkle_proof(root_block_number, leaf_block_number)
//             .map_err(|e| ServerError::InternalError(e.to_string()))?;
//         Ok(block_merkle_proof)
//     }

//     async fn get_deposit_merkle_proof(
//         &self,
//         block_number: u32,
//         deposit_index: usize,
//     ) -> Result<DepositMerkleProof, ServerError> {
//         let deposit_merkle_proof = self
//             .inner_block_validity_prover
//             .lock()
//             .unwrap()
//             .get_deposit_merkle_proof(block_number, deposit_index)
//             .map_err(|e| ServerError::InternalError(e.to_string()))?;
//         Ok(deposit_merkle_proof)
//     }
// }

use intmax2_zkp::{
    circuits::validity::validity_pis::ValidityPublicInputs,
    common::{
        trees::{
            block_hash_tree::BlockHashMerkleProof, deposit_tree::DepositMerkleProof,
            sender_tree::SenderLeaf,
        },
        witness::update_witness::UpdateWitness,
    },
    ethereum_types::{bytes32::Bytes32, u256::U256},
};
use plonky2::{field::goldilocks_field::GoldilocksField, plonk::config::PoseidonGoldilocksConfig};
use serde::{Deserialize, Serialize};

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockNumberResponse {
    pub block_number: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountIdQuery {
    pub pubkey: U256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountIdResponse {
    pub account_id: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUpdateWitnessQuery {
    pub pubkey: U256,
    pub root_block_number: u32,
    pub leaf_block_number: u32,
    pub is_prev_account_tree: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUpdateWitnessResponse {
    pub update_witness: UpdateWitness<F, C, D>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositIndexAndBlockNumberQuery {
    pub deposit_hash: Bytes32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositIndexAndBlockNumberResponse {
    pub deposit_index_and_block_number: Option<(usize, u32)>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockNumberByTxTreeRootQuery {
    pub tx_tree_root: Bytes32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockNumberByTxTreeRootResponse {
    pub block_number: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetValidityPisQuery {
    pub block_number: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetValidityPisResponse {
    pub validity_pis: Option<ValidityPublicInputs>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSenderLeavesQuery {
    pub block_number: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSenderLeavesResponse {
    pub sender_leaves: Option<Vec<SenderLeaf>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockMerkleProofQuery {
    pub root_block_number: u32,
    pub leaf_block_number: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockMerkleProofResponse {
    pub block_merkle_proof: BlockHashMerkleProof,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositMerkleProofQuery {
    pub block_number: u32,
    pub deposit_index: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositMerkleProofResponse {
    pub deposit_merkle_proof: DepositMerkleProof,
}
