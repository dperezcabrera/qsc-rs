#!/usr/bin/env bash
set -euo pipefail
DATA_DIR=${DATA_DIR:-$(pwd)/data}
echo "Head of chain.jsonl:"
tail -n 5 "$DATA_DIR/chain.jsonl" 2>/dev/null || echo "(no blocks yet)"
echo
echo "State snapshot:"
test -f "$DATA_DIR/state.json" && jq . "$DATA_DIR/state.json" || echo "(no state yet)"
