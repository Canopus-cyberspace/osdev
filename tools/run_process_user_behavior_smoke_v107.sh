#!/usr/bin/env bash
set -Eeuo pipefail
PROJECT="$(cd "$(dirname "$0")/.." && pwd)"
TS="$(date +%Y%m%d_%H%M%S)"
LOG_DIR="$PROJECT/.repair_logs"
mkdir -p "$LOG_DIR"
python3 "$PROJECT/tools/process_user_behavior_guard_v107.py" \
  --project "$PROJECT" \
  --log "$LOG_DIR/process_user_behavior_guard_v107_manual_$TS.log" \
  --manifest "$LOG_DIR/process_user_behavior_manifest_v107_manual_$TS.json"
