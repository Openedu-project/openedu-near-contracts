use models::contract::{Launchpad, LaunchpadStorageKey, LaunchpadExt, DEFAULT_MIN_STAKING};
use near_sdk::borsh::BorshSerialize;
use near_sdk::{
    collections::{LookupMap, UnorderedSet},
    env, near_bindgen, AccountId,
};

pub mod application;
pub mod models;

#[near_bindgen]
impl Launchpad {
    #[init]
    pub fn init() -> Self {
        let owner_id = env::signer_account_id();

        Self::new(owner_id)
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            all_pool_id: UnorderedSet::new(LaunchpadStorageKey::AllPoolId.try_to_vec().unwrap()),
            list_assets: Vec::new(),
            pool_metadata_by_id: LookupMap::new(LaunchpadStorageKey::PoolMetadataById.try_to_vec().unwrap()),
            min_staking_amount: DEFAULT_MIN_STAKING,
            refund_percent: 0,
            user_records: LookupMap::new(LaunchpadStorageKey::UserRecordsMap.try_to_vec().unwrap()),
        }
    }
}