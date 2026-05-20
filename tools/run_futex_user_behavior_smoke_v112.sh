#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TS="${1:-$(date +%Y%m%d_%H%M%S)}"
LOG_DIR="$ROOT/.repair_logs"
mkdir -p "$LOG_DIR"
python3 "$ROOT/tools/futex_user_behavior_guard_v112.py" \
  "$LOG_DIR/futex_user_behavior_guard_v112_${TS}.log" \
  "$LOG_DIR/futex_user_behavior_manifest_v112_${TS}.json"
