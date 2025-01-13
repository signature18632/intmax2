use crate::api::state::State;
use actix_web::{
    error::ErrorUnauthorized,
    post,
    web::{Data, Json},
    Error,
};
use intmax2_interfaces::{
    api::store_vault_server::types::{
        BatchSaveDataRequest, BatchSaveDataResponse, GetDataAllAfterRequest,
        GetDataAllAfterResponse, GetSenderProofSetRequest, GetSenderProofSetResponse,
        GetUserDataRequest, GetUserDataResponse, SaveSenderProofSetRequest, SaveUserDataRequest,
    },
    utils::signature::Signable as _,
};

#[post("/save-user-data")]
pub async fn save_user_data(
    state: Data<State>,
    request: Json<SaveUserDataRequest>,
) -> Result<Json<()>, Error> {
    request.verify(&request.auth).map_err(ErrorUnauthorized)?;
    state
        .store_vault_server
        .save_user_data(request.auth.pubkey, request.prev_digest, &request.data)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Json(()))
}

#[post("/get-user-data")]
pub async fn get_user_data(
    state: Data<State>,
    request: Json<GetUserDataRequest>,
) -> Result<Json<GetUserDataResponse>, Error> {
    request.verify(&request.auth).map_err(ErrorUnauthorized)?;
    let data = state
        .store_vault_server
        .get_user_data(request.auth.pubkey)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Json(GetUserDataResponse { data }))
}

#[post("/save-sender-proof-set")]
pub async fn save_sender_proof_set(
    state: Data<State>,
    request: Json<SaveSenderProofSetRequest>,
) -> Result<Json<()>, Error> {
    request.verify(&request.auth).map_err(ErrorUnauthorized)?;
    state
        .store_vault_server
        .save_sender_proof_set(request.auth.pubkey, &request.data)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Json(()))
}

#[post("/get-sender-proof-set")]
pub async fn get_sender_proof_set(
    state: Data<State>,
    request: Json<GetSenderProofSetRequest>,
) -> Result<Json<GetSenderProofSetResponse>, Error> {
    request.verify(&request.auth).map_err(ErrorUnauthorized)?;
    let data = state
        .store_vault_server
        .get_sender_proof_set(request.auth.pubkey)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Json(GetSenderProofSetResponse { data }))
}

#[post("/batch-save")]
pub async fn batch_save_data(
    state: Data<State>,
    request: Json<BatchSaveDataRequest>,
) -> Result<Json<BatchSaveDataResponse>, Error> {
    const MAX_BATCH_SIZE: usize = 1000;
    if request.data.len() > MAX_BATCH_SIZE {
        return Err(actix_web::error::ErrorBadRequest(format!(
            "Batch size exceeds maximum limit of {}",
            MAX_BATCH_SIZE
        )));
    }
    request.verify(&request.auth).map_err(ErrorUnauthorized)?;
    let pubkey = request.auth.pubkey;
    for entry in &request.data {
        if entry.data_type.need_auth() {
            if entry.pubkey != pubkey {
                return Err(ErrorUnauthorized(format!(
                    "Data type {:?} requires auth but given pubkey is different",
                    entry.data_type,
                )));
            }
        }
    }
    let uuids = state
        .store_vault_server
        .batch_save_data(&request.data)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Json(BatchSaveDataResponse { uuids }))
}

#[post("/get-all-after")]
pub async fn get_data_all_after(
    state: Data<State>,
    request: Json<GetDataAllAfterRequest>,
) -> Result<Json<GetDataAllAfterResponse>, Error> {
    request.verify(&request.auth).map_err(ErrorUnauthorized)?;
    let data = state
        .store_vault_server
        .get_data_all_after(request.data_type, request.auth.pubkey, request.timestamp)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Json(GetDataAllAfterResponse { data }))
}

pub fn store_vault_server_scope() -> actix_web::Scope {
    actix_web::web::scope("/store-vault-server")
        .service(save_user_data)
        .service(get_user_data)
        .service(save_sender_proof_set)
        .service(get_sender_proof_set)
        .service(batch_save_data)
        .service(get_data_all_after)
}
