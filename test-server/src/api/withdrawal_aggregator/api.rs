use actix_web::{
    get, post,
    web::{Data, Json},
    Error,
};
use intmax2_core_sdk::external_api::withdrawal_aggregator::interface::WithdrawalAggregatorInterface as _;

use crate::api::state::State;

use super::types::RequestWithdrawalRequest;

#[get("/wrap")]
pub async fn wrap(data: Data<State>) -> Result<Json<()>, Error> {
    data.withdrawal_aggregator
        .wrap()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(Json(()))
}

#[post("/request_withdrawal")]
pub async fn request_withdrawal(
    data: Data<State>,
    request: Json<RequestWithdrawalRequest>,
) -> Result<Json<()>, Error> {
    data.withdrawal_aggregator
        .request_withdrawal(&request.into_inner().single_withdrawal_proof)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(Json(()))
}

pub fn withdrawal_aggregator_scope() -> actix_web::Scope {
    actix_web::web::scope("/withdrawal_aggregator")
        .service(wrap)
        .service(request_withdrawal)
}
