#!/usr/bin/env bash
set -euo pipefail
json="${1:-.repair_logs/openat_ocreat_source_report_v136.json}"
txt="${2:-.repair_logs/openat_ocreat_source_report_v136.txt}"
python3 tools/openat_ocreat_source_locator_v136.py "$json" "$txt"
