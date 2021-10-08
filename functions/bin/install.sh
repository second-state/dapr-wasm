sudo apt-get update
sudo apt-get install -y libjpeg-dev libpng-dev

wget -q https://raw.githubusercontent.com/dapr/cli/master/install/install.sh -O - | /bin/bash
## require docker run in non-root user, refer to https://docs.docker.com/engine/install/linux-postinstall/
dapr init

wget -qO- https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | sudo bash -s -- -e all


#wget https://github.com/second-state/WasmEdge-go/releases/download/v0.8.2/install_wasmedge.sh
#sudo bash ./install_wasmedge.sh /usr/local


#wget https://github.com/second-state/WasmEdge-go/releases/download/v0.8.2/install_wasmedge_tensorflow_deps.sh
#wget https://github.com/second-state/WasmEdge-go/releases/download/v0.8.2/install_wasmedge_tensorflow.sh
#sudo bash ./install_wasmedge_tensorflow_deps.sh /usr/local
#sudo bash ./install_wasmedge_tensorflow.sh /usr/local

#wget https://github.com/second-state/WasmEdge-go/releases/download/v0.8.2/install_wasmedge_image.sh
#sudo bash ./install_wasmedge_image.sh /usr/local


#curl https://raw.githubusercontent.com/second-state/rustwasmc/master/installer/init.sh -sSf | sh

