use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::{LookupMap, UnorderedSet},
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
    pub time_start_pledge: u64,
    pub time_end_pledge: u64,
    pub mint_multiple_pledge: u8,
    pub user_records: Vec<UserTokenDepositRecord>
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Assets {
    pub token_id: AccountId,
    pub balances: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
    INIT,
    FUNDING,
    WAITING,
    VOTING,
    CLOSED,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct UserTokenDepositRecord {
    pub user_id: AccountId, // backer
    pub amount: u128, // pledge amount if backer deposited +amount
    pub voting_power: f64, // 0
}

#[derive(BorshSerialize)]
pub enum LaunchpadStorageKey {
    AllPoolId,
    PoolMetadataById
}

pub trait LaunchpadFeature {
    fn init_pool(&mut self, campaign_id: String, token_id: AccountId, mint_multiple_pledge: u8, time_start_pledge: u64, time_end_pledge: u64) -> PoolMetadata;

    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;

    fn start_voting(&mut self, pool_id: PoolId) -> PoolMetadata;

    fn change_pool_infor(&mut self, pool_id: u64, campaign_id: String, time_start_pledge: u64, time_end_pledge: u64);
    fn refund(&mut self);


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
    fn get_min_staking_amount(&self) -> U128;

    fn set_refund_reject_pool(&mut self, percent: u8);
    fn get_refund_reject_pool(&self) -> u8;

    fn approve_pool(&mut self, pool_id: PoolId) -> PoolMetadata;
    fn reject_pool(&mut self, pool_id: PoolId) -> PoolMetadata;
}

pub trait LaunchpadEnum {
    fn get_all_pool(&self) -> Option<Vec<PoolMetadata>>;
    fn get_pool_by_pool_id(&self, pool_id: PoolId) -> Option<Vec<AccountId>>;
    fn get_all_users_power_by_pool_id(&self, pool_id: PoolId) -> Option<Vec<UserTokenDepositRecord>>;
    fn get_all_funding_pools(&self) -> Option<Vec<PoolMetadata>>;
    fn get_all_init_pools(&self) -> Option<Vec<PoolMetadata>>;
    fn get_all_closed_pools(&self) -> Option<Vec<PoolMetadata>>;
    fn get_all_waiting_pools(&self) -> Option<Vec<PoolMetadata>>;
    fn get_detail_pool(&self, pool_id: PoolId) -> Option<PoolMetadata>;
    fn get_balance_creator(&self, pool_id: PoolId) -> Option<u128>;
}