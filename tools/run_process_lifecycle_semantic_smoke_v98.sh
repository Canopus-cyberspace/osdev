#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$ROOT/.repair_logs"
TS="$(date +%Y%m%d_%H%M%S)"
python3 "$ROOT/tools/process_lifecycle_semantic_guard_v98.py" "$ROOT/.repair_logs/process_lifecycle_semantic_guard_v98_${TS}.log" "$ROOT/.repair_logs/process_lifecycle_semantic_manifest_v98_${TS}.json"
