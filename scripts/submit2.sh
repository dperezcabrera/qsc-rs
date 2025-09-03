#!/usr/bin/env bash
set -euo pipefail
FROM=${1:-alice}
BASENAME=${2:-alice}
PAYLOAD_RAW=${3:-'{"contract":"token","method":"mint","args":{"to":"alice","amount":1000}}'}
KEYS_DIR=${KEYS_DIR:-$(pwd)/keys}
IMG=${IMG:-qsc-rs-simple-contracts}

# 1) pedir al servidor el payload canónico exacto
CANON=$(curl -s http://localhost:8000/canonical -H 'content-type: application/json' -d @- <<JSON | jq -r .payload
{
  "from": "$FROM",
  "contract": $(printf '%s' "$PAYLOAD_RAW" | jq -r .contract | jq -R .),
  "method":   $(printf '%s' "$PAYLOAD_RAW" | jq -r .method   | jq -R .),
  "args":     $(printf '%s' "$PAYLOAD_RAW" | jq    .args),
  "alg": "mldsa3",
  "pk": "",
  "sig": ""
}
JSON
)

# 2) firmar ese string exacto
SIG=$(docker run --rm -v "$KEYS_DIR":/keys "$IMG" \
  /usr/local/bin/qsc-tools sign --sk /keys/"$BASENAME".sk --payload "$CANON" | tr -d '\n\r')

PK=$(tr -d '\n\r' < "$KEYS_DIR/$BASENAME.pk")

# 3) enviar la tx usando los mismos fields
curl -s http://localhost:8000/call -H 'content-type: application/json' -d @- <<JSON | jq
{
  "from": "$FROM",
  "contract": $(printf '%s' "$PAYLOAD_RAW" | jq -r .contract | jq -R .),
  "method":   $(printf '%s' "$PAYLOAD_RAW" | jq -r .method   | jq -R .),
  "args":     $(printf '%s' "$PAYLOAD_RAW" | jq    .args),
  "alg": "mldsa3",
  "pk": "$PK",
  "sig": "$SIG"
}
JSON
