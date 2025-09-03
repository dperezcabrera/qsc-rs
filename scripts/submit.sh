#!/usr/bin/env bash
set -euo pipefail
FROM=${1:-alice}
BASENAME=${2:-alice}
PAYLOAD_RAW=${3:-'{"contract":"token","method":"mint","args":{"to":"alice","amount":1000}}'}
KEYS_DIR=${KEYS_DIR:-$(pwd)/keys}

PAYLOAD_MIN=$(printf '%s' "$PAYLOAD_RAW" | jq -c .)

PK=$(tr -d '\n\r' < "$KEYS_DIR/$BASENAME.pk")
SIG=$(./sign.sh "$BASENAME" "$PAYLOAD_MIN" | tr -d '\n\r')

# debug útil
echo "Submitting signed call for $FROM:"
echo "$PAYLOAD_MIN" | jq .
echo "PK len: ${#PK} hex  | SIG len: ${#SIG} hex"

CONTRACT=$(printf '%s' "$PAYLOAD_MIN" | jq -r .contract | jq -R .)
METHOD=$(printf   '%s' "$PAYLOAD_MIN" | jq -r .method   | jq -R .)
ARGS=$(printf     '%s' "$PAYLOAD_MIN" | jq    .args)

curl -s http://localhost:8000/call \
  -H "content-type: application/json" \
  -d @- <<JSON | jq
{
  "from": "$FROM",
  "contract": $CONTRACT,
  "method":   $METHOD,
  "args":     $ARGS,
  "alg": "mldsa3",
  "pk": "$PK",
  "sig": "$SIG"
}
JSON

