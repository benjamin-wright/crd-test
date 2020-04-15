#!/bin/bash

kind delete cluster --name $CLUSTER_NAME

files=$(find . -name .devspace)
for file in $files; do
    echo "removing temp dir: $file"
    rm -r $file;
done