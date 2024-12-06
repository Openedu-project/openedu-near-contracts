
use near_sdk::json_types::Base64VecU8;
use near_sdk::AccountId;
use std::collections::HashMap;
use near_token::NearToken;
use near_workspaces::{Account, Contract};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub type PoolId = u64;


#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PoolMetadata {
    pub pool_id: PoolId,
    pub campaign_id: String,
    pub creator_id: AccountId,
    pub staking_amount: u128,
    pub status: Status,
    pub token_id: AccountId,
    pub total_balance: u128,
    pub time_start_pledge: u64,
    pub time_end_pledge: u64,
    pub mint_multiple_pledge: u8,
    pub user_records: Vec<UserTokenDepositRecord>
}


#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Assets {
    pub token_id: AccountId,
    pub balances: u128,
}


#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
    INIT,
    FUNDING,
    WAITING,
    VOTING,
    CLOSED,
}


#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserTokenDepositRecord {
    pub user_id: AccountId,
    pub amount: u128,
    pub voting_power: f64,
}

pub async fn storage_deposit(
    owner: &Account,
    ft_contract: &Contract,
    user: &Account,
) -> anyhow::Result<()> {
    //Register owner storage deposit ft_contract
    let default_deposit = NearToken::from_millinear(8);
    owner
        .call(ft_contract.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": user.id()
        }))
        .deposit(default_deposit)
        .transact()
        .await?
        .into_result()?;
    Ok(())
}

// Test Get Method 