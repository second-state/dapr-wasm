
pre-install:
	cd functions/bin && sudo ./install.sh

build-wasm:
	rustup target add wasm32-wasi
	cd functions/grayscale && ./build.sh 
	cd functions/classify && ./build.sh 

build-api-go:
	cd image-api-go && go build --tags "tensorflow image"
run-api-go:
	cd image-api-go && ./run_api_go.sh

build-api-rs:
	cd image-api-rs && cargo build 
run-api-rs:
	cd image-api-rs && ./run_api_rs.sh

build-web:
	cd web-port; go build
run-web:
	cd web-port; ./run_web.sh

build: build-wasm build-api-go build-api-rs build-web

