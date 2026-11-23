#!/usr/bin/env bash
set -e

./build_wasm.sh

PORT=${1:-8080}
echo ""
echo "=========================================================="
echo "  RAMAYANA: THE RETRO EPIC (WASM)"
echo "  Open your browser and play at: http://localhost:$PORT"
echo "=========================================================="
echo ""

python3 -m http.server "$PORT" --directory dist
