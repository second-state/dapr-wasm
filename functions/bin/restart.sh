#!/usr/bin/env bash

dapr list

pushd image-api-wasi-socket-rs
nohup ./run_api_wasi_socket_rs.sh > nohup.log & 
popd

pushd web-port
nohup ./run_web.sh > nohup.log & 
popd

