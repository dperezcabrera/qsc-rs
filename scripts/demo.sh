#!/usr/bin/env bash
set -euo pipefail
IMG=${IMG:-qsc-rs-simple-contracts}
NAME=${NAME:-qsc-node}

echo "==> Build image"
./build.sh

echo "==> Run node"
./run.sh

echo "==> Generate keys (alice)"
./keygen.sh alice

echo "==> Mint 1000 to alice (signed by alice)"
./submit.sh alice alice '{"contract":"token","method":"mint","args":{"to":"alice","amount":1000}}'

echo "==> Transfer 150 alice -> bob"
./submit.sh alice alice '{"contract":"token","method":"transfer","args":{"to":"bob","amount":150}}'

echo "==> Query balances"
./query.sh token balance_of '{"who":"alice"}'
./query.sh token balance_of '{"who":"bob"}'
curl -s http://localhost:8000/head | jq

echo "Done."
