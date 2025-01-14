CREATE TABLE IF NOT EXISTS encrypted_sender_proof_set (
    pubkey VARCHAR(66) PRIMARY KEY,
    encrypted_data BYTEA NOT NULL
);

CREATE TABLE IF NOT EXISTS encrypted_user_data (
    pubkey VARCHAR(66) PRIMARY KEY,
    encrypted_data BYTEA NOT NULL,
    digest BYTEA NOT NULL,
    timestamp BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS encrypted_data (
    uuid TEXT PRIMARY KEY,
    data_type INTEGER NOT NULL,
    pubkey VARCHAR(66) NOT NULL,
    encrypted_data BYTEA NOT NULL,
    timestamp BIGINT NOT NULL
);

CREATE INDEX idx_encrypted_data_timestamp ON encrypted_data(timestamp);