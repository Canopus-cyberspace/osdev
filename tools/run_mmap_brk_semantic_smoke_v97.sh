#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
mkdir -p .repair_logs
TS="${1:-$(date +%Y%m%d_%H%M%S)}"
export V97_GUARD_LOG=".repair_logs/mmap_brk_semantic_guard_v97_${TS}.log"
export V97_MANIFEST=".repair_logs/mmap_brk_semantic_manifest_v97_${TS}.json"
python3 tools/mmap_brk_semantic_guard_v97.py
