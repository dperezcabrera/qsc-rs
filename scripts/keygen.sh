#!/usr/bin/env bash
set -euo pipefail
IMG=${IMG:-qsc-rs-simple-contracts}
OUT=${1:-alice}
KEYS_DIR=${KEYS_DIR:-$(pwd)/keys}
mkdir -p "$KEYS_DIR"
echo "Generating ML-DSA-3 keys: $OUT (.pk, .sk)"
docker run --rm -v "$KEYS_DIR":/keys "$IMG" \
  /usr/local/bin/qsc-tools keygen --out /keys/"$OUT"
echo "Keys at $KEYS_DIR/$OUT.pk and $KEYS_DIR/$OUT.sk"
