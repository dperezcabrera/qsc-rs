
#!/usr/bin/env bash
set -euo pipefail
# Usage: ./submit_addr.sh <pk_basename> <payload-template-json>
# Replaces FROM with addr(pk) and, if ARGS has "to":"SELF", replaces it with same addr.
BASENAME=${1:-alice}
PAYLOAD_RAW=${2:-'{"contract":"token","method":"mint","args":{"to":"SELF","amount":1000}}'}
KEYS_DIR=${KEYS_DIR:-$(pwd)/keys}
IMG=${IMG:-qsc-rs-simple-contracts}

ADDR=$(docker run --rm -v "$KEYS_DIR":/keys "$IMG" /usr/local/bin/qsc-tools addr --pk-file /keys/"$BASENAME".pk | tr -d '\n\r')
PAYLOAD_SELF=$(printf '%s' "$PAYLOAD_RAW" | jq --arg a "$ADDR" '(.args.to == "SELF") as $is | if $is then .args.to = $a else . end')

# Use canonical flow
CANON=$(curl -s http://localhost:8000/canonical -H 'content-type: application/json' -d @- <<JSON | jq -r .payload
{
  "from": "$ADDR",
  "contract": $(printf '%s' "$PAYLOAD_SELF" | jq -r .contract | jq -R .),
  "method":   $(printf '%s' "$PAYLOAD_SELF" | jq -r .method   | jq -R .),
  "args":     $(printf '%s' "$PAYLOAD_SELF" | jq    .args),
  "alg": "mldsa3",
  "pk": "",
  "sig": ""
}
JSON
)

SIG=$(docker run --rm -v "$KEYS_DIR":/keys "$IMG" \
  /usr/local/bin/qsc-tools sign --sk /keys/"$BASENAME".sk --payload "$CANON" | tr -d '\n\r')
PK=$(tr -d '\n\r' < "$KEYS_DIR/$BASENAME.pk")

curl -s http://localhost:8000/call -H 'content-type: application/json' -d @- <<JSON | jq
{
  "from": "$ADDR",
  "contract": $(printf '%s' "$PAYLOAD_SELF" | jq -r .contract | jq -R .),
  "method":   $(printf '%s' "$PAYLOAD_SELF" | jq -r .method   | jq -R .),
  "args":     $(printf '%s' "$PAYLOAD_SELF" | jq    .args),
  "alg": "mldsa3",
  "pk": "$PK",
  "sig": "$SIG"
}
JSON
