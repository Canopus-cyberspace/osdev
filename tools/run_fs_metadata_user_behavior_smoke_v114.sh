#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p .repair_logs
TS="${1:-$(date +%Y%m%d_%H%M%S)}"
MANIFEST=".repair_logs/fs_metadata_user_behavior_manifest_v114_${TS}.json"
python3 tools/fs_metadata_user_behavior_guard_v114.py "$MANIFEST"
echo "[PASS] run_fs_metadata_user_behavior_smoke_v114 completed; manifest: $MANIFEST"
