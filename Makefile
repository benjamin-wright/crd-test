.PHONY: build start stop start-k8s install clean images

default:
	echo "  ----  CRD-TEST  ----"
	echo ""
	echo " images    - build dev images"
	echo " start     - spin up everything from a standing start"
	echo " stop      - tear down EVERYTHING"
	echo " start-k8s - create a new k8s cluster"
	echo " install   - install the helm chart"
	echo " clean     - tear down the helm chart"

build:
	cd minion-controller && make build

images:
	docker build -t rust ./images/dev

start: start-k8s install

stop:
	kind delete cluster --name crd-test

start-k8s:
	kind create cluster --name crd-test

install:
	helm install minion helm

clean:
	helm delete minion
