use actix_web::{web::Data, App, HttpServer};
use api::{
    balance_prover::api::balance_prover_scope, block_builder::api::block_builder_scope,
    block_validity_prover::api::block_validity_prover_scope, contract::api::contract_scope,
    state::State, store_vault_server::api::store_vault_server_scope,
    withdrawal_aggregator::api::withdrawal_aggregator_scope,
};

pub mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(State::new()))
            .service(balance_prover_scope())
            .service(block_builder_scope())
            .service(block_validity_prover_scope())
            .service(contract_scope())
            .service(store_vault_server_scope())
            .service(withdrawal_aggregator_scope())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
