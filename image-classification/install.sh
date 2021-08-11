sudo apt-get update
sudo apt-get install -y libjpeg-dev libpng-dev

wget -q https://raw.githubusercontent.com/dapr/cli/master/install/install.sh -O - | /bin/bash

wget https://github.com/second-state/WasmEdge-go/releases/download/v0.8.1/install_wasmedge.sh
sudo bash ./install_wasmedge.sh /usr/local
wget https://github.com/second-state/WasmEdge-go/releases/download/v0.8.1/install_wasmedge_tensorflow_deps.sh
wget https://github.com/second-state/WasmEdge-go/releases/download/v0.8.1/install_wasmedge_tensorflow.sh
sudo bash ./install_wasmedge_tensorflow_deps.sh /usr/local
sudo bash ./install_wasmedge_tensorflow.sh /usr/local

wget https://github.com/second-state/WasmEdge-go/releases/download/v0.8.1/install_wasmedge_image.sh
sudo bash ./install_wasmedge_image.sh /usr/local

FILE="./wasmedgec-tensorflow"
if [ ! -f "$FILE" ]; then
    echo "install $FILE ..."
    curl -L https://github.com/second-state/WasmEdge-tensorflow-tools/releases/download/0.8.2-rc2/WasmEdge-tensorflow-tools-0.8.2-rc2-manylinux2014_x86_64.tar.gz -o ./WasmEdge-tensorflow-tools-0.8.2-rc2-manylinux2014_x86_64.tar.gz
    tar xzvf WasmEdge-tensorflow-tools-0.8.2-rc2-manylinux2014_x86_64.tar.gz wasmedge-tensorflow-lite
    tar xzvf WasmEdge-tensorflow-tools-0.8.2-rc2-manylinux2014_x86_64.tar.gz wasmedgec-tensorflow
    rm WasmEdge-tensorflow-tools-0.8.2-rc2-manylinux2014_x86_64.tar.gz

    curl -L https://github.com/second-state/WasmEdge-tensorflow-deps/releases/download/0.8.0/WasmEdge-tensorflow-deps-TFLite-0.8.0-manylinux2014_x86_64.tar.gz -o ./WasmEdge-tensorflow-deps-TFLite-0.8.0-manylinux2014_x86_64.tar.gz
    tar xzvf WasmEdge-tensorflow-deps-TFLite-0.8.0-manylinux2014_x86_64.tar.gz
    rm WasmEdge-tensorflow-deps-TFLite-0.8.0-manylinux2014_x86_64.tar.gz
fi

rustup target add wasm32-wasi
rustup override set 1.50.0

cp wasmedge-tensorflow-lite ../image-api-rs/lib
cp libtensorflowlite_c.so ../image-api-rs/lib

cp wasmedge-tensorflow-lite ../image-api-go/lib
cp libtensorflowlite_c.so ../image-api-go/lib


