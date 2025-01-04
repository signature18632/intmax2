use std::{env, time::Duration};

use server_common::{health_check::set_name_and_version, logger::init_logger};
use validity_prover_worker::{app::worker::Worker, EnvVar};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // set for the logs
    set_name_and_version(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    dotenv::dotenv().ok();
    init_logger()?;

    let env = envy::from_env::<EnvVar>().unwrap();
    let worker = Worker::new(&env);
    worker.run();
    log::info!("Worker started");
    loop {
        // live forever until killed
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
