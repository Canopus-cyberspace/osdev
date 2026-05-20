#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p .repair_logs
TS="${1:-$(date +%Y%m%d_%H%M%S)}"
GUARD_LOG=".repair_logs/syscall_regression_suite_embedded_${TS}.log"
BUILD_LOG=".repair_logs/build_regression_suite_${TS}.log"

echo "[INFO] running syscall regression guard" | tee "$GUARD_LOG"
python3 tools/syscall_regression_guard.py 2>&1 | tee -a "$GUARD_LOG"

echo "[INFO] running cargo build for syscall regression suite" | tee "$BUILD_LOG"
cargo build 2>&1 | tee -a "$BUILD_LOG"
if grep -nE "matches any value|unreachable pattern" "$BUILD_LOG"; then
  echo "[ERROR] forbidden Rust match-pattern warning detected" | tee -a "$BUILD_LOG"
  exit 1
fi
echo "[PASS] syscall regression suite local guard/build passed"
