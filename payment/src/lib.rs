use models::contract::{Payment, PaymentStorageKey, PaymentExt};
use near_sdk::borsh::BorshSerialize;
use near_sdk::{
    collections::{LookupMap, UnorderedSet},
    env, near_bindgen, AccountId,
};

pub mod application;
pub mod models;

#[near_bindgen]
impl Payment {
    #[init]
    pub fn init() -> Self {
        let owner_id = env::signer_account_id();

        Self::new(owner_id)
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            list_assets: Vec::new(),
            records_user_by_id: LookupMap::new(PaymentStorageKey::RecordUserById.try_to_vec().unwrap()),
            all_user_id: UnorderedSet::new(PaymentStorageKey::AllUserId.try_to_vec().unwrap())
        }
    }
}
