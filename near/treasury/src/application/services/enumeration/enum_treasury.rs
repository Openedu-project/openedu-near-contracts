use near_sdk::{env, near_bindgen, AccountId};

use crate::models::{
    contract::{Treasury, TreasuryExt, TreasuryEnum, UserTokenDepositRecord},
};

#[near_bindgen]
impl TreasuryEnum for Treasury {
    fn get_user_info_by_id(&self, user_id: AccountId) -> UserTokenDepositRecord {
        let user_info = self.records_user_by_id.get(&user_id).unwrap();

        user_info
    }
}
