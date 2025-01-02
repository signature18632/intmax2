use std::sync::Arc;

use crate::app::witness_generator::WitnessGenerator;

#[derive(Clone)]
pub struct State {
    pub validity_prover: Arc<WitnessGenerator>,
}

impl State {
    pub fn new(validity_prover: WitnessGenerator) -> Self {
        let _ = validity_prover.validity_processor(); // initialize
        log::info!("State initialized");
        Self {
            validity_prover: Arc::new(validity_prover),
        }
    }

    pub async fn sync_task(&self) -> anyhow::Result<()> {
        self.validity_prover.sync().await?;
        Ok(())
    }
}
