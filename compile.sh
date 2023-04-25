#!/usr/bin/env bash

set -e

rm counter/Cargo.lock || true
rm -rf counter/target || true
rm faucet/Cargo.lock || true
rm -rf faucet/target || true
rm nft/Cargo.lock || true
rm -rf nft/target || true
rm confidential_faucet/Cargo.lock || true
rm -rf confidential_faucet/target || true

cd counter
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/counter.wasm ../dist

cd ../faucet
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/faucet.wasm ../dist

cd ../nft
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/nft.wasm ../dist

cd ../confidential_faucet
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/confidential_faucet.wasm ../dist

