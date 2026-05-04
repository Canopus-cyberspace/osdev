#!/usr/bin/env bash
set -euo pipefail

KERNEL_ELF="${1:-target/riscv64gc-unknown-none-elf/debug/uestc-kernel}"
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOG_DIR="$PROJECT_ROOT/.repair_logs"
mkdir -p "$LOG_DIR"
TS="$(date +%Y%m%d_%H%M%S)"
SERIAL_LOG="$LOG_DIR/qemu_serial_run_${TS}.log"

echo "[run-qemu] kernel     = $KERNEL_ELF"
echo "[run-qemu] serial log = $SERIAL_LOG"

set +e
timeout 20s qemu-system-riscv64 \
  -machine virt \
  -m 128M \
  -smp 1 \
  -bios default \
  -kernel "$KERNEL_ELF" \
  -display none \
  -serial file:"$SERIAL_LOG" \
  -monitor none \
  -no-reboot
STATUS=$?
set -e

if [ "$STATUS" -ne 0 ] && [ "$STATUS" -ne 124 ]; then
  echo "[run-qemu] qemu failed with status $STATUS"
  tail -120 "$SERIAL_LOG" || true
  exit "$STATUS"
fi

echo "[run-qemu] qemu status = $STATUS"
echo "[run-qemu] serial log tail:"
tail -120 "$SERIAL_LOG" || true
