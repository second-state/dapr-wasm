

## Install script

```bash
wget https://github.com/second-state/WasmEdge-go/releases/download/v0.8.1/install_wasmedge.sh
sudo bash ./install_wasmedge.sh /usr/local
wget https://github.com/second-state/WasmEdge-go/releases/download/v0.8.1/install_wasmedge_tensorflow_deps.sh
wget https://github.com/second-state/WasmEdge-go/releases/download/v0.8.1/install_wasmedge_tensorflow.sh
sudo bash ./install_wasmedge_tensorflow_deps.sh /usr/local
sudo bash ./install_wasmedge_tensorflow.sh /usr/local
sudo apt-get update
sudo apt-get install -y libjpeg-dev libpng-dev
wget https://github.com/second-state/WasmEdge-go/releases/download/v0.8.1/install_wasmedge_image.sh
sudo bash ./install_wasmedge_image.sh /usr/local

rustup target add wasm32-wasi
```