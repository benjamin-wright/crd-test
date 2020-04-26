#!/bin/bash

NAMESPACE="crd-test"

function wait-for-kind() {
    local ready="false";
    echo 'waiting for kind to spin up...';

    while [ "$ready" == "false" ]; do
        sleep 0.5;
        ready="true"

        local podStatuses=$(kubectl get pods --namespace=kube-system -o=json | jq ".items[].status.containerStatuses[0].ready")
        local numPods=$(kubectl get pods --namespace=kube-system -o=json | jq ".items | length")

        if [ "$numPods" != "8" ]; then
            ready="false";
        fi

        for status in $podStatuses; do
            if [ "$status" != "true" ]; then
                ready="false";
            fi
        done
    done

    echo "finished!";
}

files=$(find . -name .devspace)
for file in $files; do
    echo "removing temp dir: $file"
    rm -r $file;
done

echo "CREATING KIND CLUSTER"
kind create --config infrastructure/kind-config.yaml cluster --name $CLUSTER_NAME
wait-for-kind

echo "SETTING UP DEV NAMESPACE"
kubectl create namespace $NAMESPACE
devspace use namespace $NAMESPACE

echo "DEPLOYING BASE INFRASTRUCTURE"
kubectl create namespace infra
helm dep update infrastructure/helm
helm upgrade -i --wait infra --namespace infra infrastructure/helm

echo "DEPLOYING CRDS"
devspace run install-crds

echo "PUBLISHED SHARED NPM MODULES"
(cd components/utilities/npm-modules; npm-build)