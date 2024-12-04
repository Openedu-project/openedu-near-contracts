use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum PoolStatus {
    INIT,      // Initial state when created by user
    FUNDING,   // Approved by admin, ready for backers
    VOTING,    // Funding completed, voting phase
    CLOSED,    // Rejected by admin or completed
    WAIT,      // Other states if needed
}