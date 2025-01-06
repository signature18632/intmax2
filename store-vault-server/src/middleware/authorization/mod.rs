use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    middleware::Next,
    web::Json,
    Error,
};
use intmax2_interfaces::api::store_vault_server::types::SaveDataRequestWithSignature;
use signature::verify_signature;

pub mod signature;

pub trait RequestWithSignature {
    fn verify(&self) -> anyhow::Result<()>;
}

impl RequestWithSignature for SaveDataRequestWithSignature {
    fn verify(&self) -> anyhow::Result<()> {
        verify_signature(
            self.signature.clone(),
            self.pubkey.clone(),
            self.data.clone(),
        )
    }
}

pub async fn authorization_middleware(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let body: Json<SaveDataRequestWithSignature> =
        req.extract::<Json<SaveDataRequestWithSignature>>().await?;

    match body.verify() {
        Ok(_) => next.call(req).await,
        Err(err) => Err(ErrorUnauthorized(err)),
    }
}
