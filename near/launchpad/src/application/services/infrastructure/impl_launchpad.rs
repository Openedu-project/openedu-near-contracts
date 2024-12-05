use core::num;
use std::hash::RandomState;

use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Balance, Gas, PromiseOrValue};

use crate::models::{
    contract::{Assets, Launchpad, LaunchpadExt, LaunchpadFeature, PoolMetadata, Status, UserTokenDepositRecord}, ft_request::external::cross_edu, PoolId
};


pub const GAS_FOR_CROSS_CALL: Gas = Gas(3_000_000_000_000);
pub const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(300_000_000_000_000);
pub const ATTACHED_TRANSFER_FT: u128 = 1;
pub const ATTACHED_STORAGE_DEPOSIT: u128 = 1_250_000_000_000_000_000_000;

#[near_bindgen]
impl LaunchpadFeature for Launchpad {

    #[payable]
    fn init_pool(&mut self, campaign_id: String, token_id: AccountId, mint_multiple_pledge: u8, time_start_pledge: u64, time_end_pledge: u64) -> PoolMetadata {
        let pool_id = self.all_pool_id.len() as u64 + 1;
        let creator_id = env::signer_account_id();
        let staking_amount = env::attached_deposit();

        if staking_amount <= 1_000_000_000_000_000_000 {
            env::panic_str("Attached deposit must be greater than 1 NEAR.");
        }
        
        let pool = PoolMetadata {
            pool_id,
            campaign_id,
            creator_id,
            staking_amount,
            status: Status::INIT,
            token_id,
            total_balance: 0,
            time_start_pledge,
            time_end_pledge,
            mint_multiple_pledge,
            user_records: Vec::new(),
        };
        self.all_pool_id.insert(&pool_id);
        self.pool_metadata_by_id.insert(&pool_id, &pool);
        pool
    }

    fn start_voting(&mut self, pool_id: PoolId) -> PoolMetadata {
        if let Some(mut pool) = self.pool_metadata_by_id.get(&pool_id) {
        let current_time = env::block_timestamp();
        if current_time > pool.time_start_pledge {
            for user_record in &mut pool.user_records {
                user_record.voting_power = ((user_record.amount as f64 / pool.total_balance as f64) * 100.0);
            }
            pool.status = Status::VOTING;
            self.pool_metadata_by_id.insert(&pool_id, &pool);
            pool
        } else {
            env::panic_str("Voting cannot start before the pledge start time.");
        }
        } else {
            env::panic_str("Pool with the given ID does not exist.");
        }
    }

    fn change_pool_infor(&mut self, pool_id: u64, campaign_id: String, time_start_pledge: u64, time_end_pledge: u64) {
        
        if let Some(mut pool) = self.pool_metadata_by_id.get(&pool_id) {
            if env::signer_account_id() != pool.creator_id {
                env::panic_str("Only the creator of the pool can change its information.");
            }
            pool.campaign_id = campaign_id;
            pool.time_start_pledge = time_start_pledge;
            pool.time_end_pledge = time_end_pledge;
            self.pool_metadata_by_id.insert(&pool_id, &pool);
        } else {
            env::panic_str("Pool with the given ID does not exist.");
        }
    }
    fn refund(&mut self) {}

    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {

        env::log_str(&format!("Received {} tokens from {}", amount.0, sender_id));
        
        let token_id_from_msg = env::predecessor_account_id();

        if !self.list_assets.iter().any(|asset| asset.token_id == token_id_from_msg) {
            env::panic_str("Token ID from message does not match any token ID in the list.");
        }

        PromiseOrValue::Value(U128(0))
    }

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

    fn change_admin(&mut self, new_admin: AccountId) {
       
        if env::signer_account_id() != self.owner_id {
            env::panic_str("Only the current admin can change the admin.");
        }

        self.owner_id = new_admin.clone();
        env::log_str(&format!("Admin changed to: {}", new_admin));
    }

    fn delete_token_by_token_id(
        &mut self,
        token_id: AccountId
    ) {
        if env::signer_account_id() != self.owner_id {
            env::panic_str("Only the admin can delete a token.");
        }

        env::log_str(&format!("Token with ID {} has been deleted.", token_id));
    }
}
