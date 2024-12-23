use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Gas, PromiseOrValue, Promise};

use crate::models::{
    contract::{
        Assets, Launchpad, LaunchpadExt, LaunchpadFeature, 
        PoolMetadata, Status, UserTokenDepositRecord, 
        DEFAULT_MIN_STAKING, LaunchpadStorageKey
    }, 
    ft_request::external::cross_edu, 
    PoolId
};
use near_sdk::collections::{UnorderedMap};

pub const GAS_FOR_CROSS_CALL: Gas = Gas(3_000_000_000_000);
pub const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(300_000_000_000_000);
pub const ATTACHED_TRANSFER_FT: u128 = 1;
pub const ATTACHED_STORAGE_DEPOSIT: u128 = 1_250_000_000_000_000_000_000;
pub const GAS_FOR_REFUND_CALLBACK: Gas = Gas(5_000_000_000_000);

#[near_bindgen]
impl LaunchpadFeature for Launchpad {

    /* //////////////////////////////////////////////////////////////
                            ADMIN FUNCTIONS
    ////////////////////////////////////////////////////////////// */
    fn change_pool_funding_time(&mut self, pool_id: u64, time_start_pledge: u64, time_end_pledge: u64) {
        let signer_id = env::signer_account_id();
        
        if signer_id != self.owner_id {
            env::panic_str("Only admin can change pool information.");
        }

        if let Some(mut pool) = self.pool_metadata_by_id.get(&pool_id) {
            if pool.status != Status::INIT {
                env::panic_str("Pool status must be INIT to change funding time.");
            }
            
            if time_start_pledge >= time_end_pledge {
                env::panic_str("End time must be after start time");
            }

            if time_start_pledge <= env::block_timestamp() {
                env::panic_str("Start time must be in the future");
            }

            pool.time_start_pledge = time_start_pledge;
            pool.time_end_pledge = time_end_pledge;
            
            self.pool_metadata_by_id.insert(&pool_id, &pool);

            env::log_str(&format!(
                "Pool {} information updated by admin {}",
                pool_id,
                signer_id
            ));
        } else {
            env::panic_str("Pool with the given ID does not exist.");
        }
    }

    // admin can add list token use payable
    fn add_token(
        &mut self,
        token_id: String,
    ) {

        if env::signer_account_id() != self.owner_id {
            env::panic_str("Only the admin can add a new token.");
        }

        if self.list_assets.iter().any(|asset| asset.token_id == AccountId::new_unchecked(token_id.clone())) {
            env::log_str("Token already exists in the list.");
            return;
        }

        let ft_addr = AccountId::new_unchecked(token_id.clone());
        cross_edu::ext(ft_addr.to_owned())
            .with_static_gas(GAS_FOR_CROSS_CALL)
            .with_attached_deposit(ATTACHED_STORAGE_DEPOSIT)
            .storage_deposit(env::current_account_id());

        self.list_assets.push(Assets {
            token_id: AccountId::new_unchecked(token_id),
            balances: 0,
        });
    }

    // admin can change admin contract launchpad
    fn change_admin(&mut self, new_admin: AccountId) {
       
        if env::signer_account_id() != self.owner_id {
            env::panic_str("Only the current admin can change the admin.");
        }

        self.owner_id = new_admin .clone();
        env::log_str(&format!("Admin changed to: {}", new_admin));
    }

    // admin can delete a token payable
    fn delete_token_by_token_id(
        &mut self,
        token_id: AccountId
    ) {
        if env::signer_account_id() != self.owner_id {
            env::panic_str("Only the admin can delete a token.");
        }

        env::log_str(&format!("Token with ID {} has been deleted.", token_id));
    }

    // admin can change the refund percentage for rejected pools
    fn set_refund_reject_pool(&mut self, percent: u8) {
        if env::signer_account_id() != self.owner_id {
            env::panic_str("Only admin can set refund percentage");
        }

        if percent > 100 {
            env::panic_str("Refund percentage must be between 0 and 100");
        }

        self.refund_percent = percent;

        env::log_str(&format!(
            "Refund percentage for rejected pools set to {}%",
            percent
        ));
    }

    // admin can set pool status to FUNDING or CLOSED
    fn admin_set_status_pool_pre_funding(&mut self, pool_id: PoolId, approve: bool) -> PoolMetadata {
        if env::signer_account_id() != self.owner_id {
            env::panic_str("Only admin can set pool status");
        }

        let mut pool = self.pool_metadata_by_id.get(&pool_id)
            .expect("Pool does not exist");

        if !matches!(pool.status, Status::INIT) {
            env::panic_str("Pool must be in INIT status to change status");
        }

        if approve {
            pool.status = Status::FUNDING;
            env::log_str(&format!(
                "Pool {} has been approved and is now in FUNDING status",
                pool_id
            ));
        } else {
            let refund_amount = if self.refund_percent == 0 {
                1_000_000_000_000_000_000_000
            } else {
                (pool.staking_amount * self.refund_percent as u128) / 100
            };

            Promise::new(pool.creator_id.clone())
                .transfer(refund_amount);

            pool.status = Status::REJECTED;
            pool.staking_amount = 0;  
            
            env::log_str(&format!(
                "Pool {} has been rejected. {}% of deposit ({} yoctoNEAR) returned to creator {}",
                pool_id,
                self.refund_percent,
                refund_amount,
                pool.creator_id
            ));
        }

        self.pool_metadata_by_id.insert(&pool_id, &pool);

        pool
    }

    // admin can set min staking amount
    fn set_min_staking_amount(&mut self, amount: U128) {
        // Only admin can set minimum staking amount
        if env::signer_account_id() != self.owner_id {
            env::panic_str("Only admin can set minimum staking amount");
        }

        // Ensure minimum amount is at least 1 NEAR
        if amount.0 < DEFAULT_MIN_STAKING {
            env::panic_str("Minimum staking cannot be less than 1 NEAR");
        }

        self.min_staking_amount = amount.0;

        env::log_str(&format!(
            "Minimum staking amount set to {} yoctoNEAR ({} NEAR)",
            amount.0,
            amount.0 / DEFAULT_MIN_STAKING
        ));
    }
    
    fn withdraw_to_creator(&mut self, pool_id: PoolId, amount: U128) {
        let signer_id = env::signer_account_id();

        if signer_id != self.owner_id {
            env::panic_str("Only admin can withdraw funds to the creator.");
        }

        let mut pool = self.pool_metadata_by_id.get(&pool_id)
            .expect("Pool does not exist");

        if pool.status != Status::CLOSED {
            env::panic_str("Pool must be CLOSED to withdraw funds.");
        }

        if amount.0 > pool.total_balance {
            env::panic_str("Insufficient pool balance for the requested withdrawal amount.");
        }

        pool.total_balance -= amount.0;

        cross_edu::ext(pool.token_id.clone())
            .with_static_gas(GAS_FOR_FT_TRANSFER_CALL)
            .with_attached_deposit(1)
            .ft_transfer(
                pool.creator_id.clone(),
                amount,
            );

        env::log_str(&format!(
            "Withdrawn {} tokens to creator {}. Remaining pool balance: {}",
            amount.0,
            pool.creator_id,
            pool.total_balance
        ));

        self.pool_metadata_by_id.insert(&pool_id, &pool);
    }

    fn update_pool_status(&mut self, pool_id: PoolId, status: String) {
        let signer_id = env::signer_account_id();

        if signer_id != self.owner_id {
            env::panic_str("Only the owner can update the pool status.");
        }

        let mut pool = self.pool_metadata_by_id.get(&pool_id)
            .expect("Pool does not exist");

        let new_status = match status.as_str() {
            "INIT" => Status::INIT,
            "FUNDING" => Status::FUNDING,
            "REJECTED" => Status::REJECTED,
            "CANCELED" => Status::CANCELED,
            "FAILED" => Status::FAILED,
            "WAITING" => Status::WAITING,
            "REFUNDED" => Status::REFUNDED,
            "VOTING" => Status::VOTING,
            "CLOSED" => Status::CLOSED,
            "SUCCESSFUL" => Status::SUCCESSFUL,
            _ => env::panic_str("Invalid status provided."),
        };

        pool.status = new_status;

        self.pool_metadata_by_id.insert(&pool_id, &pool);

        env::log_str(&format!(
            "Pool {} status updated to {} by owner {}",
            pool_id,
            status,
            signer_id
        ));
    }

    fn check_funding_result(&mut self, pool_id: PoolId, is_waiting_funding: bool) -> PoolMetadata {
        let mut pool = self.pool_metadata_by_id.get(&pool_id)
            .expect("Pool does not exist");

        if env::signer_account_id() != self.owner_id {
            env::panic_str("Only the owner can call this function");
        }

        if pool.status != Status::FUNDING {
            env::panic_str("Pool is not in FUNDING status");
        }

        let current_time = env::block_timestamp();
        if current_time <= pool.time_end_pledge {
            env::panic_str("Funding period has not ended yet");
        }

        // calculating voting_power to backer
        if let Some(mut user_records) = self.user_records.get(&pool_id) {
            let mut updated_records = Vec::new();
            for (user_id, record) in user_records.iter() {
                let mut updated_record = record.clone();
                updated_record.voting_power = ((updated_record.amount as f64) / (pool.total_balance as f64)) * 100.0;
                updated_records.push((user_id, updated_record));
            }
            for (user_id, updated_record) in updated_records {
                user_records.insert(&user_id, &updated_record);
            }
            self.user_records.insert(&pool_id, &user_records);
        }

        match pool.total_balance {
            0 => {
                pool.status = Status::FAILED;
                env::log_str(&format!(
                    "Pool {} status changed to FAILED due to zero total balance",
                    pool_id
                ));
            },
            _ if pool.total_balance >= pool.target_funding => {
                pool.status = Status::VOTING;
                env::log_str(&format!(
                    "Pool {} status changed to VOTING due to reaching target funding",
                    pool_id
                ));
            },
            _ if is_waiting_funding => {
                pool.status = Status::WAITING;
                pool.time_end_pledge += 3 * 24 * 60 * 60 * 1_000_000_000; // Add 3 days in nanoseconds
                env::log_str(&format!(
                    "Pool {} status changed to WAITING",
                    pool_id
                ));
            },
            _ => {
                pool.status = Status::REFUNDED;
                env::log_str(&format!(
                    "Pool {} status changed to CLOSED",
                    pool_id
                ));
            }
        }

        self.pool_metadata_by_id.insert(&pool_id, &pool);

        pool
    }

    /* //////////////////////////////////////////////////////////////
                            CREATOR FUNCTIONS
    ////////////////////////////////////////////////////////////// */
    #[payable]
    fn init_pool(&mut self, campaign_id: String, token_id: AccountId, min_multiple_pledge: u128, time_start_pledge: u64, time_end_pledge: u64, target_funding: U128) -> PoolMetadata {
        let pool_id = self.all_pool_id.len() as u64 + 1;
        let creator_id = env::signer_account_id();
        let staking_amount = env::attached_deposit();

        if staking_amount < self.min_staking_amount {
            env::panic_str(&format!(
                "Attached deposit must be at least {} yoctoNEAR ({} NEAR)",
                self.min_staking_amount,
                self.min_staking_amount / DEFAULT_MIN_STAKING
            ));
        }

        // Check if the token is in the allowed list
        if !self.list_assets.iter().any(|asset| asset.token_id == token_id) {
            env::panic_str(&format!(
                "Token {} is not supported. Only tokens added by admin can be used for pools",
                token_id
            ));
        }

        if time_start_pledge >= time_end_pledge {
            env::panic_str("End time must be after start time");
        }

        if time_start_pledge <= env::block_timestamp() {
            env::panic_str("Start time must be in the future");
        }
        
        let pool = PoolMetadata {
            pool_id,
            campaign_id,
            creator_id: creator_id.clone(),
            staking_amount,
            status: Status::INIT,
            token_id: token_id.clone(),
            total_balance: 0,
            target_funding: target_funding.0,
            time_start_pledge,
            time_end_pledge,
            min_multiple_pledge,
        };

        self.all_pool_id.insert(&pool_id);
        self.pool_metadata_by_id.insert(&pool_id, &pool);

        env::log_str(&serde_json::json!({
            "pool_id": pool_id,
            "creator_id": creator_id,
            "token_id": token_id,
            "staking_amount": staking_amount
        }).to_string());

        pool
    }

    // creator pool should be cancel pool
    // todo: remove refund_percent
    fn cancel_pool(&mut self, pool_id: PoolId) -> PoolMetadata {
        let mut pool = self.pool_metadata_by_id.get(&pool_id)
            .expect("Pool does not exist");

        if env::signer_account_id() != pool.creator_id {
            env::panic_str("Only the creator of the pool can cancel it.");
        }

        if !matches!(pool.status, Status::INIT) {
            env::panic_str("Pool must be in INIT status to be rejected");
        }

        let refund_amount = if self.refund_percent == 0 {
            1_000_000_000_000_000_000_000
        } else {
            (pool.staking_amount * self.refund_percent as u128) / 100
        };

        Promise::new(pool.creator_id.clone())
            .transfer(refund_amount);

        pool.status = Status::CANCELED;
        pool.staking_amount = 0;  
        
        self.pool_metadata_by_id.insert(&pool_id, &pool);

        env::log_str(&format!(
            "Pool {} has been canceled by creator. {}% of deposit ({} yoctoNEAR) returned to creator {}",
            pool_id,
            self.refund_percent,
            refund_amount,
            pool.creator_id
        ));

        pool
    }

    // todo: creator_accept_voting
    // todo: creator mới có thể gọi vào function này
    fn creator_set_status_pool_after_wating(&mut self, pool_id: PoolId, approve: bool) -> PoolMetadata {
        let signer_id = env::signer_account_id();

        if signer_id != self.owner_id {
            env::panic_str("Only the owner can set the pool status after waiting.");
        }

        let mut pool = self.pool_metadata_by_id.get(&pool_id)
            .expect("Pool does not exist");

        if pool.status != Status::WAITING {
            env::panic_str("Pool status must be WAITING to change it after waiting period.");
        }

        if approve {
            pool.status = Status::VOTING;
            env::log_str(&format!(
                "Pool {} status changed to VOTING by owner {}",
                pool_id,
                signer_id
            ));
        } else {
            pool.status = Status::REFUNDED;
            env::log_str(&format!(
                "Pool {} status changed to REFUNDED by owner {}",
                pool_id,
                signer_id
            ));
        }

        self.pool_metadata_by_id.insert(&pool_id, &pool);

        pool
    }

    /* //////////////////////////////////////////////////////////////
                            USER FUNCTIONS
    ////////////////////////////////////////////////////////////// */
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        env::log_str(&format!("Received {} tokens from {}", amount.0, sender_id));
        
        let token_id = env::predecessor_account_id();

        if !self.list_assets.iter().any(|asset| asset.token_id == token_id) {
            env::panic_str("Token ID from message does not match any token ID in the list.");
        }

        let pool_id: PoolId = msg.parse()
            .expect("Invalid pool ID in message");

        let mut pool = self.pool_metadata_by_id.get(&pool_id)
            .expect("Pool does not exist");

        if !matches!(pool.status, Status::FUNDING) {
            env::panic_str("Pool is not in funding status");
        }

        let current_time = env::block_timestamp();
        if current_time < pool.time_start_pledge || current_time > pool.time_end_pledge {
            env::panic_str("Not within pledge period");
        }

        if token_id != pool.token_id {
            env::panic_str("Invalid token for this pool");
        }

        let amount_value = amount.0;
        let mut user_records = if let Some(records) = self.user_records.get(&pool_id) {
            records
        } else {
            let prefix = LaunchpadStorageKey::user_records_prefix(pool_id);
            let map = UnorderedMap::new(prefix);
            self.user_records.insert(&pool_id, &map);
            map
        };

        let mut user_record = if let Some(record) = user_records.get(&sender_id) {
            record
        } else {
            UserTokenDepositRecord {
                amount: 0,
                voting_power: 0.0,
            }
        };

        user_record.amount += amount_value;
        user_records.insert(&sender_id, &user_record);
        self.user_records.insert(&pool_id, &user_records);
        
        pool.total_balance += amount_value;
        self.pool_metadata_by_id.insert(&pool_id, &pool);
        
        env::log_str(&format!(
            "User {} pledged {} tokens to pool {}",
            sender_id, amount_value, pool_id
        ));

        PromiseOrValue::Value(U128(0))
    }

    // todo: claim_refund
    fn withdraw_fund_by_backer(&mut self, pool_id: PoolId) {
        let caller_id = env::signer_account_id();

        let mut pool = self.pool_metadata_by_id.get(&pool_id)
            .expect("Pool does not exist");

        if pool.status != Status::REFUNDED {
            env::panic_str("Pool is not in REFUNDED status");
        }

        let mut user_records = self.user_records.get(&pool_id)
            .expect("No user records found for this pool");

        let mut user_record = user_records.get(&caller_id)
            .expect("User has no record in this pool")
            .clone();

        let refund_amount = (pool.total_balance as f64 * user_record.voting_power / 100.0) as u128;

        if refund_amount == 0 {
            env::panic_str("No funds available for withdrawal");
        }

        // Update the user's record amount to 0
        user_record.amount = 0;
        user_records.insert(&caller_id, &user_record);
        self.user_records.insert(&pool_id, &user_records);

        Promise::new(caller_id.clone()).transfer(refund_amount);

        env::log_str(&format!(
            "User {} withdrew {} yoctoNEAR from pool {}",
            caller_id, refund_amount, pool_id
        ));
    }

}
