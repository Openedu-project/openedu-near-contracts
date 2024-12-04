use near_sdk::json_types::U128;
use near_sdk::{AccountId, Balance};
use near_sdk::serde_json::json;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::testing_env;

use course_launchpad::Contract;
use course_launchpad::pool::enums::PoolStatus;

// >>>>> Start test helpers <<<<<
fn setup_contract() -> (VMContextBuilder, Contract) {
    let mut context = VMContextBuilder::new();
    let owner_id: AccountId = accounts(0);
    testing_env!(context.predecessor_account_id(owner_id.clone()).build());
    
    let contract = Contract::new(owner_id);
    (context, contract)
}

const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;
// >>>>> End test helpers <<<<<

// >>>>> Start creator tests <<<<<
#[test]
fn test_init_pool() {
    let (mut context, mut contract) = setup_contract();
    let creator = accounts(1);
    
    testing_env!(context.predecessor_account_id(creator.clone()).build());
    
    contract.init_pool("pool1".to_string(), "campaign1".to_string());
    
    let pool = contract.pools.get("pool1").unwrap();
    assert_eq!(pool.creator_id, creator);
    assert_eq!(pool.campaign_id, "campaign1");
    assert_eq!(pool.status, PoolStatus::INIT);
}

#[test]
#[should_panic(expected = "Pool already exists")]
fn test_init_pool_duplicate() {
    let (mut context, mut contract) = setup_contract();
    testing_env!(context.predecessor_account_id(accounts(1)).build());
    
    contract.init_pool("pool1".to_string(), "campaign1".to_string());
    contract.init_pool("pool1".to_string(), "campaign2".to_string());
}
// >>>>> End creator tests <<<<<

// >>>>> Start admin tests <<<<<
#[test]
fn test_change_pool_info() {
    let (mut context, mut contract) = setup_contract();
    let owner = accounts(0);
    let creator = accounts(1);
    
    // First create a pool
    testing_env!(context.predecessor_account_id(creator).build());
    contract.init_pool("pool1".to_string(), "campaign1".to_string());
    
    // Then change pool info as owner
    testing_env!(context.predecessor_account_id(owner).build());
    contract.change_pool_infor("pool1".to_string(), Some(2 * ONE_NEAR));
    
    let pool = contract.pools.get("pool1").unwrap();
    assert_eq!(pool.min_deposit, 2 * ONE_NEAR);
}

#[test]
fn test_start_voting() {
    let (mut context, mut contract) = setup_contract();
    let owner = accounts(0);
    let creator = accounts(1);
    let backer = accounts(2);
    
    // Create pool
    testing_env!(context.predecessor_account_id(creator).build());
    contract.init_pool("pool1".to_string(), "campaign1".to_string());
    
    // Make pledge
    testing_env!(context
        .predecessor_account_id(backer)
        .attached_deposit(2 * ONE_NEAR)
        .build());
    contract.pledge_to_pool("pool1".to_string());
    
    // Start voting
    testing_env!(context.predecessor_account_id(owner).build());
    contract.start_voting("pool1".to_string());
    
    let pool = contract.pools.get("pool1").unwrap();
    assert_eq!(pool.status, PoolStatus::VOTING);
    assert_eq!(pool.user_records.get(&backer).unwrap().power, 2);
}
// >>>>> End admin tests <<<<<

// >>>>> Start backer tests <<<<<
// #[test]
// fn test_pledge_to_pool() {
//     let (mut context, mut contract) = setup_contract();
//     let creator = accounts(1);
//     let backer = accounts(2);
    
//     // Create pool
//     testing_env!(context.predecessor_account_id(creator).build());
//     contract.init_pool("pool1".to_string(), "campaign1".to_string());
    
//     // Make pledge
//     testing_env!(context
//         .predecessor_account_id(backer.clone())
//         .attached_deposit(ONE_NEAR)
//         .build());
//     contract.pledge_to_pool("pool1".to_string());
    
//     let pool = contract.pools.get("pool1").unwrap();
//     assert_eq!(pool.total_balance, ONE_NEAR);
//     assert_eq!(pool.user_records.get(&backer).unwrap().balance, ONE_NEAR);
// }

#[test]
#[should_panic(expected = "Deposit too small")]
fn test_pledge_too_small() {
    let (mut context, mut contract) = setup_contract();
    let creator = accounts(1);
    let backer = accounts(2);
    
    testing_env!(context.predecessor_account_id(creator).build());
    contract.init_pool("pool1".to_string(), "campaign1".to_string());
    
    testing_env!(context
        .predecessor_account_id(backer)
        .attached_deposit(ONE_NEAR / 2)
        .build());
    contract.pledge_to_pool("pool1".to_string());
}
// >>>>> End backer tests <<<<<

// >>>>> Start getter tests <<<<<
#[test]
fn test_get_user_power() {
    let (mut context, mut contract) = setup_contract();
    let creator = accounts(1);
    let backer = accounts(2);
    
    // Create and setup pool with pledge
    testing_env!(context.predecessor_account_id(creator).build());
    contract.init_pool("pool1".to_string(), "campaign1".to_string());
    
    testing_env!(context
        .predecessor_account_id(backer.clone())
        .attached_deposit(2 * ONE_NEAR)
        .build());
    contract.pledge_to_pool("pool1".to_string());
    
    // Start voting to calculate power
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    contract.start_voting("pool1".to_string());
    
    assert_eq!(contract.get_user_power("pool1".to_string(), backer), 2);
}

#[test]
fn test_get_user_balance() {
    let (mut context, mut contract) = setup_contract();
    let creator = accounts(1);
    let backer = accounts(2);
    
    testing_env!(context.predecessor_account_id(creator).build());
    contract.init_pool("pool1".to_string(), "campaign1".to_string());
    
    testing_env!(context
        .predecessor_account_id(backer.clone())
        .attached_deposit(2 * ONE_NEAR)
        .build());
    contract.pledge_to_pool("pool1".to_string());
    
    assert_eq!(
        contract.get_user_balance("pool1".to_string(), backer),
        2 * ONE_NEAR
    );
}
// >>>>> End getter tests <<<<<


