use crate::{
    app::{prover_coordinator::ProverCoordinator, witness_generator::WitnessGenerator},
    Env,
};

#[derive(Clone)]
pub struct State {
    pub witness_generator: WitnessGenerator,
    pub coordinator: ProverCoordinator,
}

impl State {
    pub async fn new(env: &Env) -> anyhow::Result<Self> {
        let witness_generator = WitnessGenerator::new(env).await?;
        let coordinator = ProverCoordinator::new(env).await?;

        log::info!("State initialized");
        Ok(Self {
            witness_generator,
            coordinator,
        })
    }

    pub fn job(&self) {
        self.clone().witness_generator.job();
        self.clone().coordinator.job();
    }
}
