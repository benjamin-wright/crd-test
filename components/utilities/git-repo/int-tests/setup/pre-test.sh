#!/bin/sh

cp -r /data/test/.ssh ~/.ssh

chmod 0600 ~/.ssh/id_rsa

ssh-keyscan -H git-repo >> ~/.ssh/known_hosts
# ssh-copy-id $TEST_USER@git-repo

# echo "Host *\n    StrictHostKeyChecking no" >> ~/.ssh/config
