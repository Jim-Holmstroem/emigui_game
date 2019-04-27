#!/usr/bin/env bash
set -eu

rustup target add wasm32-unknown-unknown
if ! [[ $(wasm-bindgen --version) ]]; then
	cargo clean
	cargo install -f wasm-bindgen-cli
	cargo update
fi

BUILD=debug
# BUILD=release

rm -rf docs/*.wasm

cargo build --target wasm32-unknown-unknown

FOLDER_NAME=${PWD##*/}
TARGET_NAME="game.wasm"
wasm-bindgen \
    "target/wasm32-unknown-unknown/$BUILD/$TARGET_NAME" \
    --out-dir out \
    --no-modules \
    --no-typescript
