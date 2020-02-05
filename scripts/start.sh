#!/bin/bash

kind create --config infrastructure/kind-config.yaml cluster --name $CLUSTER_NAME

devspace use namespace crd-test

kubectl create namespace infra
helm dependencies update infrastructure/helm
helm upgrade -i infra --namespace infra infrastructure/helm
