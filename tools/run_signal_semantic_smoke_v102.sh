#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$ROOT/.repair_logs"
TS="${1:-$(date +%Y%m%d_%H%M%S)}"
MANIFEST="$ROOT/.repair_logs/signal_semantic_manifest_v102_${TS}.json"
python3 "$ROOT/tools/signal_semantic_guard_v102.py" "$MANIFEST"
