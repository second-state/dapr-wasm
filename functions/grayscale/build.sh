cargo build --target wasm32-wasi --release

cp ./target/wasm32-wasi/release/grayscale.wasm  ../../image-api-rs/lib
echo -e "finished build functions/grayscale ..."