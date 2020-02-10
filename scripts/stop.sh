#!/bin/bash

kind delete cluster --name $CLUSTER_NAME
docker system prune --all --volumes -f