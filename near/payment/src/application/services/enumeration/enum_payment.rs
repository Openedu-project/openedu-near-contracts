use near_sdk::{env, near_bindgen, AccountId};

use crate::models::{
    contract::{Payment, PaymentExt, PaymentEnum, UserTokenDepositRecord},
};

#[near_bindgen]
impl PaymentEnum for Payment {
    fn get_user_info_by_id(&self, user_id: AccountId) -> Option<UserTokenDepositRecord> {
        self.records_user_by_id.get(&user_id)
    }

    fn get_all_token_id(&self) -> Option<Vec<AccountId>> {
        if self.list_assets.is_empty() {
            None
        } else {
            Some(self.list_assets.iter().map(|asset| asset.token_id.clone()).collect())
        }
    }
}
