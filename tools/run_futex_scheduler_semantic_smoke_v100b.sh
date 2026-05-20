#!/usr/bin/env bash
set -euo pipefail
mkdir -p .repair_logs
TS="${1:-$(date +%Y%m%d_%H%M%S)}"
python3 tools/futex_scheduler_semantic_guard_v100b.py \
  ".repair_logs/futex_scheduler_semantic_guard_v100b_${TS}.log" \
  ".repair_logs/futex_scheduler_semantic_manifest_v100b_${TS}.json" \
  | tee ".repair_logs/futex_scheduler_semantic_guard_v100b_${TS}.suite.log"
