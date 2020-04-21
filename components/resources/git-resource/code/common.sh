#!/bin/sh

set -o errexit -o pipefail

function ssh_setup() {
    local ssh_location=$1
    local host=$2

    echo "copying ssh keys..."
    mkdir -p ~/.ssh
    cp $ssh_location/* ~/.ssh
    chmod 600 ~/.ssh/id_rsa

    echo "updating known_hosts.."
    ssh-keyscan -H $host >> ~/.ssh/known_hosts
}

function ensure_env() {
    local name=$1

    if [[ "$(eval echo \$$name)" == "" ]]; then
        echo "No value found for required variable \"$name\""
        exit 1
    fi
}

ensure_env REPO_HOST
ensure_env REPO
ensure_env BRANCH

ssh_setup /data/ssh $REPO_HOST