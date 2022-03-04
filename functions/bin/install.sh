sudo apt-get update
sudo apt-get install -y libjpeg-dev libpng-dev

wget -q https://raw.githubusercontent.com/dapr/cli/master/install/install.sh -O - | /bin/bash
## require docker run in non-root user, refer to https://docs.docker.com/engine/install/linux-postinstall/
dapr init

wget -qO- https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | sudo bash -s -- -e all -p /usr/local
