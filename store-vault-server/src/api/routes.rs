use std::str::FromStr;

use actix_web::{
    error::ErrorUnauthorized,
    get, post,
    web::{Data, Json, Path},
    Error,
};
use intmax2_interfaces::api::store_vault_server::{
    interface::DataType,
    types::{
        BatchGetDataQuery, BatchGetDataResponse, BatchSaveDataRequest, BatchSaveDataResponse,
        GetBalanceProofQuery, GetBalanceProofResponse, GetDataAllAfterRequestWithSignature,
        GetDataAllAfterResponse, GetDataQuery, GetDataResponse, GetUserDataRequestWithSignature,
        GetUserDataResponse, SaveBalanceProofRequest, SaveDataRequestWithSignature,
        SaveDataResponse,
    },
};
use serde_qs::actix::QsQuery;

use crate::{api::state::State, app::authorization::RequestWithSignature as _};

#[post("/save-user-data")]
pub async fn save_user_data(
    state: Data<State>,
    request: Json<SaveDataRequestWithSignature>,
) -> Result<Json<()>, Error> {
    let request = request.into_inner();
    request.verify().map_err(ErrorUnauthorized)?;
    state
        .store_vault_server
        .save_user_data(request.pubkey, request.data)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Json(()))
}

#[post("/get-user-data")]
pub async fn get_user_data(
    state: Data<State>,
    request: Json<GetUserDataRequestWithSignature>,
) -> Result<Json<GetUserDataResponse>, Error> {
    let request = request.into_inner();
    request.auth.verify().map_err(ErrorUnauthorized)?;

    let data = state
        .store_vault_server
        .get_user_data(request.auth.pubkey)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Json(GetUserDataResponse { data }))
}

#[post("/{type}/batch-save")]
pub async fn batch_save_data(
    state: Data<State>,
    path: Path<String>,
    request: Json<BatchSaveDataRequest>,
) -> Result<Json<BatchSaveDataResponse>, Error> {
    const MAX_BATCH_SIZE: usize = 1000;

    let data_type = path.into_inner();
    let data_type = DataType::from_str(data_type.as_str())
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Invalid type: {}", e)))?;

    let requests = request.into_inner().requests;
    match data_type {
        DataType::Tx => {
            return Err(actix_web::error::ErrorBadRequest(format!(
                "data_type {} is not supported",
                data_type
            )));
        }
        _ => {
            println!("No authorization required for {}", data_type);
        }
    }

    if requests.len() > MAX_BATCH_SIZE {
        return Err(actix_web::error::ErrorBadRequest(format!(
            "Batch size exceeds maximum limit of {}",
            MAX_BATCH_SIZE
        )));
    }

    let uuids = state
        .store_vault_server
        .batch_save_data(data_type, requests)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Json(BatchSaveDataResponse { uuids }))
}

#[post("/{type}/get-all-after")]
pub async fn get_data_all_after(
    state: Data<State>,
    path: Path<String>,
    request: Json<GetDataAllAfterRequestWithSignature>,
) -> Result<Json<GetDataAllAfterResponse>, Error> {
    let data_type = path.into_inner();
    let data_type = DataType::from_str(data_type.as_str())
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Invalid type: {}", e)))?;
    let request = request.into_inner();
    request.auth.verify().map_err(ErrorUnauthorized)?;

    let data = state
        .store_vault_server
        .get_data_all_after(data_type, request.auth.pubkey, request.timestamp)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(Json(GetDataAllAfterResponse { data }))
}

pub fn store_vault_server_scope() -> actix_web::Scope {
    actix_web::web::scope("/store-vault-server")
        .service(save_user_data)
        .service(get_user_data)
        .service(batch_save_data)
        .service(get_data_all_after)
}
