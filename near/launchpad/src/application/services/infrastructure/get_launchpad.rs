use near_sdk::{near_bindgen, AccountId, json_types::U128};

use crate::models::{
    contract::{Launchpad, LaunchpadGet, LaunchpadExt, PoolMetadata, Status, UserTokenDepositRecord}, 
    PoolId
};

#[near_bindgen]
impl LaunchpadGet for Launchpad {

    /* //////////////////////////////////////////////////////////////
                            GETTER FUNCTIONS
    ////////////////////////////////////////////////////////////// */

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
    
    fn get_pool_by_pool_id(&self, pool_id: PoolId) -> Option<Vec<PoolMetadata>> {
        self.pool_metadata_by_id.get(&pool_id).map(|pool| vec![pool])
    }

    fn get_all_users_power_by_pool_id(&self, pool_id: PoolId) -> Option<Vec<UserTokenDepositRecord>> {
        self.user_records.get(&pool_id).map(|user_records| {
            user_records.iter()
                .map(|(_, record)| record.clone()) // Clone giá trị
                .collect()
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

    // get refund percentage for rejected pools right now
    fn get_refund_reject_pool(&self) -> u8 {
        self.refund_percent
    }

     // get min staking amount right now
     fn get_min_staking_amount(&self) -> U128 {
        U128(self.min_staking_amount)
    }
}