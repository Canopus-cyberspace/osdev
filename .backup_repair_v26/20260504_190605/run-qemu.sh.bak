#!/usr/bin/env bash
set -euo pipefail

KERNEL_ELF="${1:-target/riscv64gc-unknown-none-elf/debug/uestc-kernel}"

if [[ ! -f "$KERNEL_ELF" ]]; then
  echo "[qemu] ERROR: kernel ELF not found: $KERNEL_ELF" >&2
  exit 2
fi

LOG_DIR="${QEMU_LOG_DIR:-.qemu_logs}"
mkdir -p "$LOG_DIR"
LOG_FILE="${QEMU_LOG_FILE:-$LOG_DIR/qemu_$(date +%Y%m%d_%H%M%S).log}"
TIMEOUT_SEC="${QEMU_TIMEOUT:-8}"

rm -f "$LOG_FILE"

echo "[qemu] kernel: $KERNEL_ELF"
echo "[qemu] serial log: $LOG_FILE"
echo "[qemu] timeout: ${TIMEOUT_SEC}s"

set +e
timeout --foreground "${TIMEOUT_SEC}s" \
  qemu-system-riscv64 \
    -machine virt \
    -m 128M \
    -smp 1 \
    -bios default \
    -kernel "$KERNEL_ELF" \
    -display none \
    -serial file:"$LOG_FILE" \
    -monitor none \
    -no-reboot
STATUS=$?
set -e

if [[ "$STATUS" -ne 0 && "$STATUS" -ne 124 ]]; then
  echo "[qemu] ERROR: qemu exited with status $STATUS" >&2
  if [[ -f "$LOG_FILE" ]]; then
    echo "[qemu] serial log tail:" >&2
    tail -120 "$LOG_FILE" >&2 || true
  fi
  exit "$STATUS"
fi

if [[ ! -s "$LOG_FILE" ]]; then
  echo "[qemu] ERROR: serial log is empty: $LOG_FILE" >&2
  exit 30
fi

cat "$LOG_FILE"

if grep -q "\[task\] skeleton reached stable idle loop" "$LOG_FILE"; then
  echo "[qemu] PASS: kernel reached stable idle loop"
else
  echo "[qemu] WARN: stable idle marker not found in serial log" >&2
fi

exit 0
