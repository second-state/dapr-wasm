rustup target add wasm32-wasi
rustup override set 1.50.0
rustwasmc  build --enable-ext
cp ./pkg/classify_bg.wasm ../../image-api-go/lib/classify_bg.wasm
echo "finished build functions/classify ...\n"