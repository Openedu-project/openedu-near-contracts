# Contracts for testnet 

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
near deploy $NFT ./target/wasm32-unknown-unknown/release/nft_25519.wasm
```