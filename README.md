# Dapr and WasmEdge

[Tutorial video](https://www.youtube.com/watch?v=3v37pAT9iK8)

## Introduction

This is a template application to showcase how [Dapr](https://dapr.io/) and [WasmEdge](https://github.com/WasmEdge/) work together to support [lightweight WebAssembly-based microservices](https://github.com/second-state/microservice-rust-mysql) in a cloud-native environment. The microservices are all written in Rust and compiled into WebAssembly. They run inside the WasmEdge Runtime [as opposed to Linux containers](https://wasmedge.org/wasm_linux_container/) or VMs for these reasons.

> While this demo is done in Rust, WasmEdge can also run [Node.js compatible JavaScript](https://wasmedge.org/book/en/write_wasm/js.html) applications. 
 
This application consists of 3 microservices and a [standalone web page](docs) that enables users to interact with the microservices using a HTML+JavaScript UI. It is a very typical JAMstack setup. Each microservice is attached to a Dapr sidecar, which provides a suite of useful services commonly required by cloud-native microservices. The overall architecture is as follows.

<img src="docs/dapr-wasmedge.png" alt="Microservices architecture" width="480" style="text-align: center; margin: 0 auto;"/>

The Rust version of [Dapr SDK for WasmEdge](https://github.com/second-state/dapr-sdk-wasi) is used to access Dapr sidecars from the microservice apps. Specifically, the [grayscale](https://github.com/second-state/dapr-wasm/tree/main/image-api-grayscale) microservice takes an image from an HTTP POST, turns it into grayscale, and returns the result image data in the HTTP response. 

* It uses Dapr to discover and invoke the [events](https://github.com/second-state/dapr-wasm/tree/main/events-service) microservice to record every successful user request. 
* It also stores each user’s IP address and last timestamp data in its Dapr sidecar’s state database. That allows the service to rate limit users if needed. 

The [classify](https://github.com/second-state/dapr-wasm/tree/main/image-api-classify) microservices takes an image from an HTTP POST, runs a Tensorflow model against it to classify the object on the image, and returns the result as a text label in the HTTP response. You can learn more about AI inference in Rust and WasmEdge [here](https://wasmedge.org/book/en/write_wasm/rust/wasinn.html). It uses its own Dapr sidecar the same way as the [grayscale](https://github.com/second-state/dapr-wasm/tree/main/image-api-grayscale) microservice. 

The [events](https://github.com/second-state/dapr-wasm/tree/main/events-service) microservice takes JSON data from a HTTP POST and saves it to an external MySQL database for later analysis. 

* It uses Dapr to make itself discoverable by name by other microservices that need to record events. 
* It also uses its Dapr sidecar to store secrets such as the MySQL database credentials.

Now, go ahead and fork this repo. Create and deploy your own lightweight microservices for better security, faster performance, and smaller footprints. 

## Build and deploy these microservices in Dapr

You will need install the following software toolchain to run these examples. The detailed steps are shown in the [GitHub Actions script](.github/workflows/main.yml).

* [Install the Dapr CLI](https://docs.dapr.io/getting-started/install-dapr-cli/)
* [Install the WasmEdge Runtime](https://wasmedge.org/docs/develop/build-and-run/install)
* [Install Rust](https://www.rust-lang.org/tools/install)
* Install the [MySQL](https://dev.mysql.com/doc/mysql-installation-excerpt/5.7/en/) or [MariaDB](https://mariadb.com/kb/en/getting-installing-and-upgrading-mariadb/) or [TiDB](https://docs.pingcap.com/tidb/dev/quick-start-with-tidb) databases

Start the database and place the connection string in the [config/secrets.json](config/secrets.json) file under `DB_URL:MYSQL`. Next, start Dapr with the following commands.

```bash
dapr init
```

### The image grayscale microservice

Build.

```bash
cd image-api-grayscale
cargo build --target wasm32-wasi --release
wasmedge compile ./target/wasm32-wasi/release/image-api-grayscale.wasm image-api-grayscale.wasm
```

Deploy.

```bash
dapr run --app-id image-api-grayscale \
        --app-protocol http \
        --app-port 9005 \
        --dapr-http-port 3503 \
        --components-path ../config \
        --log-level debug \
	wasmedge image-api-grayscale.wasm
```

### The image classification microservice

Build.

```bash
cd image-api-classify
cargo build --target wasm32-wasi --release
wasmedge compile target/wasm32-wasi/release/wasmedge_hyper_server_tflite.wasm wasmedge_hyper_server_tflite.wasm
```

Deploy.

```bash
dapr run --app-id image-api-classify \
        --app-protocol http \
        --app-port 9006 \
        --dapr-http-port 3504 \
        --log-level debug \
        --components-path ../config \
        wasmedge wasmedge_hyper_server_tflite.wasm
```

### The events recorder microservice

Build.

```bash
cd events-service
cargo build --target wasm32-wasi --release
wasmedge compile target/wasm32-wasi/release/events_service.wasm events_service.wasm
```

Deploy.

```bash
dapr run --app-id events-service \
        --app-protocol http \
        --app-port 9007 \
        --dapr-http-port 3505 \
        --log-level debug \
        --components-path ../config \
        wasmedge events_service.wasm
```

### Test

You can use the [static web page UI](http://dapr-demo.secondstate.co/) or `curl` to test the services.

Initialize the events database table.

```bash
$ curl http://localhost:9007/init
{"status":true}

$ curl http://localhost:9007/events
[]
```

Use the grayscale microservice. The return data is base64 encoded grayscale image.

```bash
$ cd docs
$ curl http://localhost:9005/grayscale -X POST --data-binary '@food.jpg'
ABCDEFG ...
```

Use the image classification microservice.

```bash
$ cd docs
$ curl http://localhost:9006/classify -X POST --data-binary '@food.jpg'
hotdog is detected with 255/255 confidence
```

Query the events database again.

```bash
$ curl http://localhost:9007/events
[{"id":1,"event_ts":1665358852918,"op_type":"grayscale","input_size":68016},{"id":2,"event_ts":1665358853114,"op_type":"classify","input_size":68016}]
```

## Learn more

* Why WebAssembly as a cloud native runtime: https://wasmedge.org/wasm_linux_container/
* Create a high-performance HTTP server in WasmEdge: https://github.com/WasmEdge/wasmedge_hyper_demo/tree/main/server
* Create a web service client in WasmEdge: https://github.com/WasmEdge/wasmedge_reqwest_demo
* Create a database client in WasmEdge: https://github.com/WasmEdge/wasmedge-db-examples
* Dapr SDK for WasmEdge and examples: https://github.com/second-state/dapr-sdk-wasi
* Using container tools (Kubernetes, Docker, Podman etc) to manage WadmEdge apps: https://wasmedge.org/book/en/use_cases/kubernetes.html
* Run JavaScript and Node.js apps in WasmEdge: https://wasmedge.org/book/en/write_wasm/js.html
