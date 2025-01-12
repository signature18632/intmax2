use std::time::Duration;

use anyhow::{anyhow, Ok, Result};
use intmax2_interfaces::{
    api::store_vault_server::{
        interface::{DataType, SaveDataEntry},
        types::DataWithMetaData,
    },
    data::meta_data::MetaData,
    utils::digest::get_digest,
};
use intmax2_zkp::ethereum_types::{bytes32::Bytes32, u256::U256, u32limb_trait::U32LimbTrait};

use sqlx::{postgres::PgPoolOptions, PgPool, Postgres};
use uuid::Uuid;

use crate::Env;

// CREATE TABLE IF NOT EXISTS encrypted_sender_proof_set (
//     pubkey VARCHAR(66) PRIMARY KEY,
//     encrypted_data BYTEA NOT NULL
// );

// CREATE TABLE IF NOT EXISTS encrypted_user_data (
//     pubkey VARCHAR(66) PRIMARY KEY,
//     encrypted_data BYTEA NOT NULL,
//     digest BYTEA NOT NULL,
//     timestamp BIGINT NOT NULL
// );

// CREATE TABLE IF NOT EXISTS encrypted_data (
//     uuid TEXT PRIMARY KEY,
//     data_type INTEGER NOT NULL,
//     pubkey VARCHAR(66) NOT NULL,
//     encrypted_data BYTEA NOT NULL,
//     timestamp BIGINT NOT NULL
// );

pub struct StoreVaultServer {
    pool: PgPool,
}

impl StoreVaultServer {
    pub async fn new(env: &Env) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(env.database_max_connections)
            .idle_timeout(Duration::from_secs(env.database_timeout))
            .connect(&env.database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn save_user_data(
        &self,
        pubkey: U256,
        prev_digest: Option<Bytes32>,
        encrypted_data: &[u8],
    ) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        let result = self.get_user_data_and_digest(&mut tx, pubkey).await?;
        // validation
        if let Some(prev_digest) = prev_digest {
            if let Some((_, digest)) = result {
                if digest != prev_digest {
                    return Err(anyhow!(
                        "Prev digest mismatch {} != {}",
                        digest,
                        prev_digest
                    ));
                }
            } else {
                return Err(anyhow!(
                    "User data not found though prev_digest is provided"
                ));
            }
        } else if result.is_some() {
            return Err(anyhow!(
                "User data already exists but prev_digest is not provided"
            ));
        }
        let pubkey_hex = pubkey.to_hex();
        let digest = get_digest(encrypted_data);
        let digest_serialized = bincode::serialize(&digest).unwrap();
        sqlx::query!(
            r#"
            INSERT INTO encrypted_user_data (pubkey, encrypted_data, digest)
            VALUES ($1, $2, $3)
            ON CONFLICT (pubkey) DO UPDATE SET encrypted_data = EXCLUDED.encrypted_data
            "#,
            pubkey_hex,
            encrypted_data,
            digest_serialized
        )
        .execute(tx.as_mut())
        .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_user_data(&self, pubkey: U256) -> Result<Option<Vec<u8>>> {
        let mut tx = self.pool.begin().await?;
        let result = self.get_user_data_and_digest(&mut tx, pubkey).await?;
        tx.commit().await?;
        Ok(result.map(|(data, _)| data))
    }

    async fn get_user_data_and_digest(
        &self,
        tx: &mut sqlx::Transaction<'_, Postgres>,
        pubkey: U256,
    ) -> Result<Option<(Vec<u8>, Bytes32)>> {
        let pubkey_hex = pubkey.to_hex();
        let record = sqlx::query!(
            r#"
            SELECT encrypted_data, digest FROM encrypted_user_data WHERE pubkey = $1
            "#,
            pubkey_hex
        )
        .fetch_optional(tx.as_mut())
        .await?;
        Ok(record.map(|r| (r.encrypted_data, Bytes32::from_bytes_be(&r.digest))))
    }

    pub async fn batch_save_data(&self, entries: &[SaveDataEntry]) -> Result<Vec<String>> {
        // Prepare values for bulk insert
        let data_types: Vec<i32> = entries.iter().map(|entry| entry.data_type as i32).collect();
        let pubkeys: Vec<String> = entries.iter().map(|entry| entry.pubkey.to_hex()).collect();
        let uuids: Vec<String> = (0..entries.len())
            .map(|_| Uuid::new_v4().to_string())
            .collect();
        let timestamps: Vec<i64> = vec![chrono::Utc::now().timestamp(); entries.len()];
        let encrypted_data: Vec<Vec<u8>> = entries
            .into_iter()
            .map(|entry| entry.encrypted_data.clone())
            .collect();

        // Execute the bulk insert
        sqlx::query!(
            r#"
            INSERT INTO encrypted_data 
            (data_type, pubkey, uuid, timestamp, encrypted_data)
            SELECT 
                UNNEST($1::integer[]),
                UNNEST($2::text[]),
                UNNEST($3::text[]),
                UNNEST($4::bigint[]),
                UNNEST($5::bytea[])
            "#,
            &data_types,
            &pubkeys,
            &uuids,
            &timestamps,
            &encrypted_data,
        )
        .execute(&self.pool)
        .await?;

        Ok(uuids)
    }

    pub async fn get_data_all_after(
        &self,
        data_type: DataType,
        pubkey: U256,
        timestamp: u64,
    ) -> Result<Vec<DataWithMetaData>> {
        let pubkey_hex = pubkey.to_hex();

        let records = sqlx::query!(
            r#"
            SELECT uuid, timestamp, encrypted_data
            FROM encrypted_data
            WHERE data_type = $1 AND pubkey = $2 AND timestamp >= $3
            ORDER BY timestamp ASC
            "#,
            data_type as i32,
            pubkey_hex,
            timestamp as i64
        )
        .fetch_all(&self.pool)
        .await?;

        let result = records
            .into_iter()
            .map(|r| {
                let meta_data = MetaData {
                    uuid: r.uuid,
                    timestamp: r.timestamp as u64,
                    block_number: None,
                };
                DataWithMetaData {
                    meta_data,
                    data: r.encrypted_data,
                }
            })
            .collect();

        Ok(result)
    }
}
