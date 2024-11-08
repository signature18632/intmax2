use intmax2_core_sdk::external_api::{
    block_validity_prover::local::LocalBlockValidityProver, contract::local::LocalContract,
};

#[tokio::test]
async fn e2e_test() {
    let contract = LocalContract::new();
    let validity_prover = LocalBlockValidityProver::new(contract.0.clone());
}
