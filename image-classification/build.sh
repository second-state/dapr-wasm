rustwasmc  build --enable-ext
cp ./pkg/classify_lib_bg.wasm ../image-api-go/lib

cargo build --target wasm32-wasi

 ./wasmedgec-tensorflow --generic-binary ./target/wasm32-wasi/debug/classify_bin.wasm  classify.so
cp ./classify.so ../image-api-rs/lib
cp ./classify.so ../image-api-go/lib

echo "finished build image-classify ..."