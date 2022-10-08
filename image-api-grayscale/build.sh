#!/usr/bin/env bash
rustup override set 1.58.0
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release
wasmedgec ./target/wasm32-wasi/release/image-api-grayscale.wasm ./target/wasm32-wasi/release/image-api-grayscale-opt.wasm