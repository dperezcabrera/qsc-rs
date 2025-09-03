#!/usr/bin/env bash

set -euo pipefail

IMG=${IMG:-qsc-rs-simple-contracts}
echo "Building Docker image: $IMG"
docker build -t "$IMG" .
echo "Done."

./scripts/keygen.sh v1
./scripts/keygen.sh v2
./scripts/keygen.sh v3

docker network create qsc-net || true
