use intmax2_zkp::{common::tx::Tx, ethereum_types::bytes32::Bytes32};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::external_api::{
    common::{error::ServerError, response::ServerCommonResponse},
    utils::{
        handler::{handle_response, ResponseType},
        retry::with_retry,
    },
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TxRequestResponse {
    message: String,
}

pub async fn tx_request(server_base_url: &str, pubkey: Bytes32, tx: Tx) -> Result<(), ServerError> {
    let url = format!("{}/transaction", server_base_url);
    let request = json!({
        "sender": pubkey,
        "transferTreeRoot": tx.transfer_tree_root,
        "nonce": tx.nonce,
        "powNonce": 0, // TODO: implement PoW
        "signature": "" // TODO: implement signature
    });

    let response = with_retry(|| async {
        reqwest::Client::new()
            .post(&url)
            .json(&request)
            .send()
            .await
    })
    .await
    .map_err(|e| ServerError::NetworkError(format!("Failed to send tx request: {}", e)))?;

    match handle_response(response).await? {
        ResponseType::Success(response) => {
            let info: ServerCommonResponse<TxRequestResponse> =
                response.json().await.map_err(|e| {
                    ServerError::DeserializationError(format!(
                        "Failed to parse tx request response: {}",
                        e
                    ))
                })?;
            if !info.success {
                return Err(ServerError::InvalidResponse(info.data.message));
            }
            Ok(())
        }
        _ => Err(ServerError::InternalError(
            "Failed to send tx request".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use intmax2_zkp::common::tx::Tx;
    use intmax2_zkp::ethereum_types::bytes32::Bytes32;
    use intmax2_zkp::ethereum_types::u32limb_trait::U32LimbTrait;

    use crate::external_api::block_builder::server::tx_request::tx_request;

    #[tokio::test]
    async fn test_tx_request() -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();
        let server_base_url = "http://localhost:4000/v1";
        let pubkey = Bytes32::rand(&mut rng);
        let tx = Tx::rand(&mut rng);
        tx_request(server_base_url, pubkey, tx).await?;
        Ok(())
    }
}
