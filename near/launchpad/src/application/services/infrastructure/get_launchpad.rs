use near_sdk::{near_bindgen, AccountId, json_types::U128};

use crate::models::{
    contract::{Launchpad, LaunchpadGet, LaunchpadExt, PoolMetadata, Status, UserTokenDepositRecord, UserRecordDetail}, 
    PoolId
};

#[near_bindgen]
impl LaunchpadGet for Launchpad {

    /* //////////////////////////////////////////////////////////////
                            GETTER FUNCTIONS
    ////////////////////////////////////////////////////////////// */

    fn is_token_supported(&self, token_id: AccountId) -> bool {
        self.list_assets.iter().any(|asset| asset.token_id == token_id)
    }

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
    
    fn get_pools_by_status(&self, status_str: String) -> Option<Vec<PoolMetadata>> {
        if self.all_pool_id.is_empty() {
            return None;
        }

        let status = match status_str.as_str() {
            "FUNDING" => Status::FUNDING,
            "INIT" => Status::INIT,
            "CLOSED" => Status::CLOSED,
            "WAITING" => Status::WAITING,
            "REJECTED" => Status::REJECTED,
            "CANCELED" => Status::CANCELED,
            "FAILED" => Status::FAILED,
            "REFUNDED" => Status::REFUNDED,
            "VOTING" => Status::VOTING,
            "SUCCESSFUL" => Status::SUCCESSFUL,
            _ => return None,
        };

        let pools: Vec<PoolMetadata> = self.all_pool_id.iter()
            .filter_map(|pool_id| {
                let pool = self.pool_metadata_by_id.get(&pool_id)?;
                if pool.status == status {
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

    fn get_user_records_by_pool_id(&self, pool_id: PoolId) -> Option<Vec<UserRecordDetail>> {
        if let Some(user_records) = self.user_records.get(&pool_id) {
            let records: Vec<UserRecordDetail> = user_records
                .iter()
                .map(|(user_id, record)| UserRecordDetail {
                    user_id,
                    record: record.clone(),
                })
                .collect();

            if records.is_empty() {
                None
            } else {
                Some(records)
            }
        } else {
            None
        }
    }
}