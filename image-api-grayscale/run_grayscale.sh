#!/usr/bin/env bash

cargo build --target wasm32-wasi --release
wasmedgec ./target/wasm32-wasi/release/image-api-grayscale.wasm image-api-grayscale.wasm
dapr stop image-api-grayscale
dapr run --app-id image-api-grayscale \
        --app-protocol http \
        --app-port 9005 \
        --dapr-http-port 3503 \
        --components-path ../config \
        --log-level debug \
	wasmedge image-api-grayscale.wasm

