use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::{LookupMap, UnorderedSet, UnorderedMap},
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault,
    PromiseOrValue,
    json_types::U128
};

use super::PoolId;

pub const DEFAULT_MIN_STAKING: u128 = 1_000_000_000_000_000_000_000; // 1 NEAR


#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Launchpad {
    /// Account ID of the owner of the contract.
    pub owner_id: AccountId,  
    pub all_pool_id: UnorderedSet<PoolId>,
    pub list_assets: Vec<Assets>,
    pub pool_metadata_by_id: LookupMap<PoolId, PoolMetadata>,
    pub min_staking_amount: u128,
    pub refund_percent: u8,
    pub user_records: LookupMap<PoolId, UnorderedMap<AccountId, UserTokenDepositRecord>>,
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
    pub target_funding: u128,
    pub time_start_pledge: u64,
    pub time_end_pledge: u64,
    pub mint_multiple_pledge: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Assets {
    pub token_id: AccountId,
    pub balances: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
    INIT,
    FUNDING,
    REJECTED,
    CANCELED,
    FAILED,
    WAITING,
    REFUNDED,
    VOTING,
    CLOSED
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct UserTokenDepositRecord {
    pub amount: u128, // pledge amount if backer deposited +amount
    pub voting_power: f64, // 0
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct UserRecordDetail {
    pub user_id: AccountId,
    pub record: UserTokenDepositRecord,
}

#[derive(BorshSerialize)]
pub enum LaunchpadStorageKey {
    AllPoolId,
    PoolMetadataById,
    UserRecordsMap,
    UserRecordsById { pool_id: PoolId },
}

impl LaunchpadStorageKey {
    pub fn user_records_prefix(pool_id: PoolId) -> Vec<u8> {
        let mut prefix = Vec::with_capacity(4 + 8);
        prefix.extend_from_slice(b"user");
        prefix.extend_from_slice(&pool_id.to_le_bytes());
        prefix
    }
}

pub trait LaunchpadFeature {
    fn init_pool(&mut self, campaign_id: String, token_id: AccountId, mint_multiple_pledge: u8, time_start_pledge: u64, time_end_pledge: u64, target_funding: u128) -> PoolMetadata;

    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
    fn change_pool_funding_time(&mut self, pool_id: u64, campaign_id: String, time_start_pledge: u64, time_end_pledge: u64);
    fn add_token(
        &mut self,
        token_id: String,
    );
    fn change_admin(
        &mut self,
        new_admin: AccountId
    );
    fn delete_token_by_token_id(
        &mut self,
        token_id: AccountId
    );
    fn set_min_staking_amount(&mut self, amount: U128);
    fn set_refund_reject_pool(&mut self, percent: u8);
    fn approve_pool(&mut self, pool_id: PoolId) -> PoolMetadata;
    fn reject_pool(&mut self, pool_id: PoolId) -> PoolMetadata;
    fn cancel_pool(&mut self, pool_id: PoolId) -> PoolMetadata;
    fn withdraw_to_creator(&mut self, pool_id: PoolId, amount: U128);
    fn check_funding_result(&mut self, pool_id: PoolId, is_waiting_funding: bool) -> PoolMetadata;
}


pub trait LaunchpadGet {
    fn get_min_staking_amount(&self) -> U128;
    fn get_refund_reject_pool(&self) -> u8;
    fn get_all_pool(&self) -> Option<Vec<PoolMetadata>>;
    fn get_pool_by_pool_id(&self, pool_id: PoolId) -> Option<Vec<PoolMetadata>>;
    fn get_all_users_power_by_pool_id(&self, pool_id: PoolId) -> Option<Vec<UserTokenDepositRecord>>;
    fn get_all_funding_pools(&self) -> Option<Vec<PoolMetadata>>;
    fn get_all_init_pools(&self) -> Option<Vec<PoolMetadata>>;
    fn get_all_closed_pools(&self) -> Option<Vec<PoolMetadata>>;
    fn get_all_waiting_pools(&self) -> Option<Vec<PoolMetadata>>;
    fn get_detail_pool(&self, pool_id: PoolId) -> Option<PoolMetadata>;
    fn get_balance_creator(&self, pool_id: PoolId) -> Option<u128>;
    fn get_user_records_by_pool_id(&self, pool_id: PoolId) -> Option<Vec<UserRecordDetail>>;
}