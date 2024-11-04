#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    InsufficientFunds,
}

#[async_trait]
pub trait Contract {
    async fn deposit(
        &self,
        signer_private_key: H256,
        pubkey_salt_hash: H256,
        token_address: Address,
        amount: U256,
    ) -> Result<(), ContractError>;

    async fn post_registration_block(
        &self,
        tx_tree_root: Bytes32,
        sender_flag: Bytes16,
        agg_pubkey: FlatG1,
        agg_signature: FlatG2,
        message_point: FlatG2,
        sender_public_keys: Vec<U256>,
    ) -> Result<(), ContractError>;

    async fn post_non_registration_block(
        &self,
        tx_tree_root: Bytes32,
        sender_flag: Bytes16,
        agg_pubkey: FlatG1,
        agg_signature: FlatG2,
        message_point: FlatG2,
        public_keys_hash: Bytes32,
        account_ids: Vec<u8>, // dummy accounts are trimmed
    ) -> Result<(), ContractError>;
}
