use near_gas::NearGas;
use near_token::NearToken;
use near_units::parse_near;
use serde_json::json;
mod helpers;
use near_sdk::json_types::U128;
use near_workspaces::{Account, Contract};
use tokio::time::{sleep, Duration};

use helpers::{
    storage_deposit, Status, PoolMetadata, UserRecordDetail
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

    let backer1 = owner
        .create_subaccount("backer1")
        .initial_balance(INITIAL_NEAR)
        .transact()
        .await?
        .into_result()?;

    let backer2 = owner
        .create_subaccount("backer2")
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
    // transfer token to backer 1&2
    test_transfer_token_to_all_backer(&owner_ft, &ft_contract, &backer1, &backer2).await?;

    // test add token to launchpad contract
    test_add_token(&launchpad_contract, &owner_launchpad, &ft_contract).await?;
    test_init_pools(&launchpad_contract, &owner_launchpad, &creator, &ft_contract).await?;
    test_admin_set_status_pool_pre_funding(&launchpad_contract, &owner_launchpad).await?;
    
    sleep(Duration::from_secs(3)).await;
    
    test_backers_deposit_token_to_pools(&ft_contract, &launchpad_contract, &backer1, &backer2).await?;
    
    let balance_backer1: U128 = ft_contract
        .call("ft_balance_of")
        .args_json(json!({"account_id": backer1.id()}))
        .view()
        .await?
        .json()?;
    
    println!("Balance Backer1: {}", balance_backer1.0);

    // delay waited time ending funding
    sleep(Duration::from_secs(32)).await;
    
    test_check_funding_result(&launchpad_contract, &owner_launchpad).await?;
    test_claim_refund_for_backers(&launchpad_contract, &ft_contract, &backer1, &backer2).await?;

    sleep(Duration::from_secs(3)).await;
    
    let balance_backer1: U128 = ft_contract
        .call("ft_balance_of")
        .args_json(json!({"account_id": backer1.id()}))
        .view()
        .await?
        .json()?;
    
    println!("Balance Backer1: {}", balance_backer1.0);
    Ok(())
}

pub async fn test_transfer_token_to_all_backer(
    owner_ft: &Account, 
    ft_contract: &Contract, 
    backer1: &Account, 
    backer2: &Account
) -> anyhow::Result<()> {

    // add storage for two backer
    storage_deposit(owner_ft, ft_contract, backer1).await?;
    storage_deposit(owner_ft, ft_contract, backer2).await?;
    
    // transfer
    owner_ft
        .call(ft_contract.id(), "ft_transfer")
        .args_json(json!({
            "receiver_id": backer1.id(),
            "amount": U128(parse_near!("10 N"))
        }))
        .deposit(DEFAULT_DEPOSIT)
        .transact()
        .await?
        .into_result()?;

    owner_ft
        .call(ft_contract.id(), "ft_transfer")
        .args_json(json!({
            "receiver_id": backer2.id(),
            "amount": U128(parse_near!("10 N"))
        }))
        .deposit(DEFAULT_DEPOSIT)
        .transact()
        .await?
        .into_result()?;
    
    let balance_backer1: U128 = ft_contract
        .call("ft_balance_of")
        .args_json(json!({"account_id": backer1.id()}))
        .view()
        .await?
        .json()?;
    
    let balance_backer2: U128 = ft_contract
        .call("ft_balance_of")
        .args_json(json!({"account_id": backer2.id()}))
        .view()
        .await?
        .json()?;
    
    assert_eq!(balance_backer1, U128::from(10_000_000_000_000_000_000_000_000), "Backer1 balance should be 10,000,000,000,000,000,000,000,000 yoctoNEAR");
    assert_eq!(balance_backer2, U128::from(10_000_000_000_000_000_000_000_000), "Backer2 balance should be 10,000,000,000,000,000,000,000,000 yoctoNEAR");

    println!("      Passed ✅ test_transfer_token_to_all_backer");
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

pub async fn test_init_pools(
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
    
    let time_start_pledge = time_now + 1_000_000_000; // 1 s in nanoseconds
    let time_end_pledge = time_now + 30_000_000_000; // 20 minute

    // init pool 1
    creator
        .call(launchpad_contract.id(), "init_pool")
        .args_json(json!({
            "campaign_id": "test1",
            "token_id": ft_contract.id(),
            "min_multiple_pledge": 10000,
            "time_start_pledge": time_start_pledge,
            "time_end_pledge": time_end_pledge,
            "target_funding": "10000000"
        }))
        .deposit(INIT_POOL)
        .transact()
        .await?
        .into_result()?;

    // init pool 2
    creator
        .call(launchpad_contract.id(), "init_pool")
        .args_json(json!({
            "campaign_id": "test2",
            "token_id": ft_contract.id(),
            "min_multiple_pledge": 10000,
            "time_start_pledge": time_start_pledge + 1_000_000_000,
            "time_end_pledge": time_end_pledge,
            "target_funding": "10000000"
        }))
        .deposit(INIT_POOL)
        .transact()
        .await?
        .into_result()?;

    
    // init pool 3
    creator
        .call(launchpad_contract.id(), "init_pool")
        .args_json(json!({
            "campaign_id": "test3",
            "token_id": ft_contract.id(),
            "min_multiple_pledge": 10000,
            "time_start_pledge": time_start_pledge + 3_000_000_000,
            "time_end_pledge": time_end_pledge,
            "target_funding": "100000000000000"
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
    assert_eq!(pool_metadata.target_funding, 10000000, "Target funding should match.");
    println!("      Passed ✅ test_init_pools");
    Ok(())
}

pub async fn test_admin_set_status_pool_pre_funding(
    launchpad_contract: &Contract, 
    owner_launchpad: &Account
) -> anyhow::Result<()> {   

    // approve pool1 by admin
    owner_launchpad
        .call(launchpad_contract.id(), "admin_set_status_pool_pre_funding")
        .args_json(json!({
            "pool_id": 1, 
            "approve": true
        }))
        .transact()
        .await?
        .into_result()?;

    // approve pool2 by admin
    owner_launchpad
        .call(launchpad_contract.id(), "admin_set_status_pool_pre_funding")
        .args_json(json!({
            "pool_id": 2, 
            "approve": true
        }))
        .transact()
        .await?
        .into_result()?;

    // approve pool3 by admin
    owner_launchpad
        .call(launchpad_contract.id(), "admin_set_status_pool_pre_funding")
        .args_json(json!({
            "pool_id": 3, 
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

pub async fn test_backers_deposit_token_to_pools(
    ft_contract: &Contract,
    launchpad_contract: &Contract, 
    backer1: &Account,
    backer2: &Account
) -> anyhow::Result<()> {
    
    let backers = vec![
        (backer1, "10000000000000"),
        (backer2, "20000000000000"),
    ];

    // backers deposit token to pool1
    for (backer, amount) in backers.clone() {
        backer
            .call(ft_contract.id(), "ft_transfer_call")
            .args_json(json!({
                "receiver_id": launchpad_contract.id(), 
                "amount": amount, 
                "msg": "1"
            }))
            .deposit(DEFAULT_DEPOSIT)
            .gas(NearGas::from_tgas(300)) // Add more gas to prevent execution error
            .transact()
            .await?
            .into_result()?;
    }

    // backers deposit token to pool3 
    for (backer, amount) in backers.clone() {
        backer
            .call(ft_contract.id(), "ft_transfer_call")
            .args_json(json!({
                "receiver_id": launchpad_contract.id(), 
                "amount": amount, 
                "msg": "3"
            }))
            .deposit(DEFAULT_DEPOSIT)
            .gas(NearGas::from_tgas(300)) // Add more gas to prevent execution error
            .transact()
            .await?
            .into_result()?;
    }

    // record pool1
    let list_records_pool1: Option<Vec<UserRecordDetail>> = backer1
        .call(launchpad_contract.id(), "get_user_records_by_pool_id")
        .args_json(json!({
            "pool_id": 1
        }))
        .transact()
        .await?
        .json()?;
        
    if let Some(records) = list_records_pool1 {
        for record in records {
            println!("Pool 1: User ID: {}, Amount: {}", record.user_id, record.record.amount);
        }
    } else {
        println!("No records found for the given pool1");
    }

    // record pool3
    let list_records_pool3: Option<Vec<UserRecordDetail>> = backer1
        .call(launchpad_contract.id(), "get_user_records_by_pool_id")
        .args_json(json!({
            "pool_id": 3
        }))
        .transact()
        .await?
        .json()?;
        
    if let Some(records) = list_records_pool3 {
        for record in records {
            println!("Pool 3: User ID: {}, Amount: {}", record.user_id, record.record.amount);
        }
    } else {
        println!("No records found for the given pool2");
    }
        
    println!("      Passed ✅ test_backers_deposit_token_to_pools");
    Ok(())
}

pub async fn test_check_funding_result(
    launchpad_contract: &Contract,
    owner_launchpad: &Account
) -> anyhow::Result<()> {

    // check funding result pool 1
    owner_launchpad
        .call(launchpad_contract.id(), "check_funding_result")
        .args_json(json!({
            "pool_id": 1, 
            "is_waiting_funding": true
        }))
        .transact()
        .await?
        .into_result()?;

    // check funding result pool 2
    owner_launchpad
        .call(launchpad_contract.id(), "check_funding_result")
        .args_json(json!({
            "pool_id": 2, 
            "is_waiting_funding": true
        }))
        .transact()
        .await?
        .into_result()?;

    // check funding result pool 3
    owner_launchpad
        .call(launchpad_contract.id(), "check_funding_result")
        .args_json(json!({
            "pool_id": 3, 
            "is_waiting_funding": false
        }))
        .transact()
        .await?
        .into_result()?;

    let list_records: Option<Vec<UserRecordDetail>> = owner_launchpad
        .call(launchpad_contract.id(), "get_user_records_by_pool_id")
        .args_json(json!({
            "pool_id": 1
        }))
        .transact()
        .await?
        .json()?;

    if let Some(records) = list_records {
        for record in records {
            println!("User ID: {}, Amount: {}, Voting Power: {}", record.user_id, record.record.amount, record.record.voting_power);
        }
    } else {
        println!("No records found for the given pool_id.");
    }

    let pool1: Option<PoolMetadata> = owner_launchpad
        .call(launchpad_contract.id(), "get_detail_pool")
        .args_json(json!({
            "pool_id": 1
        }))
        .transact()
        .await?
        .json()?;
    
    let pool2: Option<PoolMetadata> = owner_launchpad
        .call(launchpad_contract.id(), "get_detail_pool")
        .args_json(json!({
            "pool_id": 2
        }))
        .transact()
        .await?
        .json()?;

    let pool3: Option<PoolMetadata> = owner_launchpad
        .call(launchpad_contract.id(), "get_detail_pool")
        .args_json(json!({
            "pool_id": 3
        }))
        .transact()
        .await?
        .json()?;

    // check status pool 1    
    assert!(pool1.is_some(), "Pool should be initialized and exist.");
    let pool_metadata1 = pool1.unwrap();
    assert_eq!(pool_metadata1.status, Status::VOTING, "Pool status should be VOTING.");

    // check status pool 2
    assert!(pool2.is_some(), "Pool should be initialized and exist.");
    let pool_metadata2 = pool2.unwrap();
    assert_eq!(pool_metadata2.status, Status::FAILED, "Pool status should be FAILED.");

    // check status pool 3
    assert!(pool3.is_some(), "Pool should be initialized and exist.");
    let pool_metadata3 = pool3.unwrap();
    assert_eq!(pool_metadata3.status, Status::REFUNDED, "Pool status should be REFUNDED.");

    println!("      Passed ✅ test_check_funding_result_is_voting");

    Ok(())
}

pub async fn test_claim_refund_for_backers(
    launchpad_contract: &Contract,
    ft_contract: &Contract,
    backer1: &Account,
    backer2: &Account
) -> anyhow::Result<()> {

    backer1
        .call(launchpad_contract.id(), "claim_refund")
        .args_json(json!({
            "pool_id": 3, 
        }))
        .gas(NearGas::from_tgas(30))
        .transact()
        .await?
        .into_result()?;

    backer2
        .call(launchpad_contract.id(), "claim_refund")
        .args_json(json!({
            "pool_id": 3, 
        }))
        .gas(NearGas::from_tgas(30))
        .transact()
        .await?
        .into_result()?;
    

    let list_records: Option<Vec<UserRecordDetail>> = backer1
        .call(launchpad_contract.id(), "get_user_records_by_pool_id")
        .args_json(json!({
            "pool_id": 3
        }))
        .transact()
        .await?
        .json()?;

    if let Some(records) = list_records {
        for record in records {
            println!("User ID: {}, Amount: {}, Voting Power: {}", record.user_id, record.record.amount, record.record.voting_power);
        }
    } else {
        println!("No records found for the given pool_id.");
    }

    println!("      Passed ✅ test_claim_refund_for_backers");

    Ok(())

}