#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$ROOT/.repair_logs"
TS="${1:-$(date +%Y%m%d_%H%M%S)}"
python3 "$ROOT/tools/identity_time_resource_user_behavior_guard_v115b.py" "$ROOT" "$ROOT/.repair_logs/identity_time_resource_manifest_v115b_${TS}.json"
