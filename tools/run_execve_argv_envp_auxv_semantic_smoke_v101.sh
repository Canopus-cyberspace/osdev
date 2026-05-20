#!/usr/bin/env bash
set -euo pipefail
PROJECT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TS="$(date +%Y%m%d_%H%M%S)"
LOG_DIR="$PROJECT/.repair_logs"
mkdir -p "$LOG_DIR"
MANIFEST="$LOG_DIR/execve_argv_envp_auxv_semantic_manifest_v101_manual_${TS}.json"
GUARD_LOG="$LOG_DIR/execve_argv_envp_auxv_semantic_guard_v101_manual_${TS}.log"
BUILD_LOG="$LOG_DIR/build_v101_manual_${TS}.log"
cd "$PROJECT"
python3 tools/execve_argv_envp_auxv_semantic_guard_v101.py --project "$PROJECT" --manifest "$MANIFEST" | tee "$GUARD_LOG"
set +e
cargo build 2>&1 | tee "$BUILD_LOG"
STATUS=${PIPESTATUS[0]}
set -e
if [[ "$STATUS" -ne 0 ]]; then
  echo "[ERROR] cargo build failed with status $STATUS"
  exit "$STATUS"
fi
if grep -En "matches any value|unreachable pattern" "$BUILD_LOG"; then
  echo "[ERROR] forbidden Rust match-pattern warning detected"
  exit 1
fi
echo "[PASS] execve argv/envp/auxv semantic smoke v101 completed"
