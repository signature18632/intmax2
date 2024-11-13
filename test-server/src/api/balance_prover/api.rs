use actix_web::{
    post,
    web::{Data, Json},
    Error,
};
use intmax2_core_sdk::external_api::balance_prover::interface::BalanceProverInterface as _;

use crate::api::{
    balance_prover::types::{ProveResponse, ProveSendRequest, ProveSpentRequest},
    state::State,
};

#[post("/prove-spent")]
pub async fn prove_spent(
    state: Data<State>,
    request: Json<ProveSpentRequest>,
) -> Result<Json<ProveResponse>, Error> {
    let proof = state
        .balance_prover
        .prove_spent(&request.spent_witness)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(Json(ProveResponse { proof }))
}

#[post("/prove-send")]
pub async fn prove_send(
    state: Data<State>,
    request: Json<ProveSendRequest>,
) -> Result<Json<ProveResponse>, Error> {
    let proof = state
        .balance_prover
        .prove_send(
            request.pubkey,
            &request.tx_witnes,
            &request.update_witness,
            &request.spent_proof,
            &request.prev_proof,
        )
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(Json(ProveResponse { proof }))
}
