#!/usr/bin/env bash
set -euo pipefail
NAME=${NAME:-qsc-node}
docker rm -f "$NAME" || true
