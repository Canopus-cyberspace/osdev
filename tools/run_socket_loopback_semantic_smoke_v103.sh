#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p .repair_logs
STAMP="$(date +%Y%m%d_%H%M%S)"
MANIFEST=".repair_logs/socket_loopback_semantic_manifest_v103_${STAMP}.json"
python3 tools/socket_loopback_semantic_guard_v103.py "$MANIFEST"
echo "[PASS] socket loopback semantic smoke v103 guard completed; manifest: $ROOT/$MANIFEST"
