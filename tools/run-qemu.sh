#!/usr/bin/env bash
set -euo pipefail

KERNEL_ELF="${1:-target/riscv64gc-unknown-none-elf/debug/uestc-kernel}"
LOG_DIR="${QEMU_LOG_DIR:-.repair_logs}"
mkdir -p "$LOG_DIR"
TS="$(date +%Y%m%d_%H%M%S)"
QEMU_STDERR_LOG="$LOG_DIR/qemu_stderr_${TS}.log"
QEMU_SERIAL_LOG="$LOG_DIR/qemu_serial_${TS}.log"

echo "[INFO] qemu kernel = $KERNEL_ELF"
echo "[INFO] qemu stderr log = $QEMU_STDERR_LOG"
echo "[INFO] qemu serial log = $QEMU_SERIAL_LOG"

set +e
timeout 12s qemu-system-riscv64 \
  -machine virt \
  -m 128M \
  -smp 1 \
  -bios default \
  -kernel "$KERNEL_ELF" \
  -display none \
  -serial file:"$QEMU_SERIAL_LOG" \
  -monitor none \
  -no-reboot \
  >"$QEMU_STDERR_LOG" 2>&1
STATUS=$?
set -e

cat "$QEMU_SERIAL_LOG" || true

echo "[INFO] QEMU status = $STATUS"

if [ "$STATUS" -ne 0 ] && [ "$STATUS" -ne 124 ]; then
  echo "[ERROR] QEMU failed with status $STATUS"
  echo "[INFO] stderr tail:"
  tail -80 "$QEMU_STDERR_LOG" || true
  exit "$STATUS"
fi

exit 0
