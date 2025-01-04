use actix_web::{
    post,
    web::{Data, Json},
    Error,
};
use intmax2_interfaces::api::validity_prover::types::{
    AssignResponse, CompleteRequest, HeartBeatRequest,
};

use crate::api::state::State;

#[post("/assign")]
pub async fn assign_task(data: Data<State>) -> Result<Json<AssignResponse>, Error> {
    let task = data.coordinator.assign_task().await.map_err(|e| {
        log::error!("Failed to assign task: {:?}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;
    Ok(Json(AssignResponse { task }))
}

#[post("/complete")]
pub async fn complete_task(
    data: Data<State>,
    request: Json<CompleteRequest>,
) -> Result<Json<()>, Error> {
    data.coordinator
        .complete_task(request.block_number, &request.transition_proof)
        .await
        .map_err(|e| {
            log::error!("Failed to complete task: {:?}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;
    Ok(Json(()))
}

#[post("/heartbeat")]
pub async fn heartbeat(
    data: Data<State>,
    request: Json<HeartBeatRequest>,
) -> Result<Json<()>, Error> {
    data.coordinator
        .heartbeat(request.block_number)
        .await
        .map_err(|e| {
            log::error!("Failed to heartbeat: {:?}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;
    Ok(Json(()))
}

pub fn coordinator_scope() -> actix_web::Scope {
    actix_web::web::scope("/coordinator")
        .service(assign_task)
        .service(complete_task)
        .service(heartbeat)
}
