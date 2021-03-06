rustup override set 1.58.0
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release

cp ./target/wasm32-wasi/release/grayscale_lib.wasm ../../image-api-rs/lib/grayscale_lib_origin.wasm
cp ./target/wasm32-wasi/release/grayscale_lib.wasm ../../image-api-go/lib/grayscale_lib_origin.wasm
wasmedgec ./target/wasm32-wasi/release/grayscale.wasm ./target/wasm32-wasi/release/grayscale.wasm
wasmedgec ./target/wasm32-wasi/release/grayscale_lib.wasm ./target/wasm32-wasi/release/grayscale_lib.wasm
cp ./target/wasm32-wasi/release/grayscale.wasm ../../image-api-rs/lib
cp ./target/wasm32-wasi/release/grayscale_lib.wasm ../../image-api-go/lib
echo -e "finished build functions/grayscale ..."
