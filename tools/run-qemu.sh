#!/usr/bin/env bash
set -euo pipefail

KERNEL_ELF="${1:-target/riscv64gc-unknown-none-elf/debug/uestc-kernel}"
LOG_DIR="${QEMU_LOG_DIR:-.repair_logs}"
mkdir -p "$LOG_DIR"
SERIAL_LOG="${QEMU_SERIAL_LOG:-$LOG_DIR/qemu-serial.log}"

echo "[INFO] QEMU serial log: $SERIAL_LOG"

qemu-system-riscv64 \
  -machine virt \
  -m 128M \
  -smp 1 \
  -bios default \
  -kernel "$KERNEL_ELF" \
  -display none \
  -serial "file:$SERIAL_LOG" \
  -monitor none \
  -no-reboot &

QEMU_PID=$!
sleep "${QEMU_TIMEOUT_SEC:-8}"
kill "$QEMU_PID" 2>/dev/null || true
wait "$QEMU_PID" 2>/dev/null || true

echo "[INFO] QEMU stopped"
echo "[INFO] serial log tail:"
tail -80 "$SERIAL_LOG" || true
