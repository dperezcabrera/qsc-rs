#!/usr/bin/env bash
set -euo pipefail
NAME=${NAME:-qsc-node}
docker logs -f "$NAME"
