use std::time::Duration;

use server_common::logger::init_logger;
use validity_prover_worker::{app::worker::Worker, EnvVar};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    init_logger()?;
    let env = envy::from_env::<EnvVar>().unwrap();
    let worker = Worker::new(&env);
    worker.run();
    loop {
        // live forever until killed
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
