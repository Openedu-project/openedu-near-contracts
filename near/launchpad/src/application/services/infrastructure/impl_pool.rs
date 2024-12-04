use core::num;
use std::hash::RandomState;

use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Balance, Gas, PromiseOrValue};

use crate::models::{
    contract::{Launchpad, LaunchpadExt, LaunchpadFeature, Assets, UserTokenDepositRecord}, ft_request::external::cross_edu
};


pub const GAS_FOR_CROSS_CALL: Gas = Gas(3_000_000_000_000);
pub const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(300_000_000_000_000);
pub const ATTACHED_TRANSFER_FT: u128 = 1;
pub const ATTACHED_STORAGE_DEPOSIT: u128 = 1_250_000_000_000_000_000_000;

#[near_bindgen]
impl LaunchpadFeature for Launchpad {
    fn init_pool(&mut self) -> PoolMetadata {
        let pool = PoolMetadata {
            
        };
    } // TODO

    fn start_voting(&mut self) {}
    fn change_pool_infor(&mut self) {}
    fn refund(&mut self) {}


    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {

        env::log_str(&format!("Received {} tokens from {}", amount.0, sender_id));
        
        let token_id_from_msg = env::predecessor_account_id();

        if !self.list_assets.iter().any(|asset| asset.token_id == token_id_from_msg.clone()) {
            env::panic_str("Token ID from message does not match any token ID in the list.");
        }

        PromiseOrValue::Value(U128(0))

    } // TODO

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
            token_id: AccountId::new_unchecked
            (token_id.clone()),
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

        // Log the deletion of the token
        env::log_str(&format!("Token with ID {} has been deleted.", token_id));
    } // TODO
}
