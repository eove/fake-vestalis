#!/usr/bin/env bash
set -xe
DIR=$(dirname "${BASH_SOURCE[0]}")
source "$DIR/config.sh"

mkdir -p "${CA_DIR}"
openssl genrsa -des3 -out "${CA_PRIVATE_KEY_PATH}" 2048
openssl req -x509 -new -nodes -key "${CA_PRIVATE_KEY_PATH}" -sha256 -days 365 -out "${CA_CERTIFICATE_PATH}"
