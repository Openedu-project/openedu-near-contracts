#!/bin/bash
set -e
cd "`dirname $0`"/../ft_token
cargo build --all --target wasm32-unknown-unknown --release
cd ..
cd "`dirname $0`"/../payment
cargo build --all --target wasm32-unknown-unknown --release
cd ..
cd "`dirname $0`"/../launchpad
cargo build --all --target wasm32-unknown-unknown --release
cd ..

# Create the directory if it doesn't exist
mkdir -p ./res/

cp ./target/wasm32-unknown-unknown/release/*.wasm ./res/