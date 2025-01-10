use intmax2_client_sdk::utils::signature::{hex_to_bytes, verify_signature};
use intmax2_interfaces::api::store_vault_server::types::{
    AuthInfoForGetData, SaveDataRequestWithSignature,
};

pub trait RequestWithSignature {
    fn verify(&self) -> anyhow::Result<()>;
}

impl RequestWithSignature for SaveDataRequestWithSignature {
    fn verify(&self) -> anyhow::Result<()> {
        if self.auth.is_none() {
            anyhow::bail!("Signature is missing");
        }

        verify_signature(
            self.auth.clone().unwrap().signature,
            self.pubkey,
            self.data.clone(),
        )
    }
}

impl RequestWithSignature for AuthInfoForGetData {
    fn verify(&self) -> anyhow::Result<()> {
        let challenge = hex_to_bytes(&self.challenge)?;
        if challenge.len() != 32 {
            anyhow::bail!("Challenge should be a 32-byte hex string");
        }

        verify_signature(self.signature.clone(), self.pubkey, challenge)
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
