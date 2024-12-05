use core::num;
use std::hash::RandomState;

use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Balance, Gas, PromiseOrValue};

use crate::models::{
    contract::{Assets, Launchpad, LaunchpadEnum, LaunchpadExt, PoolMetadata, UserTokenDepositRecord}, PoolId
};

#[near_bindgen]
impl LaunchpadEnum for Launchpad {
    fn get_all_pool(&self) -> Option<Vec<PoolMetadata>> {
        if self.all_pool_id.is_empty() {
            return None;
        }
        let pools: Vec<PoolMetadata> = self.all_pool_id.iter()
            .filter_map(|pool_id| self.pool_metadata_by_id.get(&pool_id))
            .collect();
        if pools.is_empty() {
            None
        } else {
            Some(pools)
        }
    }
    
    fn get_pool_by_pool_id(&self, pool_id: PoolId) -> Option<Vec<AccountId>> {
        self.pool_metadata_by_id.get(&pool_id).map(|pool| {
            pool.user_records.iter().map(|record| record.user_id.clone()).collect()
        })
    }

    fn get_all_users_power_by_pool_id(&self, pool_id: PoolId) -> Option<Vec<UserTokenDepositRecord>> {
        self.pool_metadata_by_id.get(&pool_id).map(|pool| {
            pool.user_records.clone()
        })
    }
}