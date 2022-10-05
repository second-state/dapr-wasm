
pre-install:
	cd functions/bin && ./install.sh

build-wasm:
	rustup target add wasm32-wasi
	cd functions/grayscale && ./build.sh 
	cd functions/classify && ./build.sh 

build-api-wasi-socket-rs:
	cd image-api-wasi-socket-rs && ./build.sh
run-api-wasi-socket-rs:
	cd image-api-wasi-socket-rs && ./run_api_wasi_socket_rs.sh

build-web:
	cd web-port; go build
run-web:
	cd web-port; ./run_web.sh

build: build-wasm build-api-wasi-socket-rs build-web

