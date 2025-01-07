use intmax2_client_sdk::utils::signature::verify_signature;
use intmax2_interfaces::api::store_vault_server::types::SaveDataRequestWithSignature;

pub trait RequestWithSignature {
    fn verify(&self) -> anyhow::Result<()>;
}

impl RequestWithSignature for SaveDataRequestWithSignature {
    fn verify(&self) -> anyhow::Result<()> {
        if self.signature.is_none() {
            anyhow::bail!("Signature is missing");
        }

        verify_signature(
            self.signature.clone().unwrap(),
            self.pubkey.clone(),
            self.data.clone(),
        )
    }
}

// pub async fn authorization_middleware(
//     mut req: ServiceRequest,
//     next: Next<impl MessageBody>,
// ) -> Result<ServiceResponse<impl MessageBody>, Error> {
//     match req.path() {
//         "/store-vault-server/withdrawal/save" | "/store-vault-server/tx/save" => {
//             let body = req.extract::<Json<SaveDataRequestWithSignature>>().await?;
//             println!("body: {:?}", body);

//             let result = body.verify();
//             println!("Verifying signature for {:?}", result);
//             match result {
//                 Ok(_) => next.call(req).await,
//                 Err(err) => Err(ErrorUnauthorized(err)),
//             }
//         }
//         _ => {
//             println!("No authorization required for {}", req.path());
//             next.call(req).await
//         }
//     }
// }
