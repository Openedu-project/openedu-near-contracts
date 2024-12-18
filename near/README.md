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
# Admin add author & add token 
near call $LAUNCHPAD add_token '{"token_id": "'$TOKEN_ID'"}' --accountId $ADMIN
near call $LAUNCHPAD change_admin '{"new_admin": "''"}' --accountId $ADMIN
near call $LAUNCHPAD delete_token_by_token_id '{"token_id": ""}' --accountId $ADMIN


```