## Instructions to new features

**🚩New:** Calling Dapr API within WasmEdge Sandbox Environment


[Dapr state API document](https://docs.dapr.io/getting-started/get-started-api/)

***note:*** If HTTPS URL is used, make sure *wasmedge_ssl* feature is enabled in *wasmedge_http_req*  and the *httpsreq* plugin also needs to be [installed](https://github.com/second-state/wasmedge_http_req#https-support).  Otherwise, disable the *wasmedge_ssl* feature in [*wasmedge_http_req*](https://github.com/second-state/wasmedge_http_req/blob/master/Cargo.toml#L17).

### 1. Build
```bash
cargo build --target wasm32-wasi
sh build.sh
```

### 2. Start the WasmEdge microservice

```bash
sh run_api_wasi_socket_rs.sh
```
OR

```bash
cd image-api-wasi-socket-rs
dapr run --app-id image-api-wasi-socket-rs \
         --app-protocol http \
         --app-port 9005 \
         --dapr-http-port 3503 \
         --components-path ../config \
         --log-level debug \
	 wasmedge ./target/wasm32-wasi/debug/image-api-wasi-socket-rs.wasm
```

### 3. Save a new State Object to Dapr
```bash
curl -X POST -H "Content-Type: application/json" -d '[{ "key": "name", "value": "Bruce Wayne"}]' http://localhost:9005/v1.0/state/statestore
```
