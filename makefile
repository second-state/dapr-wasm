
pre-install:
	./install.sh

run-api-grayscale:
	cd image-api-grayscale && ./run_grayscale.sh

run-api-classify:
	cd image-api-classify && ./run_classify.sh

build: run-api-grayscale run-api-classify

