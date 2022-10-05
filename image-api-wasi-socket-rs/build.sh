#!/usr/bin/env bash
rustup override set 1.58.0
rustup target add wasm32-wasi
cargo build --target wasm32-wasi
wasmedgec ./target/wasm32-wasi/debug/image-api-wasi-socket-rs.wasm ./target/wasm32-wasi/debug/image-api-wasi-socket-rs-opt.wasm