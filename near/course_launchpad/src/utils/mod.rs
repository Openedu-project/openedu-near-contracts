use near_sdk::{env, NearToken};

pub const ONE_NEAR: NearToken = NearToken::from_near(1);
pub const MIN_DEPOSIT: NearToken = ONE_NEAR;

pub fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        NearToken::from_yoctonear(1),
        "Requires attached deposit of exactly 1 yoctoNEAR"
    );
}

pub fn assert_at_least_one_near() {
    assert!(
        env::attached_deposit() >= ONE_NEAR,
        "Requires attached deposit of at least 1 NEAR"
    );
}