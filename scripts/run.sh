#!/usr/bin/env bash
set -euo pipefail

IMG=${IMG:-qsc-rs-simple-contracts}
NAME=${NAME:-qsc-node}
DATA_DIR=${DATA_DIR:-$(pwd)/data}
mkdir -p "$DATA_DIR"

if [ ! -f "scripts/keys/alice.pk" ]; then
	./scripts/keygen.sh alice
fi

ALICE_ADDR=$(docker run --rm -v $(pwd)/scripts/keys:/keys qsc-rs-simple-contracts /usr/local/bin/qsc-tools addr --pk-file /keys/alice.pk)

docker run --rm -it --name qsc-node -p 8000:8000 \
  -e QSC_DATA_DIR=/data \
  -e QSC_MINTER_ADDR="$ALICE_ADDR" \
  -e QSC_TOKEN_MAX_SUPPLY=1000000000 \
  -e QSC_MAX_PENDING_PER_ADDR=50 \
  -e QSC_MAX_TX_PER_BLOCK=5000 \
  -v $(pwd)/data:/data qsc-rs-simple-contracts
