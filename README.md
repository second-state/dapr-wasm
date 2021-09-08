## [Live Demo](http://23.100.38.125/static/home.html)
## 1. Introduction

[DAPR](https://dapr.io/) is a portable, event-driven runtime that makes it easy for any developer to build resilient, stateless and stateful applications that run on the cloud and edge and embraces the diversity of languages and developer frameworks. It's a Microsoft-incubated [open-source](https://github.com/dapr/dapr) project.

[WasmEdge](https://github.com/WasmEdge/WasmEdge) is a open-source, high-performance, extensible, and hardware optimized WebAssembly Virtual Machine for automotive, cloud, AI, and blockchain applications.

In this demonstration App, we create two image processing web services, integrated with Dapr.
This project is built to demonstrate how to use Dapr to integrate Web applications in any programming language, and how WasmEdge can be embed in Go and Rust applications.

## 2. Architecture

This project contains mainly three components:

* The [Web port service](./web-port)

It is a simple Go Web application which is exposed as an endpoint of the whole application.
It will render a static HTML page for the user to upload an image, and receive the image from the user, redirect request to internal image APIs.

* The [image service in Golang](./image-api-go)

This Dapr service is written in Golang. It uses `WASI` to call a prebuild wasm file to classify an image using a Tensorflow model.

* The [image service in Rust](./image-api-rs)

This Dapr service is written in Rust. It simply starts a new process for the WasmEdge VM to run and perform grayscale on a image.

![doc](./doc/dapr-wasmedge.png)

## 3. Prerequisites

* [Install Golang](https://golang.org/doc/install)
* [install Rust](https://www.rust-lang.org/en-US/install.html)
* [Install Dapr](https://docs.dapr.io/getting-started/)
* [Install WasmEdge](https://github.com/WasmEdge/WasmEdge/blob/master/docs/install.md)


## 4. Build

```bash
make pre-install  ## Install WasmEdge dependences
make build        ## Will build all the components

## If you modify the wasm functions project,
## Use the commands in ./functions/grayscale/build.sh 
## and ./functions/classify/build.sh to generate new compiled files
make build-wasm
```
## 5. Run

To simplify the deployment, we provide a script to run the services:

```bash
make run-api-go ## Run the image-api-go
make run-api-rs ## Run the image-api-rs
make run-web ## Run the Web port service
```

For each component, you can also run it individually:
### Start the web-port service

```bash
cd web-port
dapr run --app-id go-web-port \
         --app-protocol http \
         --app-port 8080 \
         --dapr-http-port 3500 \
         --components-path ../config \
         --log-level debug \
         ./web-port
```

### Start the image-api-go service

```bash
cd image-api-go
dapr run --app-id image-api-go \
         --app-protocol http \
         --app-port 9003 \
         --dapr-http-port 3501 \
         --log-level debug \
         --components-path ../config \
         ./image-api-go
```

### Start the image-api-rust service

```bash
cd image-api-rs
dapr run --app-id image-api-rs \
         --app-protocol http \
         --app-port 9004 \
         --dapr-http-port 3502 \
         --components-path ../config \
         --log-level debug \
         ./target/debug/image-api-rs
```

After all the services started, we can use this command to verify:

```bash
dapr list
```
```
  APP ID        HTTP PORT  GRPC PORT  APP PORT  COMMAND               AGE  CREATED              PID
  go-web-port   3500       44483      8080      ./web-port            15m  2021-08-26 12:19.59  270961
  image-api-rs  3502       41661      9004      ./target/release/...  9m   2021-08-26 12:25.27  285749
  image-api-go  3501       34291      9003      ./image-api-go        9m   2021-08-26 12:25.27  285852
```
## 6. [Online Demo: Dapr-WasmEdge](http://23.100.38.125/static/home.html)

![](./doc/demo.png)
## 7. Appendix: an introduction to Dapr SDK

Dapr provides [SDKs](https://docs.dapr.io/developing-applications/sdks/) for different programming languages. Using the SDKs is the easiest way to run your applications in Dapr.

The SDK contains Client, Service, and Runtime API, and it is easy to use. For example, we use Service SDK in `go-sdk` to create the `image-api-go` service

```go
func main() {
	s := daprd.NewService(":9003")

	if err := s.AddServiceInvocationHandler("/api/image", imageHandlerWASI); err != nil {
		log.Fatalf("error adding invocation handler: %v", err)
	}

	if err := s.Start(); err != nil && err != http.ErrServerClosed {
		log.Fatalf("error listenning: %v", err)
	}
}
```

In `web-port/web_port.go`, we use Dapr's Client to send request to Service:

```go
func daprClientSend(image []byte, w http.ResponseWriter) {
	ctx := context.Background()

	// create the client
	client, err := dapr.NewClient()
	if err != nil {
		panic(err)
	}

	content := &dapr.DataContent{
		ContentType: "text/plain",
		Data:        image,
	}

	resp, err := client.InvokeMethodWithContent(ctx, "image-api-go", "/api/image", "post", content)
	if err != nil {
		panic(err)
	}
	log.Printf("dapr-wasmedge-go method api/image has invoked, response: %s", string(resp))
	fmt.Printf("Image classify result: %q\n", resp)
	w.WriteHeader(http.StatusOK)
	fmt.Fprintf(w, "%s", string(resp))
}
```

For any Web Service which don't use Dapr SDK but registered as a Dapr instance, we can still can use `http` or `gRpc` to interact with it. Dapr will start a `sidecar` for each service instance. Essentially, `sidecar` works as a proxy for a service instance. We send request to `sidecar`, then the request is forwarded to the service instance. For example, in `web-port/web_port.go` we send a request to Rust api like this(3502 is the port of Sidecar):

```go
client := &http.Client{}
	// http://localhost:<daprPort>/v1.0/invoke/<appId>/method/<method-name>
	req, err := http.NewRequest("POST", "http://localhost:3502/v1.0/invoke/image-api-rs/method/api/image", bytes.NewBuffer(image))
	if err != nil {
		panic(err)
	}
	req.Header.Set("Content-Type", "text/plain")
	resp, _ := client.Do(req)
	defer resp.Body.Close()
	body, _ := ioutil.ReadAll(resp.Body)
	fmt.Fprintf(w, "%s", body)
```
