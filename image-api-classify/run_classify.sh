#!/usr/bin/env bash

cargo build --target wasm32-wasi --release
dapr stop image-api-classify
dapr run --app-id image-api-classify \
        --app-protocol http \
        --app-port 9006 \
        --dapr-http-port 3504 \
        --log-level debug \
        --components-path ../config \
        wasmedge-tensorflow-lite target/wasm32-wasi/release/wasmedge_hyper_server_tflite.wasm
