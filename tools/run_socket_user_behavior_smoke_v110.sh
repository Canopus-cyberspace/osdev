#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p .repair_logs
TS="${TS:-$(date +%Y%m%d_%H%M%S)}"
MANIFEST="${MANIFEST:-$ROOT/.repair_logs/socket_user_behavior_manifest_v110_${TS}.json}"
python3 tools/socket_user_behavior_guard_v110.py "$MANIFEST"
