rustup override set 1.58.0
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release

cp ./target/wasm32-wasi/release/classify.wasm ../../image-api-go/lib/classify.wasm
echo -e "finished build functions/classify ..."
