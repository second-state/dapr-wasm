# TFLite server example

## Prequsites

In order to run this example, you will need to install [wasmedge-tensorflow-lite](https://github.com/second-state/WasmEdge-tensorflow-tools) like this.

```
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash -s -- -e all
```

## Build

```
cargo build --target wasm32-wasi --release
```

## Run

```
wasmedge-tensorflow-lite target/wasm32-wasi/release/wasmedge_hyper_server_tflite.wasm
```

## Test

Run the following from another terminal.

```
$ curl http://localhost:8080/classify -X POST --data-binary "@grace_hopper.jpg"
military uniform is detected with 206/255 confidence
```
