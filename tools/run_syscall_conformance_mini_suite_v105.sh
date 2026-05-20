#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TS="${1:-$(date +%Y%m%d_%H%M%S)}"
MARKER="${2:-hello from external init.elf v105 syscall write}"
mkdir -p "$ROOT/.repair_logs"
python3 "$ROOT/tools/syscall_conformance_mini_suite_v105.py" \
  --project "$ROOT" \
  --marker "$MARKER" \
  --guard-log "$ROOT/.repair_logs/syscall_conformance_mini_suite_v105_${TS}.log" \
  --manifest "$ROOT/.repair_logs/syscall_conformance_mini_suite_manifest_v105_${TS}.json"
