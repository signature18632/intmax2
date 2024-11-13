use actix_web::{
    post,
    web::{Data, Json},
    Error,
};
use intmax2_core_sdk::external_api::block_builder::interface::BlockBuilderInterface;

use crate::api::state::State;

use super::types::{TxRequestRequest, TxRequestResponse};

#[post("/tx-request")]
pub async fn tx_request(
    state: Data<State>,
    request: Json<TxRequestRequest>,
) -> Result<Json<TxRequestResponse>, Error> {
    let request = request.into_inner();
    state
        .block_builder
        .send_tx_request("", request.pubkey, request.tx, None)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(Json(TxRequestResponse { success: true }))
}
