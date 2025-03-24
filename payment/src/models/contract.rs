use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::{LookupMap, UnorderedSet},
    json_types::Base64VecU8,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault,
    PromiseOrValue,
    json_types::U128
};

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Payment {
    /// Account ID of the owner of the contract.
    pub owner_id: AccountId,  
    pub list_assets: Vec<Assets>,
    pub records_user_by_id: LookupMap<AccountId, UserTokenDepositRecord>,
    pub all_user_id: UnorderedSet<AccountId>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Assets {
    pub token_id: AccountId,
    pub balances: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct UserTokenDepositRecord {
    pub user_id: AccountId,
    pub deposits: Vec<TokenDeposit>,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PaymentInfo {
    pub user_id: AccountId,
    pub amount: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenDeposit {
    pub token_id: AccountId,
    pub amount: u128,
}

#[derive(BorshSerialize)]
pub enum PaymentStorageKey {
    RecordUserById,
    AllUserId,
}

pub trait PaymentFeature {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;

    fn add_token(
        &mut self,
        token_id: String,
    );

    fn claim(
        &mut self,
        token_id: String,
    );

    fn change_admin(
        &mut self,
        new_admin: AccountId
    );

    fn delete_token_by_token_id(
        &mut self,
        token_id: AccountId
    );
}

pub trait PaymentEnum {
    fn get_user_info_by_id(&self, user_id: AccountId) -> Option<UserTokenDepositRecord>;
    fn get_all_token_id(&self) -> Option<Vec<AccountId>>;
}