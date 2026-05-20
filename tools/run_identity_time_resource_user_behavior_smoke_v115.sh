#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p .repair_logs
TS="${V115_TS:-$(date +%Y%m%d_%H%M%S)}"
export V115_GUARD_LOG="${V115_GUARD_LOG:-$ROOT/.repair_logs/identity_time_resource_user_behavior_guard_v115_${TS}.log}"
export V115_MANIFEST="${V115_MANIFEST:-$ROOT/.repair_logs/identity_time_resource_user_behavior_manifest_v115_${TS}.json}"
python3 tools/identity_time_resource_user_behavior_guard_v115.py | tee "$V115_GUARD_LOG"
