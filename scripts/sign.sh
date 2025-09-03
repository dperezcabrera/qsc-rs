#!/usr/bin/env bash
set -euo pipefail
IMG=${IMG:-qsc-rs-simple-contracts}
SK_BASENAME=${1:-alice}
PAYLOAD_RAW=${2:-'{"contract":"token","method":"mint","args":{"to":"alice","amount":1000}}'}
KEYS_DIR=${KEYS_DIR:-$(pwd)/keys}

mkdir -p "$KEYS_DIR"
PAYLOAD_MIN=$(printf '%s' "$PAYLOAD_RAW" | jq -c .)

docker run --rm -v "$KEYS_DIR":/keys "$IMG" \
  /usr/local/bin/qsc-tools sign --sk /keys/"$SK_BASENAME".sk --payload "$PAYLOAD_MIN"

