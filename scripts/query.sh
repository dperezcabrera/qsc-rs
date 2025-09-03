#!/usr/bin/env bash
set -euo pipefail
CONTRACT=${1:-token}
METHOD=${2:-balance_of}
ARGS_JSON=${3:-'{"who":"alice"}'}
curl -s "http://localhost:8000/query?contract=${CONTRACT}&method=${METHOD}&args=$(printf %s "$ARGS_JSON" | jq -s -R -r @uri)" | jq
