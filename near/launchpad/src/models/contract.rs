use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::{LookupMap, UnorderedSet},
    json_types::Base64VecU8,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault,
    PromiseOrValue,
    json_types::U128
};

use super::PoolId;

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Launchpad {
    /// Account ID of the owner of the contract.
    pub owner_id: AccountId,  
    pub all_pool_id: UnorderedSet<PoolId>,
    pub pool_metadata_by_id: LookupMap<PoolId, PoolMetadata>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PoolMetadata {
    pub pool_id: PoolId,
    pub campaign_id: String,
    pub creator_id: AccountId,
    pub staking_amount: u128,
    pub status: Status,
    pub token_id: AccountId,
    pub total_balance: u128,
    pub mint_multiple_pledge: u8,
    pub user_records: Vec<UserTokenDepositRecord>
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
    INIT,
    FUNDING,
    WAIT,
    VOTING,
    CLOSED,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct UserTokenDepositRecord {
    pub user_id: AccountId,
    pub amount: u128,
    pub voting_power: u8,
}

#[derive(BorshSerialize)]
pub enum LaunchpadStorageKey {
    AllPoolId,
    PoolMetadataById
}