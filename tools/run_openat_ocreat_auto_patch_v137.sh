#!/usr/bin/env bash
set -euo pipefail
ts="${1:-manual}"
excerpt="${2:-.repair_logs/openat_ocreat_patch_excerpt_v137.txt}"
report="${3:-.repair_logs/openat_ocreat_patch_report_v137.json}"
python3 tools/openat_ocreat_auto_patch_v137.py "$ts" "$excerpt" "$report"
