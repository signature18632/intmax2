use std::fmt::Display;

use intmax2_interfaces::{
    api::{
        store_vault_server::interface::StoreVaultClientInterface,
        validity_prover::interface::ValidityProverClientInterface,
    },
    data::{
        deposit_data::{DepositData, TokenType},
        meta_data::MetaDataWithBlockNumber,
        user_data::ProcessStatus,
    },
};
use intmax2_zkp::{
    circuits::claim::determine_lock_time::get_lock_time,
    common::{block::Block, signature::key_set::KeySet},
    ethereum_types::u256::U256,
};
use num_bigint::BigUint;

use crate::{
    client::strategy::deposit::fetch_deposit_info,
    external_api::contract::liquidity_contract::LiquidityContract,
};

use super::error::StrategyError;

#[derive(Debug, Clone)]
pub struct Mining {
    pub meta: MetaDataWithBlockNumber,
    pub deposit_data: DepositData,
    pub block: Block,  // the first block that contains the deposit
    pub maturity: u64, // maturity unix timestamp
    pub status: MiningStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MiningStatus {
    Disqualified, // Disqualified because there is a send tx after the deposit
    Locking,      // In locking period
    Claimable,    // Claimable
}

impl Display for MiningStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiningStatus::Disqualified => write!(f, "Disqualified"),
            MiningStatus::Locking => write!(f, "Locking"),
            MiningStatus::Claimable => write!(f, "Claimable"),
        }
    }
}

pub async fn fetch_mining_info<S: StoreVaultClientInterface, V: ValidityProverClientInterface>(
    store_vault_server: &S,
    validity_prover: &V,
    liquidity_contract: &LiquidityContract,
    key: KeySet,
    deposit_timeout: u64,
) -> Result<Vec<Mining>, StrategyError> {
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
    // filter out ineligible deposits
    let candidate_deposits = deposit_info
        .settled
        .into_iter()
        .filter(|(_, deposit_data)| {
            let deposit = deposit_data.deposit().unwrap(); // unwrap is safe here because already settled
            if !deposit.is_eligible {
                // skip ineligible deposits
                return false;
            }
            if !validate_mining_deposit_criteria(deposit_data.token_type, deposit.amount) {
                // skip deposits that do not meet the mining criteria
                return false;
            }
            true
        })
        .collect::<Vec<_>>();
    if candidate_deposits.is_empty() {
        // early return if no eligible deposits
        return Ok(vec![]);
    }

    // fetch last block number
    let account_info = validity_prover.get_account_info(key.pubkey).await?;
    let last_block_number = account_info.last_block_number;

    let mut minings = Vec::new();
    let current_time = chrono::Utc::now().timestamp() as u64;

    for (meta, deposit_data) in candidate_deposits {
        let validity_witness = validity_prover
            .get_validity_witness(meta.block_number)
            .await?;
        let block = validity_witness.block_witness.block;
        let lock_time = get_lock_time(block.hash(), deposit_data.deposit_salt);
        let maturity = block.timestamp + lock_time;
        let status = {
            if block.block_number <= last_block_number {
                // there is a send tx after the deposit
                MiningStatus::Disqualified
            } else if current_time < maturity {
                // in locking period
                MiningStatus::Locking
            } else {
                // claimable
                MiningStatus::Claimable
            }
        };
        minings.push(Mining {
            meta,
            deposit_data,
            block,
            maturity,
            status,
        });
    }
    Ok(minings)
}

pub fn validate_mining_deposit_criteria(token_type: TokenType, amount: U256) -> bool {
    if token_type != TokenType::NATIVE {
        return false;
    }
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
