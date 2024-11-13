use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::{
    balance_prover::api::balance_prover_scope, block_builder::api::block_builder_scope,
    block_validity_prover::api::block_validity_prover_scope, contract::api::contract_scope,
    health_check::health_check, state::State, store_vault_server::api::store_vault_server_scope,
    withdrawal_aggregator::api::withdrawal_aggregator_scope,
};
use log::init_logger;

pub mod api;
pub mod log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger()?;

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(State::new()))
            .service(health_check)
            .service(balance_prover_scope())
            .service(block_builder_scope())
            .service(block_validity_prover_scope())
            .service(contract_scope())
            .service(store_vault_server_scope())
            .service(withdrawal_aggregator_scope())
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
