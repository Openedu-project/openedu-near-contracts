use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault};
use near_sdk::collections::UnorderedMap;
use std::collections::HashMap;

mod pool;
mod utils;

use pool::{models::*, enums::*, actions::*};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub pools: UnorderedMap<u64, Pool>,
    pub pool_id_counter: u64,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            pools: UnorderedMap::new(b"p"),
            pool_id_counter: 0,
        }
    }
}