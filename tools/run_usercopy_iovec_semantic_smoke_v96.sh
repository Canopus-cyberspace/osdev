#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$ROOT/.repair_logs"
TS="$(date +%Y%m%d_%H%M%S)"
MANIFEST="$ROOT/.repair_logs/usercopy_iovec_semantic_manifest_v96_suite_${TS}.json"
echo "[INFO] running v96 usercopy/iovec/timespec semantic guard"
python3 "$ROOT/tools/usercopy_iovec_semantic_guard_v96.py" "$ROOT" "$MANIFEST"
echo "[PASS] run_usercopy_iovec_semantic_smoke_v96.sh completed"
