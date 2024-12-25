use near_gas::NearGas;
use near_token::NearToken;
use near_units::parse_near;
use serde_json::json;
mod helpers;
use near_sdk::json_types::U128;
use near_workspaces::{Account, Contract};


use helpers::{
    storage_deposit, Status, PoolMetadata
};

use crate::helpers::{};

const LAUNCHPAD_WASM_FILEPATH: &str = "../res/launchpad.wasm";
const FT_WASM_FILEPATH: &str = "../res/ft_token.wasm";

const INITIAL_NEAR: NearToken = NearToken::from_near(30);

const DEFAULT_DEPOSIT: NearToken = NearToken::from_yoctonear(1);
const DEFAULT_GAS: NearGas = NearGas::from_tgas(200);
const INIT_POOL: NearToken = NearToken::from_near(1);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initiate environment
    let worker = near_workspaces::sandbox().await?;

    // deploy contracts
    let ft_wasm = std::fs::read(FT_WASM_FILEPATH)?;
    let ft_contract = worker.dev_deploy(&ft_wasm).await?;

    let launchpad_wasm = std::fs::read(LAUNCHPAD_WASM_FILEPATH)?;
    let launchpad_contract = worker.dev_deploy(&launchpad_wasm).await?;

    let owner = worker.root_account().unwrap();

    let owner_ft = owner
        .create_subaccount("ft")
        .initial_balance(INITIAL_NEAR)
        .transact()
        .await?
        .into_result()?;

    let owner_launchpad = owner
        .create_subaccount("launchpad")
        .initial_balance(INITIAL_NEAR)
        .transact()
        .await?
        .into_result()?;

    // Create Creator Pool Account
    let creator = owner
        .create_subaccount("creator")
        .initial_balance(INITIAL_NEAR)
        .transact()
        .await?
        .into_result()?;

    let backer = owner
        .create_subaccount("backer")
        .initial_balance(INITIAL_NEAR)
        .transact()
        .await?
        .into_result()?;

    // Call new construct for FT
    ft_contract
        .call("new_default_meta")
        .args_json(json!({
            "owner_id": owner_ft.id(),
            "total_supply": U128::from(parse_near!("1,000,000,000 N")),
        }))
        .transact()
        .await?
        .into_result()?;

    // Call init constructor for launchpad contract
    owner_launchpad
        .call(launchpad_contract.id(), "init")
        .args_json(json!({     
        }))
        .transact()
        .await?
        .into_result()?;

    // test add token to launchpad contract
    test_add_token(&launchpad_contract, &owner_launchpad, &ft_contract).await?;
    test_init_pool(&launchpad_contract, &owner_launchpad, &creator, &ft_contract).await?;
    test_admin_set_status_pool_pre_funding(&launchpad_contract, &owner_launchpad).await?;

    Ok(())
}

pub async fn test_add_token(
    launchpad_contract: &Contract,
    owner_launchpad: &Account,
    ft_contract: &Contract,
) -> anyhow::Result<()> {

    owner_launchpad
        .call(launchpad_contract.id(), "add_token")
        .args_json(json!({"token_id": ft_contract.id()}))
        .transact()
        .await?
        .into_result()?;

    let token_valid: bool = owner_launchpad
        .call(launchpad_contract.id(), "is_token_supported")
        .args_json(json!({
            "token_id": ft_contract.id()
        }))
        .transact()
        .await?
        .json()?;
    assert_eq!(token_valid, true);
    println!("      Passed ✅ test_add_token");
    Ok(())

}

pub async fn test_init_pool(
    launchpad_contract: &Contract,
    owner_launchpad: &Account, 
    creator: &Account, 
    ft_contract: &Contract
) -> anyhow::Result<()> {
    
    let time_now: u64 = launchpad_contract
        .call("get_current_timestamp")
        .view()
        .await?
        .json()?;
    
    let time_start_pledge = time_now + 60_000_000_000; // 1 minute in nanoseconds
    let time_end_pledge = time_now + 20 * 60_000_000_000; // 20 minute

    creator
        .call(launchpad_contract.id(), "init_pool")
        .args_json(json!({
            "campaign_id": "test1",
            "token_id": ft_contract.id(),
            "min_multiple_pledge": 10000,
            "time_start_pledge": time_start_pledge,
            "time_end_pledge": time_end_pledge,
            "target_funding": "1000000"
        }))
        .deposit(INIT_POOL)
        .transact()
        .await?
        .into_result()?;

    let pool1: Option<PoolMetadata> = owner_launchpad
        .call(launchpad_contract.id(), "get_detail_pool")
        .args_json(json!({
            "pool_id": 1
        }))
        .transact()
        .await?
        .json()?;

    assert!(pool1.is_some(), "Pool should be initialized and exist.");
    let pool_metadata = pool1.unwrap();
    assert_eq!(pool_metadata.campaign_id, "test1", "Campaign ID should match.");
    assert_eq!(pool_metadata.token_id.to_string(), ft_contract.id().to_string(), "Token ID should match.");
    assert_eq!(pool_metadata.min_multiple_pledge, 10000, "Min multiple pledge should match.");
    assert_eq!(pool_metadata.time_start_pledge, time_start_pledge, "Start time should match.");
    assert_eq!(pool_metadata.time_end_pledge, time_end_pledge, "End time should match.");
    assert_eq!(pool_metadata.target_funding, 1000000, "Target funding should match.");
    println!("      Passed ✅ test_init_pool");
    Ok(())
}

pub async fn test_admin_set_status_pool_pre_funding(
    launchpad_contract: &Contract, 
    owner_launchpad: &Account
) -> anyhow::Result<()> {   

    owner_launchpad
        .call(launchpad_contract.id(), "admin_set_status_pool_pre_funding")
        .args_json(json!({
            "pool_id": 1, 
            "approve": true
        }))
        .transact()
        .await?
        .into_result()?;

    let pool1: Option<PoolMetadata> = owner_launchpad
        .call(launchpad_contract.id(), "get_detail_pool")
        .args_json(json!({
            "pool_id": 1
        }))
        .transact()
        .await?
        .json()?;

    assert!(pool1.is_some(), "Pool should be initialized and exist.");
    let pool_metadata = pool1.unwrap();
    
    assert_eq!(pool_metadata.status, Status::FUNDING, "Pool status should be FUNDING.");

    println!("      Passed ✅ test_admin_set_status_pool_pre_funding");
    Ok(())
}
