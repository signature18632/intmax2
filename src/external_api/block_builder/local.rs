use std::sync::{Arc, Mutex};

use intmax2_zkp::mock::{block_builder::BlockBuilder as InnerBlockBuilder, contract::MockContract};

use crate::external_api::block_validity_prover::local::LocalBlockValidityProver;

pub struct LocalBlockBuilder {
    pub contract: Arc<Mutex<MockContract>>,
    pub validity_prover: Arc<Mutex<LocalBlockValidityProver>>,
    pub inner_block_builder: Arc<Mutex<InnerBlockBuilder>>,
}
