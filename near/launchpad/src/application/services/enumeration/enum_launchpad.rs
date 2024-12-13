use near_sdk::{near_bindgen, AccountId};

use crate::models::{
    contract::{Launchpad, LaunchpadEnum, LaunchpadExt, PoolMetadata, Status, UserTokenDepositRecord}, 
    PoolId
};

#[near_bindgen]

// todo: LaunchpadGet (enum tưởng constant hardcode) 
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

    fn get_all_funding_pools(&self) -> Option<Vec<PoolMetadata>> {
        if self.all_pool_id.is_empty() {
            return None;
        }
        let pools: Vec<PoolMetadata> = self.all_pool_id.iter()
            .filter_map(|pool_id| {
                let pool = self.pool_metadata_by_id.get(&pool_id)?;
                if matches!(pool.status, Status::FUNDING) {
                    Some(pool)
                } else {
                    None
                }
            })
            .collect();
        
        if pools.is_empty() {
            None
        } else {
            Some(pools)
        }
    }

    fn get_all_init_pools(&self) -> Option<Vec<PoolMetadata>> {
        if self.all_pool_id.is_empty() {
            return None;
        }
        
        let pools: Vec<PoolMetadata> = self.all_pool_id.iter()
            .filter_map(|pool_id| {
                let pool = self.pool_metadata_by_id.get(&pool_id)?;
                if matches!(pool.status, Status::INIT) {
                    Some(pool)
                } else {
                    None
                }
            })
            .collect();
        
        if pools.is_empty() {
            None
        } else {
            Some(pools)
        }
    }

    fn get_all_closed_pools(&self) -> Option<Vec<PoolMetadata>> {
        if self.all_pool_id.is_empty() {
            return None;
        }
        
        let pools: Vec<PoolMetadata> = self.all_pool_id.iter()
            .filter_map(|pool_id| {
                let pool = self.pool_metadata_by_id.get(&pool_id)?;
                if matches!(pool.status, Status::CLOSED) {
                    Some(pool)
                } else {
                    None
                }
            })
            .collect();
        
        if pools.is_empty() {
            None
        } else {
            Some(pools)
        }
    }

    fn get_all_waiting_pools(&self) -> Option<Vec<PoolMetadata>> {
        if self.all_pool_id.is_empty() {
            return None;
        }
        
        let pools: Vec<PoolMetadata> = self.all_pool_id.iter()
            .filter_map(|pool_id| {
                let pool = self.pool_metadata_by_id.get(&pool_id)?;
                if matches!(pool.status, Status::WAITING) {
                    Some(pool)
                } else {
                    None
                }
            })
            .collect();
        
        if pools.is_empty() {
            None
        } else {
            Some(pools)
        }
    }

    fn get_detail_pool(&self, pool_id: PoolId) -> Option<PoolMetadata> {
        self.pool_metadata_by_id.get(&pool_id)
    }

    fn get_balance_creator(&self, pool_id: PoolId) -> Option<u128> {
        self.pool_metadata_by_id.get(&pool_id).map(|pool| {
            pool.total_balance
        })
    }
}