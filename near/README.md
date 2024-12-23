# Contracts for testnet 

## Compile Contract
```bash
cargo make clean
cargo make build
```

## PAYMENT

```bash
near deploy $PAYMENT ./target/wasm32-unknown-unknown/release/payment.wasm
near call $PAYMENT init --accountId $ADMIN
# Admin add author & add token 
near call $PAYMENT add_token '{"token_id": "'$TOKEN_ID'"}' --accountId $ADMIN
near call $PAYMENT change_admin '{"new_admin": "''"}' --accountId $ADMIN
near call $PAYMENT delete_token_by_token_id '{"token_id": ""}' --accountId $ADMIN
```

### Function for Be
```bash
# user transfer token to contract
near call fun-token2.testnet ft_transfer_call '{"receiver_id": "payment-5.testnet", "amount": "30000", "msg": "[{\"user_id\": \"refferal-1.testnet\", \"amount\": 10000}, {\"user_id\": \"refferal-3.testnet\", \"amount\": 20000}]"}' --accountId creator1.testnet --gas 300000000000000 --depositYocto 1 

# user claim token from contract

near call $PAYMENT claim '{"token_id": "'$TOKEN_ID'"}' --accountId $USER1
```

### read data user_id
```bash
near view $PAYMENT  get_user_info_by_id '{"user_id": "refferal-1.testnet"}'
near view $PAYMENT get_all_token_id
```

## NFT-Ed25519
```bash
# init
near deploy $NFT ./target/wasm32-unknown-unknown/release/nft_25519.wasm
near call $NFT new_default_meta '{"owner_id": "'$NFT'", "admin_pub_key": "'$PUBKEY'"}' --accountId $NFT

# mint for sponsor
## 1. deposit amount to init course_id
near call $NFT deposit_sponsor '{"course_id": "'$COURSE1'"}' --accountId creator1.testnet --deposit 0.1
## 2. mint 
near call $NFT nft_mint_for_sponsor '{"token_id": "3", "receiver_id": "creator1.testnet", "token_metadata": { "title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "media": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg", "copies": 1}, "course_id": "'$COURSE1'"}' --accountId $ADMIN --deposit 0.01
## 3. get sponsor balance
near view $NFT get_sponsor_balance '{"course_id": "'$COURSE1'", "sponsor_id": "creator1.testnet"}'
## 4. sponsor withdraw
near call $NFT withdraw_sponsor '{"course_id": "'$COURSE1'", "amount": 100000000000}' --accountId creator1.testnet

# mint with signature
near call $NFT nft_mint_with_signature '{"token_id": "3", "receiver_id": "collab_1.testnet", "token_metadata": { "title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "media": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg", "copies": 1}, "signature_base64": "BfGtrma4UjoZ+QsqQElj+qU7tXGInTy4BTUWqYGTH6qurNYKz+BE9cili5ekeBZhD5sm5D/+GbTh8XmiisDBA==", "course_id": "cardano-cert-2"}' --accountId collab_1.testnet --deposit 0.015

# mint by admin
near call $NFT nft_mint '{"token_id": "3", "receiver_id": "collab_1.testnet", "token_metadata": { "title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "media": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg", "copies": 1}}' --accountId $ADMIN --deposit 0.01 

```

## Launchpad

```bash
near deploy $LAUCNHPAD ./target/wasm32-unknown-unknown/release/launchpad.wasm
near call $LAUNCHPAD init --accountId $ADMIN

# Add a new token
near call $LAUNCHPAD add_token '{"token_id": "'$FT'"}' --accountId $ADMIN

# Change admin
near call $LAUNCHPAD change_admin '{"new_admin": "new-admin.testnet"}' --accountId $ADMIN

# Delete a token by token ID
near call $LAUNCHPAD delete_token_by_token_id '{"token_id": "token-1.testnet"}' --accountId $ADMIN

# Initialize a new pool
near call $LAUNCHPAD init_pool '{"campaign_id": "campaign-1", "token_id": "'$FT'", "min_multiple_pledge": 100, "time_start_pledge": 1633046400000000000, "time_end_pledge": 1633132800000000000, "target_funding": "1000000"}' --accountId $ADMIN --deposit 1

# Admin set status pool pre-funding
near call $LAUNCHPAD admin_set_status_pool_pre_funding '{"pool_id": 1, "approve": true}' --accountId $ADMIN

# Backer deposit
near call $FT ft_transfer_call '{"receiver_id": "'$LAUNCHPAD'", "amount": "1000000000000000000000000", "msg": "'$POOL_ID'"}' --accountId $BACKER --depositYocto 1

# Change pool funding time
near call $LAUNCHPAD change_pool_funding_time '{"pool_id": 1, "time_start_pledge": 1633046400000000000, "time_end_pledge": 1633132800000000000}' --accountId $ADMIN

# Set minimum staking amount
near call $LAUNCHPAD set_min_staking_amount '{"amount": "1000000000000000000000000"}' --accountId $ADMIN

# Set refund percent for rejected pool
near call $LAUNCHPAD set_refund_reject_pool '{"percent": 10}' --accountId $ADMIN

# Cancel a pool
near call $LAUNCHPAD cancel_pool '{"pool_id": 1}' --accountId $ADMIN

# Withdraw to creator
near call $LAUNCHPAD withdraw_to_creator '{"pool_id": 1, "amount": "500000"}' --accountId $ADMIN

# Check funding result
near call $LAUNCHPAD check_funding_result '{"pool_id": 1, "is_waiting_funding": false}' --accountId $ADMIN

# Claim refund
near call $LAUNCHPAD claim_refund '{"pool_id": 1}' --accountId $USER

# Update pool status
near call $LAUNCHPAD update_pool_status '{"pool_id": 1, "status": "CLOSED"}' --accountId $ADMIN

# Creator accept voting
near call $LAUNCHPAD creator_accept_voting '{"pool_id": 1, "approve": true}' --accountId $ADMIN


# Get all pools
near view $LAUNCHPAD get_all_pool
# This command retrieves all pools available in the launchpad.

# Get pools by status
near view $LAUNCHPAD get_pools_by_status '{"status_str": "FUNDING"}'
# This command retrieves pools filtered by the specified status.

# Get detailed information of a specific pool
near view $LAUNCHPAD get_detail_pool '{"pool_id": 1}'
# This command retrieves detailed information about a specific pool using its pool ID.

# Get balance of the creator for a specific pool
near view $LAUNCHPAD get_balance_creator '{"pool_id": 1}'
# This command retrieves the balance of the creator for a specific pool.

# Get refund percentage for rejected pools
near view $LAUNCHPAD get_refund_reject_pool
# This command retrieves the refund percentage set for rejected pools.

# Get minimum staking amount
near view $LAUNCHPAD get_min_staking_amount
# This command retrieves the minimum staking amount required for pools.

# Get user records by pool ID
near view $LAUNCHPAD get_user_records_by_pool_id '{"pool_id": 1}'
# This command retrieves user records associated with a specific pool ID.

```