#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p .repair_logs
STAMP="${1:-$(date +%Y%m%d_%H%M%S)}"
GUARD_LOG=".repair_logs/regression_baseline_snapshot_guard_v94_${STAMP}.log"
MANIFEST=".repair_logs/regression_baseline_snapshot_manifest_v94_${STAMP}.json"
python3 tools/regression_baseline_snapshot_v94.py --manifest "$MANIFEST" --guard-log "$GUARD_LOG"
echo "[PASS] v94 baseline snapshot manifest: $MANIFEST"
