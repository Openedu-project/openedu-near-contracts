use core::num;
use std::hash::RandomState;

use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Balance, Gas, PromiseOrValue};

use crate::models::{
    contract::{Treasury, TreasuryExt, TreasuryFeature, Assets, UserTokenDepositRecord, TokenDeposit, PaymentInfo}, ft_request::external::cross_edu
};


pub const GAS_FOR_CROSS_CALL: Gas = Gas(3_000_000_000_000);
pub const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(300_000_000_000_000);
pub const ATTACHED_TRANSFER_FT: u128 = 1;
pub const ATTACHED_STORAGE_DEPOSIT: u128 = 1_250_000_000_000_000_000_000;

#[near_bindgen]
impl TreasuryFeature for Treasury {
   
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

        let payment_info: Vec<PaymentInfo> = near_sdk::serde_json::from_str(&msg)
            .expect("Invalid message format");

        for user_info in payment_info {
            let user = self.records_user_by_id.get(&user_info.user_id);

            if user.is_none() {
                let user_record = UserTokenDepositRecord {
                    user_id: user_info.user_id.clone(),
                    deposits: vec![TokenDeposit {
                        token_id: token_id_from_msg.clone(),
                        amount: user_info.amount,
                    }],
                };


                self.records_user_by_id.insert(&user_info.user_id, &user_record);
            } else {
                let mut user = user.unwrap();
                
                if let Some(deposit) = user.deposits.iter_mut().find(|d| d.token_id == token_id_from_msg) {
                    deposit.amount += user_info.amount;
                } else {
                    user.deposits.push(TokenDeposit {
                        token_id: token_id_from_msg.clone(),
                        amount: user_info.amount,
                    });
                }
                self.records_user_by_id.insert(&user_info.user_id, &user);
            }
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
            token_id: AccountId::new_unchecked
            (token_id.clone()),
            balances: 0,
        });
    }


    fn claim(
        &mut self,
        token_id: String,
    ) {
        let signer_id = env::signer_account_id();
        let token_id_account = AccountId::new_unchecked(token_id.clone());

        if let Some(mut user_record) = self.records_user_by_id.get(&signer_id.clone()) {
            if let Some(deposit) = user_record.deposits.iter_mut().find(|d| d.token_id == token_id_account) {
                if deposit.amount > 0 {
                    // User has assets with the specified token_id, allow withdrawal
                    cross_edu::ext(token_id_account.to_owned())
                    .with_static_gas(GAS_FOR_CROSS_CALL)
                    .with_attached_deposit(ATTACHED_STORAGE_DEPOSIT)
                    .storage_deposit(signer_id.clone())
                    .then(
                        cross_edu::ext(token_id_account.to_owned())
                        .with_static_gas(GAS_FOR_CROSS_CALL)
                        .with_attached_deposit(ATTACHED_TRANSFER_FT)
                        .ft_transfer(signer_id.clone(), U128(deposit.amount)));

                    let withdrawn_amount = deposit.amount;
                    deposit.amount = 0; // Reset the amount to zero after withdrawal
                    env::log_str(&format!("{}", withdrawn_amount));

                    // Update the user record in the storage
                    self.records_user_by_id.insert(&signer_id, &user_record);
                } else {
                    env::panic_str("User does not have any assets with the specified token_id.");
                }
            } else {
                env::panic_str("User does not have any deposits with the specified token_id.");
            }
        } else {
            env::panic_str("User record not found.");
        }
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

        // Remove the token from all user records
        for user_id in self.all_user_id.iter() {
            if let Some(mut user_record) = self.records_user_by_id.get(&user_id) {
                user_record.deposits.retain(|deposit| deposit.token_id != token_id);
                self.records_user_by_id.insert(&user_id, &user_record);
            }
        }

        // Log the deletion of the token
        env::log_str(&format!("Token with ID {} has been deleted.", token_id));
    }
}
