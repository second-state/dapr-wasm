rustup override set 1.50.0
rustwasmc  build --enable-ext
cp ./pkg/grayscale.wasm ../../image-api-rs/lib
echo -e "finished build functions/grayscale ..."
