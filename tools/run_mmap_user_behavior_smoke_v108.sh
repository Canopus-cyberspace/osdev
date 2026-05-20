#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TS="${TS:-$(date +%Y%m%d_%H%M%S)}"
LOG="${1:-$ROOT/.repair_logs/mmap_user_behavior_guard_v108_${TS}.log}"
MANIFEST="${2:-$ROOT/.repair_logs/mmap_user_behavior_manifest_v108_${TS}.json}"
mkdir -p "$ROOT/.repair_logs"
cd "$ROOT"
python3 tools/mmap_user_behavior_guard_v108.py "$LOG" "$MANIFEST"
