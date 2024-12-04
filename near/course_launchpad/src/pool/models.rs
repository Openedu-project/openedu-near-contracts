use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, NearToken};
use std::collections::HashMap;
use crate::pool::enums::PoolStatus;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Pool {
    pub pool_id: u64,
    pub campaign_id: String,
    pub creator_id: AccountId,
    pub staking_amount: NearToken,
    pub status: PoolStatus,
    pub total_balance: NearToken,
    pub min_deposit: NearToken,
    pub min_staking: NearToken,
    pub user_records: HashMap<AccountId, UserRecord>,
    pub created_at: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
pub struct UserRecord {
    pub balance: NearToken,
    pub power: u32,
}