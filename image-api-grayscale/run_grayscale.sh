#!/usr/bin/env bash

rustup override set 1.58.0
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release

dapr stop image-api-grayscale
dapr run --app-id image-api-grayscale \
        --app-protocol http \
        --app-port 9005 \
        --dapr-http-port 3503 \
        --components-path ../config \
        --log-level debug \
	wasmedge ./target/wasm32-wasi/release/image-api-grayscale.wasm

