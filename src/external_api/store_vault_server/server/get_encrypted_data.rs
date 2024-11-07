use intmax2_zkp::{ethereum_types::bytes32::Bytes32, mock::data::meta_data::MetaData};
use reqwest::Response;
use serde::{Deserialize, Serialize};

use crate::external_api::{
    common::{error::ServerError, response::ServerCommonResponse},
    utils::{
        encode::decode_base64,
        handler::{handle_response, ResponseType},
        retry::with_retry,
    },
};

use super::data_type::EncryptedDataType;

pub async fn get_encrypted_data(
    server_base_url: &str,
    data_type: EncryptedDataType,
    uuid: &str,
) -> Result<Option<(MetaData, Vec<u8>)>, ServerError> {
    let url = format!("{}/{}/{}", server_base_url, data_type, uuid);

    let response = with_retry(|| async { reqwest::Client::new().get(&url).send().await })
        .await
        .map_err(|e| {
            ServerError::NetworkError(format!("Failed to get encrypted data from server: {}", e))
        })?;
    match handle_response(response).await? {
        ResponseType::Success(response) => {
            let (meta, data) = deserialize_response(response, data_type).await?;
            Ok(Some((meta, data)))
        }
        ResponseType::NotFound(error) => {
            log::warn!(
                "Failed to get encrypted {} data: {}",
                data_type,
                error.message
            );
            Ok(None)
        }
        ResponseType::ServerError(error) => {
            log::error!("Failed to get encrypted data: {}", error.message);
            Err(ServerError::ServerError(error.message))
        }
        ResponseType::UnknownError(error) => {
            log::error!("Failed to get encrypted data: {}", error);
            Err(ServerError::UnknownError(error))
        }
    }
}

async fn deserialize_response(
    response: Response,
    data_type: EncryptedDataType,
) -> Result<(MetaData, Vec<u8>), ServerError> {
    match data_type {
        EncryptedDataType::Deposit => {
            let response: ServerCommonResponse<GetDepositData> =
                response.json().await.map_err(|e| {
                    ServerError::DeserializationError(format!(
                        "Error while deserializing response: {}",
                        e
                    ))
                })?;
            let uuid = response.data.deposit.uuid;
            let data =
                decode_base64(&response.data.deposit.encrypted_deposit_data).map_err(|e| {
                    ServerError::DeserializationError(format!("Error while decoding data: {}", e))
                })?;
            let created_at = response.data.deposit.created_at;
            let meta = MetaData {
                uuid,
                timestamp: created_at,
                block_number: None,
            };
            Ok((meta, data))
        }
        EncryptedDataType::Transaction => {
            let response: ServerCommonResponse<GetTransactionData> =
                response.json().await.map_err(|e| {
                    ServerError::DeserializationError(format!(
                        "Error while deserializing response: {}",
                        e
                    ))
                })?;
            let uuid = response.data.transaction.uuid;
            let data = decode_base64(&response.data.transaction.encrypted_transaction_data)
                .map_err(|e| {
                    ServerError::DeserializationError(format!("Error while decoding data: {}", e))
                })?;
            let created_at = response.data.transaction.created_at;
            let meta = MetaData {
                uuid,
                timestamp: created_at,
                block_number: None,
            };
            Ok((meta, data))
        }
        EncryptedDataType::Transfer => {
            let response: ServerCommonResponse<GetTransferData> =
                response.json().await.map_err(|e| {
                    ServerError::DeserializationError(format!(
                        "Error while deserializing response: {}",
                        e
                    ))
                })?;
            let uuid = response.data.transfer.uuid;
            let data =
                decode_base64(&response.data.transfer.encrypted_transfer_data).map_err(|e| {
                    ServerError::DeserializationError(format!("Error while decoding data: {}", e))
                })?;
            let created_at = response.data.transfer.created_at;
            let meta = MetaData {
                uuid,
                timestamp: created_at,
                block_number: None,
            };
            Ok((meta, data))
        }
        EncryptedDataType::Withdrawal => {
            let response: ServerCommonResponse<GetWithdrawalData> =
                response.json().await.map_err(|e| {
                    ServerError::DeserializationError(format!(
                        "Error while deserializing response: {}",
                        e
                    ))
                })?;
            let uuid = response.data.transfer.uuid;
            let data =
                decode_base64(&response.data.transfer.encrypted_withdrawal_data).map_err(|e| {
                    ServerError::DeserializationError(format!("Error while decoding data: {}", e))
                })?;
            let created_at = response.data.transfer.created_at;
            let meta = MetaData {
                uuid,
                timestamp: created_at,
                block_number: None,
            };
            Ok((meta, data))
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetDepositData {
    deposit: Deposit,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Deposit {
    uuid: String,
    recipient: Bytes32,
    encrypted_deposit_data: String,
    created_at: u64,
}

#[derive(Serialize, Deserialize)]
struct GetTransactionData {
    transaction: Transaction,
}

#[derive(Serialize, Deserialize)]
struct Transaction {
    uuid: String,
    recipient: Bytes32,
    encrypted_transaction_data: String,
    created_at: u64,
}

#[derive(Serialize, Deserialize)]
struct GetTransferData {
    transfer: Transfer,
}

#[derive(Serialize, Deserialize)]
struct Transfer {
    uuid: String,
    recipient: Bytes32,
    encrypted_transfer_data: String,
    created_at: u64,
}

#[derive(Serialize, Deserialize)]
struct GetWithdrawalData {
    transfer: Withdrawal,
}

#[derive(Serialize, Deserialize)]
struct Withdrawal {
    uuid: String,
    recipient: Bytes32,
    encrypted_withdrawal_data: String,
    created_at: u64,
}
