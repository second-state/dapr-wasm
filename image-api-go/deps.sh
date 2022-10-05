#!/usr/bin/env bash

set -e

# Install the pre-released library
wget https://github.com/WasmEdge/WasmEdge/releases/download/0.11.1/WasmEdge-0.11.1-manylinux2014_x86_64.tar.gz
tar -xzf WasmEdge-0.11.1-manylinux2014_x86_64.tar.gz
sudo cp WasmEdge-0.11.1-Linux/include/wasmedge/wasmedge.h /usr/local/include
sudo cp WasmEdge-0.11.1-Linux/lib64/libwasmedge.so /usr/local/lib
sudo ldconfig

# Install the prebuilt tensorflow dependencies for the manylinux2014 platforms
wget https://github.com/second-state/WasmEdge-tensorflow-deps/releases/download/0.11.1/WasmEdge-tensorflow-deps-TF-0.11.1-manylinux2014_x86_64.tar.gz
wget https://github.com/second-state/WasmEdge-tensorflow-deps/releases/download/0.11.1/WasmEdge-tensorflow-deps-TFLite-0.11.1-manylinux2014_x86_64.tar.gz
sudo tar -C /usr/local/lib -xzf WasmEdge-tensorflow-deps-TF-0.11.1-manylinux2014_x86_64.tar.gz
sudo tar -C /usr/local/lib -xzf WasmEdge-tensorflow-deps-TFLite-0.11.1-manylinux2014_x86_64.tar.gz
sudo ln -sf libtensorflow.so.2.4.0 /usr/local/lib/libtensorflow.so.2
sudo ln -sf libtensorflow.so.2 /usr/local/lib/libtensorflow.so
sudo ln -sf libtensorflow_framework.so.2.4.0 /usr/local/lib/libtensorflow_framework.so.2
sudo ln -sf libtensorflow_framework.so.2 /usr/local/lib/libtensorflow_framework.so
sudo ldconfig

# Install WasmEdge-tensorflow and WasmEdge-tensorflowlite
wget https://github.com/second-state/WasmEdge-tensorflow/releases/download/0.11.1/WasmEdge-tensorflow-0.11.1-manylinux2014_x86_64.tar.gz
wget https://github.com/second-state/WasmEdge-tensorflow/releases/download/0.11.1/WasmEdge-tensorflowlite-0.11.1-manylinux2014_x86_64.tar.gz
sudo tar -C /usr/local/ -xzf WasmEdge-tensorflow-0.11.1-manylinux2014_x86_64.tar.gz
sudo tar -C /usr/local/ -xzf WasmEdge-tensorflowlite-0.11.1-manylinux2014_x86_64.tar.gz
sudo ldconfig
