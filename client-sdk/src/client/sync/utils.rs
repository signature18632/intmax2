use intmax2_interfaces::data::user_data::UserData;
use intmax2_zkp::{
    common::{salt::Salt, transfer::Transfer, trees::transfer_tree::TransferTree},
    constants::{NUM_TRANSFERS_IN_TX, TRANSFER_TREE_HEIGHT},
};
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{config::PoseidonGoldilocksConfig, proof::ProofWithPublicInputs},
};

use super::error::SyncError;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

pub fn generate_salt() -> Salt {
    let mut rng = rand::thread_rng();
    Salt::rand(&mut rng)
}

pub fn generate_transfer_tree(transfers: &[Transfer]) -> TransferTree {
    let mut transfers = transfers.to_vec();
    transfers.resize(NUM_TRANSFERS_IN_TX, Transfer::default());
    let mut transfer_tree = TransferTree::new(TRANSFER_TREE_HEIGHT);
    for transfer in &transfers {
        transfer_tree.push(*transfer);
    }
    transfer_tree
}

pub fn get_balance_proof(
    user_data: &UserData,
) -> Result<Option<ProofWithPublicInputs<F, C, D>>, SyncError> {
    let balance_proof = user_data
        .balance_proof
        .as_ref()
        .map(|bp| bp.decompress())
        .transpose()?;
    Ok(balance_proof)
}
