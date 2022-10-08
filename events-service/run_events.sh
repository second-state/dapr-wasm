#!/usr/bin/env bash

cargo build --target wasm32-wasi --release
wasmedgec target/wasm32-wasi/release/events_service.wasm events_service.wasm
dapr stop events-service
dapr run --app-id events-service \
        --app-protocol http \
        --app-port 9007 \
        --dapr-http-port 3505 \
        --log-level debug \
        --components-path ../config \
        wasmedge events_service.wasm
