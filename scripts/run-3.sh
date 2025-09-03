#!/usr/bin/env bash
set -euo pipefail

NODO=3

IMG=${IMG:-qsc-rs-simple-contracts}
PORT=800$NODO
DATA_DIR=${DATA_DIR:-"$(pwd)/data/node${NODO}"}
mkdir -p "$DATA_DIR"

V_PK=$(tr -d '\n\r' < keys/v${NODO}.pk)
V_SK=$(tr -d '\n\r' < keys/v${NODO}.sk)

V1_PK=$(tr -d '\n\r' < keys/v1.pk)
V1_SK=$(tr -d '\n\r' < keys/v1.sk)
V2_PK=$(tr -d '\n\r' < keys/v2.pk)
V2_SK=$(tr -d '\n\r' < keys/v2.sk)
V3_PK=$(tr -d '\n\r' < keys/v3.pk)
V3_SK=$(tr -d '\n\r' < keys/v3.sk)

VALS=$(jq -cn --arg v1 "$V1_PK" --arg v2 "$V2_PK" --arg v3 "$V3_PK" '
[
  {"id":"n1","url":"http://qsc-node-1:8000","pk":$v1},
  {"id":"n2","url":"http://qsc-node-2:8000","pk":$v2},
  {"id":"n3","url":"http://qsc-node-3:8000","pk":$v3}
]')

docker run --rm -d --name qsc-node-${NODO} --network qsc-net -p ${PORT}:8000 \
  -e QSC_DATA_DIR=/data \
  -e QSC_CONSENSUS=poa \
  -e QSC_SLOT_MS=3000 \
  -e QSC_VALIDATOR_PK="$V_PK" \
  -e QSC_VALIDATOR_SK="$V_SK" \
  -e QSC_MINTER_ADDR="$V_PK" \
  -e QSC_VALIDATORS_JSON="$VALS" \
  -v "$DATA_DIR:/data" $IMG
