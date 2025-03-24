use near_sdk::{env, Gas};

pub fn yocto_near_to_near(yocto: u128) -> f64 {
    let near = yocto as f64 / 1e24;
    return near;
}

pub fn mul_div_down(x: u128, y: u128, denominator: u128) -> u128 {
    assert!(denominator != 0, "Denominator cannot be zero");

    let max_uint256: u128 = u128::MAX;

    if denominator == 0 || (y != 0 && x > max_uint256 / y) {
        // Equivalent to require(denominator != 0 && (y == 0 || x <= type(uint256).max / y))
        env::panic_str("mulDivDown: Denominator is zero or overflow");
    }

    // Divide x * y by the denominator.
    x * y / denominator
}

pub fn random_in_range(start: i64, end: i64) -> u64 {
    let block_timestamp = env::block_timestamp();
    //todo
    let random_seed = block_timestamp % ((end - start).abs() as u64 + 1) + start as u64;

    random_seed
}

pub fn sqrt(x: u128) -> u128 {
    // Kiểm tra giá trị x
    if x <= 0 {
        return 0;
    }

    // Phương pháp tìm kiếm nhị phân
    let mut lo = 0;
    let mut hi = x;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if mid * mid <= x {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    // Trả về giá trị căn bậc 2
    return lo - 1;
}

pub const GAS_FOR_CROSS_CALL: Gas = Gas(3_000_000_000_000);
pub const ATTACHED_DEPOSIT_NFT: u128 = 100_000_000_000_000_000_000_000;
pub const ATTACHED_BURN_FT: u128 = 1_000_000_000_000;
pub const PRECISION: u128 = 1e24 as u128;

pub const SECOND: u128 = 1_000_000_000;
pub const MINUTE: u128 = 60_000_000_000;
pub const HOUR: u128 = 3_600_000_000_000;
pub const DAY: u128 = 86_400_000_000_000; 
