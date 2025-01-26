use intmax2_interfaces::{
    api::{
        store_vault_server::interface::StoreVaultClientInterface,
        validity_prover::interface::ValidityProverClientInterface,
    },
    data::{
        deposit_data::DepositData,
        encryption::Encryption as _,
        meta_data::MetaDataWithBlockNumber,
        transfer_data::TransferData,
        tx_data::TxData,
        user_data::{Balances, ProcessStatus, UserData},
    },
};
use itertools::Itertools;

use intmax2_zkp::{
    circuits::claim::determine_lock_time::get_lock_time, common::signature::key_set::KeySet,
    ethereum_types::u256::U256,
};
use num_bigint::BigUint;

use crate::{
    client::strategy::withdrawal::fetch_withdrawal_info,
    external_api::contract::liquidity_contract::LiquidityContract,
};

use super::{
    deposit::fetch_deposit_info, error::StrategyError, transfer::fetch_transfer_info,
    tx::fetch_tx_info,
};

// Next sync action
#[derive(Debug, Clone)]
pub enum Action {
    Receive(Vec<ReceiveAction>),
    Tx(MetaDataWithBlockNumber, Box<TxData>), // Send tx
}

#[derive(Debug, Clone)]
pub enum ReceiveAction {
    Deposit(MetaDataWithBlockNumber, DepositData),
    Transfer(MetaDataWithBlockNumber, Box<TransferData>), // Boxed to avoid large stack size
}

impl ReceiveAction {
    pub fn meta(&self) -> &MetaDataWithBlockNumber {
        match self {
            ReceiveAction::Deposit(meta, _) => meta,
            ReceiveAction::Transfer(meta, _) => meta,
        }
    }

    pub fn apply_to_balances(&self, balances: &mut Balances) {
        match self {
            ReceiveAction::Deposit(_, data) => {
                balances.add_deposit(data);
            }
            ReceiveAction::Transfer(_, data) => {
                balances.add_transfer(data);
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PendingInfo {
    pub pending_deposit_uuids: Vec<String>,
    pub pending_transfer_uuids: Vec<String>,
}

/// Determine the sequence of receives/send tx to be incorporated into the balance proof
pub async fn determine_sequence<S: StoreVaultClientInterface, V: ValidityProverClientInterface>(
    store_vault_server: &S,
    validity_prover: &V,
    liquidity_contract: &LiquidityContract,
    key: KeySet,
    deposit_timeout: u64,
    tx_timeout: u64,
) -> Result<(Vec<Action>, PendingInfo), StrategyError> {
    log::info!("determine_sequence");
    let user_data = get_user_data(store_vault_server, key).await?;
    let mut balances = user_data.balances();
    if balances.is_insufficient() {
        return Err(StrategyError::BalanceInsufficientBeforeSync);
    }
    let tx_info = fetch_tx_info(
        store_vault_server,
        validity_prover,
        key,
        &user_data.tx_status,
        tx_timeout,
    )
    .await?;

    //  First, if there is a pending tx, return a pending error
    if let Some((meta, _tx_data)) = tx_info.pending.first() {
        return Err(StrategyError::PendingTxError(format!(
            "pending tx: {:?}",
            meta.uuid
        )));
    }

    // Then, collect deposit and transfer data
    let deposit_info = fetch_deposit_info(
        store_vault_server,
        validity_prover,
        liquidity_contract,
        key,
        &user_data.deposit_status,
        deposit_timeout,
    )
    .await?;
    let transfer_info = fetch_transfer_info(
        store_vault_server,
        validity_prover,
        key,
        &user_data.transfer_status,
        tx_timeout,
    )
    .await?;

    let mut deposits = deposit_info.settled;
    let mut transfers = transfer_info.settled;

    // Next, for each settled tx, take deposits and transfers that are strictly smaller than the block number of the tx
    let mut sequence = Vec::new();
    for (tx_meta, tx_data) in tx_info.settled.iter() {
        let receives = collect_receives(
            &Some((tx_meta.clone(), tx_data.clone())),
            &mut deposits,
            &mut transfers,
        )
        .await?;

        // Apply receives to balances
        for receive in &receives {
            receive.apply_to_balances(&mut balances);
        }
        let is_insufficient = balances.sub_tx(tx_data);
        if is_insufficient {
            if deposit_info.pending.is_empty() && transfer_info.pending.is_empty() {
                // Unresolved balance shortage
                return Err(StrategyError::BalanceInsufficientDuringSync);
            } else {
                // To incorporate the tx, you need to incorporate the pending deposit/transfer to solve the balance shortage.
                // TODO: Processing when the balance shortage is not resolved even if the pending deposit/transfer is incorporated
                return Err(StrategyError::PendingReceivesError(format!(
                    "pending receives to proceed tx: {:?}",
                    tx_meta.meta.uuid
                )));
            }
        }

        // Here tx can be incorporated

        sequence.push(Action::Receive(receives));
        sequence.push(Action::Tx(tx_meta.clone(), Box::new(tx_data.clone())));
    }

    // Finally, take all deposits and transfers
    let receives = collect_receives(&None, &mut deposits, &mut transfers).await?;
    sequence.push(Action::Receive(receives));

    let pending_deposit_uuids = deposit_info
        .pending
        .iter()
        .map(|(meta, _)| meta.uuid.clone())
        .collect();
    let pending_transfer_uuids = transfer_info
        .pending
        .iter()
        .map(|(meta, _)| meta.uuid.clone())
        .collect();

    Ok((
        sequence,
        PendingInfo {
            pending_deposit_uuids,
            pending_transfer_uuids,
        },
    ))
}

/// For each settled tx, take deposits and transfers that are strictly smaller than the block number of the tx
/// If there is no tx, take all deposit and transfer data
async fn collect_receives(
    tx: &Option<(MetaDataWithBlockNumber, TxData)>,
    deposits: &mut Vec<(MetaDataWithBlockNumber, DepositData)>,
    transfers: &mut Vec<(MetaDataWithBlockNumber, TransferData)>,
) -> Result<Vec<ReceiveAction>, StrategyError> {
    let mut receives: Vec<ReceiveAction> = Vec::new();
    if let Some((meta, _tx_data)) = tx {
        let block_number = meta.block_number;

        // take and remove deposit that are strictly smaller than the block number of the tx
        let receive_deposit = deposits
            .iter()
            .filter(|(meta, _)| meta.block_number < block_number)
            .map(|(meta, data)| ReceiveAction::Deposit(meta.clone(), data.clone()))
            .collect_vec();
        deposits.retain(|(meta, _)| meta.block_number >= block_number);

        // take and remove transfer that are strictly smaller than the block number of the tx
        let receive_transfer = transfers
            .iter()
            .filter(|(meta, _)| meta.block_number < block_number)
            .map(|(meta, data)| ReceiveAction::Transfer(meta.clone(), Box::new(data.clone())))
            .collect_vec();
        transfers.retain(|(meta, _)| meta.block_number >= block_number);

        // add to receives
        receives.extend(receive_deposit);
        receives.extend(receive_transfer);
    } else {
        // if there is no tx, take all deposit and transfer data
        let receive_deposit = deposits
            .iter()
            .map(|(meta, data)| ReceiveAction::Deposit(meta.clone(), data.clone()))
            .collect_vec();
        deposits.clear();

        let receive_transfer = transfers
            .iter()
            .map(|(meta, data)| ReceiveAction::Transfer(meta.clone(), Box::new(data.clone())))
            .collect_vec();
        transfers.clear();

        receives.extend(receive_deposit);
        receives.extend(receive_transfer);
    }

    // sort by block number first, then by uuid to make the order deterministic
    receives.sort_by_key(|action| {
        let meta = action.meta();
        (meta.block_number, meta.meta.uuid.clone())
    });

    Ok(receives)
}

/// Determine the sequence of withdrawal tx
pub async fn determine_withdrawals<
    S: StoreVaultClientInterface,
    V: ValidityProverClientInterface,
>(
    store_vault_server: &S,
    validity_prover: &V,
    key: KeySet,
    tx_timeout: u64,
) -> Result<
    (
        Vec<(MetaDataWithBlockNumber, TransferData)>,
        Vec<String>, // pending withdrawals
    ),
    StrategyError,
> {
    log::info!("determine_withdrawals");
    let user_data = get_user_data(store_vault_server, key).await?;
    let withdrawal_info = fetch_withdrawal_info(
        store_vault_server,
        validity_prover,
        key,
        &user_data.withdrawal_status,
        tx_timeout,
    )
    .await?;
    let pending_withdrawal_uuids = withdrawal_info
        .pending
        .iter()
        .map(|(meta, _)| meta.uuid.clone())
        .collect();
    Ok((withdrawal_info.settled, pending_withdrawal_uuids))
}

/// Determine the
pub async fn determine_claim<S: StoreVaultClientInterface, V: ValidityProverClientInterface>(
    store_vault_server: &S,
    validity_prover: &V,
    liquidity_contract: &LiquidityContract,
    key: KeySet,
    deposit_timeout: u64,
) -> Result<Option<(MetaDataWithBlockNumber, DepositData)>, StrategyError> {
    log::info!("determine_claims");
    let user_data = get_user_data(store_vault_server, key).await?;
    let current_block_number = validity_prover.get_block_number().await?;

    // get last block number from validity prover
    let update_witness = validity_prover
        .get_update_witness(
            key.pubkey,
            current_block_number,
            current_block_number,
            false,
        )
        .await?;
    let last_block_number = update_witness.account_membership_proof.get_value() as u32;

    // get all deposit info
    let deposit_info = fetch_deposit_info(
        store_vault_server,
        validity_prover,
        liquidity_contract,
        key,
        &ProcessStatus::default(),
        deposit_timeout,
    )
    .await?;

    let current_time = chrono::Utc::now().timestamp() as u64;

    let mut claim_candidates = Vec::new();
    // get only deposits that last block number < deposit block number
    for (meta, deposit_data) in &deposit_info.settled {
        if user_data
            .claim_status
            .processed_uuids
            .contains(&meta.meta.uuid)
        {
            // already processed
            continue;
        }
        if !validate_eligible_deposit_amount(deposit_data.amount) {
            // amount is not eligible for claim
            continue;
        }
        if meta.block_number <= last_block_number {
            // there is a send tx after the deposit, so this is not eligible for claim
            continue;
        }
        let validity_witness = validity_prover
            .get_validity_witness(meta.block_number)
            .await?;
        let deposit_block_timestamp = validity_witness.block_witness.block.timestamp;
        let block_hash = validity_witness.block_witness.block.hash();
        let deposit_salt = deposit_data.deposit_salt;
        let lock_time = get_lock_time(block_hash, deposit_salt);
        if current_time <= deposit_block_timestamp + lock_time {
            // not yet claimable
            continue;
        }
        claim_candidates.push((meta.clone(), deposit_data.clone()));
    }

    // pickup the largest deposit
    let claim_data = claim_candidates
        .into_iter()
        .max_by_key(|(_, deposit_data)| deposit_data.amount);

    Ok(claim_data)
}

async fn get_user_data<S: StoreVaultClientInterface>(
    store_vault_server: &S,
    key: KeySet,
) -> Result<UserData, StrategyError> {
    let user_data = store_vault_server
        .get_user_data(key)
        .await?
        .map(|encrypted| UserData::decrypt(&encrypted, key))
        .transpose()
        .map_err(|e| StrategyError::UserDataDecryptionError(e.to_string()))?
        .unwrap_or(UserData::new(key.pubkey));
    Ok(user_data)
}

fn validate_eligible_deposit_amount(amount: U256) -> bool {
    let amount: BigUint = amount.into();
    let base = BigUint::from(10u32).pow(17); // 0.1 ETH
    if base.clone() % amount.clone() != BigUint::from(0u32) {
        // amount must be a divisor of 0.1 ETH
        return false;
    }
    let mut ratio = base / amount;
    while ratio > BigUint::from(1u32) {
        // If temp is not divisible by 10, ratio is not 10^n
        if ratio.clone() % 10u32 != BigUint::ZERO {
            return false;
        }
        ratio /= 10u32;
    }
    true
}
