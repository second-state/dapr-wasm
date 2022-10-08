
pre-install:
	cd functions/bin && ./install.sh

run-api-grayscale:
	cd image-api-grayscale && ./run_grayscale.sh

run-api-classify:
	cd image-api-classify && ./run_classify.sh

build-web:
	cd web-port; go build
run-web:
	cd web-port; ./run_web.sh

build: run-api-grayscale run-api-classify build-web

