#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p .repair_logs
TS="${TS:-$(date +%Y%m%d_%H%M%S)}"
MANIFEST="${MANIFEST:-.repair_logs/signal_user_behavior_manifest_v109_${TS}.json}"
python3 tools/signal_user_behavior_guard_v109.py --version v109 --manifest "$MANIFEST"
