use crate::*;
use near_sdk::{env, NearToken, Promise};
use crate::utils::{MIN_DEPOSIT, DEFAULT_MIN_STAKING};

#[near_bindgen]
impl Contract {
    // >>>>> Start creator funcs <<<<<
    #[payable]
    pub fn init_pool(&mut self, campaign_id: String) -> u64 {
        let creator_id = env::predecessor_account_id();
        let staking_amount = env::attached_deposit();
        let pool_id = self.pool_id_counter;
        
        let pool = self.pools.get(&pool_id).unwrap_or_else(|| Pool {
            pool_id,
            campaign_id,
            creator_id,
            staking_amount,
            status: PoolStatus::INIT,
            total_balance: NearToken::from_near(0),
            min_deposit: MIN_DEPOSIT,
            min_staking: DEFAULT_MIN_STAKING,
            user_records: HashMap::new(),
            created_at: env::block_timestamp(),
        });

        // Check minimum staking requirement
        assert!(
            staking_amount >= pool.min_staking,
            "Must stake at least {} NEAR to create pool",
            pool.min_staking.as_near() as u64
        );
        
        self.pools.insert(&pool_id, &pool);
        self.pool_id_counter += 1;
        pool_id
    }

    // New admin function to set minimum staking amount
    pub fn set_min_staking(&mut self, pool_id: u64, amount: NearToken) {
        let mut pool = self.pools.get(&pool_id).expect("Pool not found");
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can set minimum staking");
        assert!(amount >= DEFAULT_MIN_STAKING, "Minimum staking cannot be less than 1 NEAR");
        
        pool.min_staking = amount;
        self.pools.insert(&pool_id, &pool);
    }

    // >>>>> Start admin funcs <<<<<
    pub fn approve_pool(&mut self, pool_id: u64) {
        let mut pool = self.pools.get(&pool_id).expect("Pool not found");
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can approve pool");
        assert_eq!(pool.status, PoolStatus::INIT, "Pool must be in INIT status");
        
        pool.status = PoolStatus::FUNDING;
        self.pools.insert(&pool_id, &pool);
    }

    pub fn reject_pool(&mut self, pool_id: u64) {
        let mut pool = self.pools.get(&pool_id).expect("Pool not found");
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can reject pool");
        assert_eq!(pool.status, PoolStatus::INIT, "Pool must be in INIT status");
        
        // Return staking amount to creator when pool is rejected
        Promise::new(pool.creator_id.clone()).transfer(pool.staking_amount);
        
        pool.status = PoolStatus::CLOSED;
        self.pools.insert(&pool_id, &pool);
    }

    pub fn change_pool_infor(&mut self, pool_id: u64, min_deposit: Option<NearToken>, min_staking: Option<NearToken>) {
        let mut pool = self.pools.get(&pool_id).expect("Pool not found");
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can change pool info");
        
        if let Some(min_deposit) = min_deposit {
            pool.min_deposit = min_deposit;
        }
        
        if let Some(min_staking) = min_staking {
            assert!(min_staking >= DEFAULT_MIN_STAKING, "Minimum staking cannot be less than 1 NEAR");
            pool.min_staking = min_staking;
        }
        
        self.pools.insert(&pool_id, &pool);
    }

    pub fn start_voting(&mut self, pool_id: u64) {
        let mut pool = self.pools.get(&pool_id).expect("Pool not found");
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can start voting");
        assert_eq!(pool.status, PoolStatus::FUNDING, "Pool must be in FUNDING status");
        
        pool.status = PoolStatus::VOTING;
        // Calculate voting power for each user
        for (_, user_record) in pool.user_records.iter_mut() {
            let balance_yocto = user_record.balance.as_yoctonear();
            let min_deposit_yocto = MIN_DEPOSIT.as_yoctonear();
            user_record.power = (balance_yocto / min_deposit_yocto) as u32;
        }
        self.pools.insert(&pool_id, &pool);
    }

    // >>>>> Start getter funcs <<<<<
    pub fn get_all_init_pools(&self) -> Vec<Pool> {
        self.pools
            .values()
            .filter(|pool| pool.status == PoolStatus::INIT)
            .collect()
    }

    pub fn get_all_funding_pools(&self) -> Vec<Pool> {
        self.pools
            .values()
            .filter(|pool| pool.status == PoolStatus::FUNDING)
            .collect()
    }

    pub fn get_user_power(&self, pool_id: u64, user_id: AccountId) -> u32 {
        let pool = self.pools.get(&pool_id).expect("Pool not found");
        pool.user_records.get(&user_id).map(|r| r.power).unwrap_or(0)
    }

    pub fn get_user_balance(&self, pool_id: u64, user_id: AccountId) -> NearToken {
        let pool = self.pools.get(&pool_id).expect("Pool not found");
        pool.user_records.get(&user_id).map(|r| r.balance).unwrap_or(NearToken::from_near(0))
    }
}