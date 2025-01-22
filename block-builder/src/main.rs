use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use block_builder::{
    api::{api::block_builder_scope, block_builder::BlockBuilder, state::State},
    Env,
};
use intmax2_client_sdk::external_api::contract::utils::get_address;
use server_common::{
    health_check::{health_check, set_name_and_version},
    logger,
};
use std::io;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_name_and_version(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    logger::init_logger().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    dotenv::dotenv().ok();

    let env = envy::from_env::<Env>()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("env error: {}", e)))?;
    log::info!(
        "Starting block builder with block builder address: {:?}",
        get_address(env.l2_chain_id, env.block_builder_private_key)
    );

    let eth_allowance_for_block = ethers::utils::parse_ether(env.eth_allowance_for_block).unwrap();
    let block_builder = BlockBuilder::new(
        &env.l2_rpc_url,
        env.l2_chain_id,
        env.rollup_contract_address,
        env.rollup_contract_deployed_block_number,
        env.block_builder_private_key,
        eth_allowance_for_block,
        &env.validity_prover_base_url,
    );
    let state = State::new(block_builder);
    state.run().await;

    let state = Data::new(state);
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(TracingLogger::<logger::CustomRootSpanBuilder>::new())
            .app_data(state.clone())
            .service(health_check)
            .service(block_builder_scope())
    })
    .bind(format!("0.0.0.0:{}", env.port))?
    .run()
    .await
}
