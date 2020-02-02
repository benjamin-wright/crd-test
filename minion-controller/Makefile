.PHONY: build

build:
	docker build -t minion-controller .
	kind load docker-image minion-controller

build-local:
	cargo build --release