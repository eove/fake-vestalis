#!/usr/bin/env bash
SCRIPT_DIR=$(dirname "${BASH_SOURCE[0]}")
SECURITY_DIR="${CA_DIR:-security}"

CA_DIR="${CA_DIR:-$SECURITY_DIR/ca}"
CA_PRIVATE_KEY_PATH="${CA_PRIVATE_KEY_PATH:-$CA_DIR/CAPrivate.key}"
CA_CERTIFICATE_PATH="${CA_CERTIFICATE_PATH:-$CA_DIR/CACertificate.pem}"

SERVER_DIR="${SERVER_DIR:-$SECURITY_DIR/server}"
SERVER_PRIVATE_KEY_PATH="${SERVER_PRIVATE_KEY_PATH:-$SERVER_DIR/ServerPrivate.key}"
SERVER_CERTIFICATE_REQUEST_PATH="${SERVER_CERTIFICATE_REQUEST_PATH:-$SERVER_DIR/ServerCertificateRequest.csr}"
SERVER_CERTIFICATE_EXTENSION_PATH="${SERVER_CERTIFICATE_EXTENSION_PATH:-$SCRIPT_DIR/localhost.ext}"
SERVER_CERTIFICATE_PATH="${SERVER_CERTIFICATE_PATH:-$SERVER_DIR/ServerCertificate.crt}"

KEYS_DIR="${KEYS_DIR:-$SECURITY_DIR/keys}"
