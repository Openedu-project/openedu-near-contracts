use near_gas::NearGas;
use near_token::NearToken;
use near_units::parse_near;
use serde_json::json;
mod helpers;
use near_sdk::json_types::U128;
use near_workspaces::{Account, Contract};

use helpers::{
    storage_deposit, Status
};

use crate::helpers::{};

const LAUNCHPAD_WASM_FILEPATH: &str = "../res/launchpad.wasm";
const FT_WASM_FILEPATH: &str = "../res/ft_token.wasm";

const INITIAL_NEAR: NearToken = NearToken::from_near(30);

const DEFAULT_DEPOSIT: NearToken = NearToken::from_yoctonear(1);
const DEFAULT_GAS: NearGas = NearGas::from_tgas(200);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initiate environemnt
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

    // Call new construct for NFT
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
    test_add_token(&launchpad_contract , &owner_launchpad, &ft_contract).await?;
        
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
