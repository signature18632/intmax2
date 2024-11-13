use actix_web::{
    get, post,
    web::{Data, Json},
    Error,
};
use intmax2_core_sdk::external_api::block_builder::interface::BlockBuilderInterface;

use crate::api::{
    block_builder::types::{PostSignatureRequest, QueryProposalRequest, QueryProposalResponse},
    state::State,
};

use super::types::TxRequestRequest;

#[get("/construct-block")]
pub async fn construct_block(state: Data<State>) -> Result<Json<()>, Error> {
    state
        .block_builder
        .construct_block()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(Json(()))
}

#[get("/post-block")]
pub async fn post_block(state: Data<State>) -> Result<Json<()>, Error> {
    state
        .block_builder
        .post_block()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(Json(()))
}

#[get("/post-empty-block")]
pub async fn post_empty_block(state: Data<State>) -> Result<Json<()>, Error> {
    state
        .block_builder
        .post_empty_block()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(Json(()))
}

#[post("/tx-request")]
pub async fn tx_request(
    state: Data<State>,
    request: Json<TxRequestRequest>,
) -> Result<Json<()>, Error> {
    let request = request.into_inner();
    state
        .block_builder
        .send_tx_request("", request.pubkey, request.tx, None)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(Json(()))
}

#[post("/query-proposal")]
pub async fn query_proposal(
    state: Data<State>,
    request: Json<QueryProposalRequest>,
) -> Result<Json<QueryProposalResponse>, Error> {
    let request = request.into_inner();
    let block_proposal = state
        .block_builder
        .query_proposal("", request.pubkey, request.tx)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(Json(QueryProposalResponse { block_proposal }))
}

#[post("/post-signature")]
pub async fn post_signature(
    state: Data<State>,
    request: Json<PostSignatureRequest>,
) -> Result<Json<()>, Error> {
    let request = request.into_inner();
    state
        .block_builder
        .post_signature("", request.pubkey, request.tx, request.signature)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(Json(()))
}
