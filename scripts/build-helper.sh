#!/bin/bash
set -eox pipefail


echo ">> Building contract: helper-contract"

rustup target add wasm32-unknown-unknown
cargo build -p helper-contract --target wasm32-unknown-unknown --profile=contract

cp ./target/wasm32-unknown-unknown/contract/helper_contract.wasm res/helper_contract.wasm
