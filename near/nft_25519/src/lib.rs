/*!
Non-Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, log, serde::{Deserialize, Serialize},
};

use ed25519_dalek::{PublicKey, Signature, Verifier};
use near_sdk::base64::decode;

pub type CourseId = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    admin_pub_key: String,
    course_metadata_by_id: LookupMap<CourseId, CourseMetadata>,
    total_balances: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CourseMetadata {
    course_id: CourseId,
    sponsor_balance: u128,
    creator_id: AccountId,
}

#[derive(BorshSerialize)]
pub enum CourseStorageKey {
    CourseById,
    AllCourseId,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by owner_id with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, admin_pub_key: String) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example NEAR non-fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
            admin_pub_key,
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata, admin_pub_key: String) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            admin_pub_key,
            course_metadata_by_id: LookupMap::new(CourseStorageKey::CourseById.try_to_vec().unwrap()),
            total_balances: 0,
        }
    }

    /// Mint a new token with ID=token_id belonging to receiver_id.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. self.tokens.mint will also require it to be Some, since
    /// StorageKey::TokenMetadata was provided at initialization.
    ///
    /// self.tokens.mint will enforce predecessor_account_id to equal the owner_id given in
    /// initialization call to new.
    #[payable]
    pub fn nft_mint_with_signature(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
        signature_base64: String,
        course_id: String,
    ) -> Token {
        let user_address = env::predecessor_account_id();

        let signature_bytes = decode(&signature_base64).expect("Invalid signature (Base64 decode error)");
        let pubkey_bytes = decode(&self.admin_pub_key).expect("Invalid public key");

        // Convert Vec<u8> to Signature
        let signature = Signature::from_bytes(&signature_bytes).expect("Invalid signature (Signature error)");

        // Create the message to verify
        let expected_message = format!("{}:{}", course_id, user_address);

        let public_key = PublicKey::from_bytes(&pubkey_bytes).expect("Invalid public key");

        assert!(
            public_key.verify(expected_message.as_bytes(), &signature).is_ok(),
            "Invalid signature"
        );

        self.tokens.internal_mint(token_id, receiver_id, Some(token_metadata))
    }

    #[payable]
    pub fn nft_mint_for_sponsor(
        &mut self,
        token_id: TokenId,
        token_metadata: TokenMetadata,
        receiver_id: AccountId,
        course_id: CourseId
    ) -> Token {
        
        assert_eq!(
            env::predecessor_account_id(),
            self.tokens.owner_id,
            "Unauthorized"
        );

        let mut course_metadata = self.course_metadata_by_id.get(&course_id)
            .expect("Course metadata not found");
        
            let before_storage = env::storage_usage();
        
        // Mint NFT
        let token = self.tokens.internal_mint(token_id, receiver_id, Some(token_metadata));
        // Calculate storage fees
        let after_storage = env::storage_usage();
        let storage_used = after_storage - before_storage;
        let storage_cost = storage_used as u128 * env::storage_byte_cost();

        // TODO admin should be change
        let price_per_tgas: u128 = 100_000_000_000_000_000_000; // 0.0001 NEAR = 10^20 yoctoNEAR
        let additional_tgas: u128 = 5; // 3 TGas

        let used_tgas = env::used_gas().0 as u128 / u128::pow(10, 12); // Convert gas units to TGas
        let gas_cost = (used_tgas + additional_tgas) * price_per_tgas;
        let total_gas_cost = gas_cost + storage_cost;

        // Retrieve the current balance for the sponsor_id and course_id

        // Deduct the total_gas_cost from the sponsor's balance
        if course_metadata.sponsor_balance < total_gas_cost {
            // If the balance is insufficient, set the balance to 0
            course_metadata.sponsor_balance = 0;
        } else {
            course_metadata.sponsor_balance -= total_gas_cost;
        }

        // Update the course metadata with the new balance
        self.course_metadata_by_id.insert(&course_id, &course_metadata);

        // Log transaction details
        log!(
            "NFT Mint Transaction Details:\n\
            - Storage Cost: {} yoctoNEAR\n\
            - Gas Cost: {} yoctoNEAR\n\
            - Total Gas Cost: {} yoctoNEAR",
            storage_cost,
            gas_cost,
            total_gas_cost
        );

        token
    }

    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        assert_eq!(
            env::predecessor_account_id(),
            self.tokens.owner_id,
            "Unauthorized"
        );
        self.tokens
            .internal_mint(token_id, token_owner_id, Some(token_metadata))
    }

    // Ensure the function is payable to allow NEAR deposits
    #[payable]
    pub fn deposit_sponsor(&mut self, course_id: CourseId) {
        // Get the amount of NEAR deposited
        let deposit_amount = env::attached_deposit();

        // Update the sponsor records with the deposited amount
        let sponsor_id = env::predecessor_account_id().clone();

        // Check if a record for the course_id already exists with a different creator_id
        if let Some(record) = self.course_metadata_by_id.get(&course_id) {
            if record.creator_id != sponsor_id {
                env::panic_str("This course_id is not owned by the caller.");
            }
        }

        // Check if a record for the course_id and sponsor_id already exists
        let mut record = self.course_metadata_by_id.get(&course_id).unwrap_or(CourseMetadata {
            course_id: course_id.clone(),
            sponsor_balance: 0,
            creator_id: sponsor_id.clone(),
        });

        if record.creator_id == sponsor_id {
            record.sponsor_balance += deposit_amount;
        } else {
            env::panic_str("This course_id is not owned by the caller.");
        }

        // Update the record in the map
        self.course_metadata_by_id.insert(&course_id, &record);

        // Log the deposit action
        log!(
            "Sponsor {} deposited {} yoctoNEAR for course {}",
            sponsor_id,
            deposit_amount,
            course_id
        );
    }

    #[payable]
    pub fn withdraw_sponsor(&mut self, course_id: CourseId, amount: u128) {
        // Get the sponsor ID
        let sponsor_id = env::predecessor_account_id();

        // Find the sponsor record for the given course_id
        let mut record = self.course_metadata_by_id.get(&course_id).expect("No sponsor record found for the given course_id.");

        if record.creator_id != sponsor_id {
            env::panic_str("This course_id is not owned by the caller.");
        }
        if record.sponsor_balance < amount {
            env::panic_str("Insufficient balance to withdraw.");
        }

        record.sponsor_balance -= amount;
        self.course_metadata_by_id.insert(&course_id, &record);

        // Transfer the specified amount of NEAR back to the sponsor
        Promise::new(sponsor_id.clone()).transfer(amount);

        // Log the withdrawal action
        log!(
            "Sponsor {} withdrew {} yoctoNEAR from course {}",
            sponsor_id,
            amount,
            course_id
        );
    }

    pub fn get_sponsor_balance(&self, course_id: CourseId, sponsor_id: AccountId) -> Option<u128> {
        if let Some(record) = self.course_metadata_by_id.get(&course_id) {
            if record.creator_id == sponsor_id {
                return Some(record.sponsor_balance);
            }
        }
        None
    }
}

near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
