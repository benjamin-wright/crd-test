#!/bin/bash

NAMESPACE="crd-test"

function wait-for-kind() {
    local ready="false";

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

kind create --config infrastructure/kind-config.yaml cluster --name $CLUSTER_NAME

wait-for-kind

devspace use namespace $NAMESPACE

kubectl create namespace infra
helm dependencies update infrastructure/helm
helm upgrade -i --wait infra --namespace infra infrastructure/helm

rm -rf .devspace
