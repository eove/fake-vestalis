#!/usr/bin/env bash
set -xe
DIR=$(dirname "${BASH_SOURCE[0]}")
source "$DIR/config.sh"

mkdir -p "${SERVER_DIR}"
openssl genrsa -out "${SERVER_PRIVATE_KEY_PATH}" 2048
openssl req -new -key "${SERVER_PRIVATE_KEY_PATH}" -out "${SERVER_CERTIFICATE_REQUEST_PATH}"
openssl x509 -req -in "${SERVER_CERTIFICATE_REQUEST_PATH}"\
  -CA "${CA_CERTIFICATE_PATH}"\
  -CAkey "${CA_PRIVATE_KEY_PATH}"\
  -CAcreateserial\
  -out "${SERVER_CERTIFICATE_PATH}"\
  -extfile "${SERVER_CERTIFICATE_EXTENSION_PATH}"\
  -days 365 -sha256
