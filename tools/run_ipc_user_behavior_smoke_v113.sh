#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TS="${1:-$(date +%Y%m%d_%H%M%S)}"
LOG_DIR="$ROOT/.repair_logs"
mkdir -p "$LOG_DIR"
python3 "$ROOT/tools/ipc_user_behavior_guard_v113.py" \
  "$LOG_DIR/ipc_user_behavior_guard_v113_${TS}.log" \
  "$LOG_DIR/ipc_user_behavior_manifest_v113_${TS}.json"
