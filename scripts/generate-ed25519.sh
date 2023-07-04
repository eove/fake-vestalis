#!/usr/bin/env bash

set -xe
DIR=$(dirname "${BASH_SOURCE[0]}")
source "$DIR/config.sh"

openssl genpkey -algorithm Ed25519 -out ${KEYS_DIR}/ed25519key.pem
openssl pkey -in ${KEYS_DIR}/ed25519key.pem -pubout -out ${KEYS_DIR}/ed25519key.pub

