#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p .repair_logs
TS="${V106_TS:-$(date +%Y%m%d_%H%M%S)}"
export V106_MARKER="${V106_MARKER:-hello from external init.elf v106 syscall write}"
export V106_GUARD_LOG="${V106_GUARD_LOG:-$ROOT/.repair_logs/vfs_user_behavior_guard_v106_${TS}.log}"
export V106_MANIFEST="${V106_MANIFEST:-$ROOT/.repair_logs/vfs_user_behavior_manifest_v106_${TS}.json}"
python3 tools/vfs_user_behavior_guard_v106.py
