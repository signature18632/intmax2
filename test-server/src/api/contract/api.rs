use actix_web::{
    post,
    web::{Data, Json},
    Error,
};
use ethers::types::H256;
use intmax2_core_sdk::external_api::contract::interface::ContractInterface as _;

use crate::api::state::State;

use super::types::DepositNativeTokenRequest;

#[post("/deposit-native-token")]
pub async fn deposit_native_token(
    data: Data<State>,
    request: Json<DepositNativeTokenRequest>,
) -> Result<Json<()>, Error> {
    let request = request.into_inner();
    data.contract
        .deposit_native_token(H256::zero(), request.pubkey_salt_hash, request.amount)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(Json(()))
}

pub fn contract_scope() -> actix_web::Scope {
    actix_web::web::scope("/contract").service(deposit_native_token)
}
