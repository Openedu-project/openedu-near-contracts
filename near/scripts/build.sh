#!/bin/bash
set -e
cd "`dirname $0`"/../ft_token
cargo build --all --target wasm32-unknown-unknown --release
cd ..
cd "`dirname $0`"/../faucet
cargo build --all --target wasm32-unknown-unknown --release
cd ..
cd "`dirname $0`"/../joy_v1
cargo build --all --target wasm32-unknown-unknown --release
cd ..
cp ./target/wasm32-unknown-unknown/release/*.wasm ./res/