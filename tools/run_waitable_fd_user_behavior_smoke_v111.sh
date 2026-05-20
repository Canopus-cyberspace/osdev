#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p .repair_logs
TS="${TS:-$(date +%Y%m%d_%H%M%S)}"
MANIFEST="${MANIFEST:-$ROOT/.repair_logs/waitable_fd_user_behavior_manifest_v111_${TS}.json}"
python3 tools/waitable_fd_user_behavior_guard_v111.py "$MANIFEST"
