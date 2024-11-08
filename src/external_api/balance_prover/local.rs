use std::sync::{Arc, Mutex};

use intmax2_zkp::{
    circuits::balance::balance_processor::BalanceProcessor,
    mock::block_validity_prover::BlockValidityProver,
};
use plonky2::{field::goldilocks_field::GoldilocksField, plonk::config::PoseidonGoldilocksConfig};

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

pub struct LocalBalanceProver {
    pub validity_prover: Arc<Mutex<BlockValidityProver<F, C, D>>>,
    pub balance_processor: BalanceProcessor<F, C, D>,
}
