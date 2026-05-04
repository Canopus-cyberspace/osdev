#!/usr/bin/env bash
set -euo pipefail

KERNEL_ELF="${1:-target/riscv64gc-unknown-none-elf/debug/uestc-kernel}"
LOG_DIR=".repair_logs"
mkdir -p "$LOG_DIR"
SERIAL_LOG="$LOG_DIR/qemu_serial_$(date +%Y%m%d_%H%M%S).log"

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

QEMU_PID="$!"
trap 'kill "$QEMU_PID" 2>/dev/null || true' INT TERM EXIT

sleep 3
kill "$QEMU_PID" 2>/dev/null || true
wait "$QEMU_PID" 2>/dev/null || true
trap - INT TERM EXIT

echo "[INFO] QEMU serial log tail:"
tail -120 "$SERIAL_LOG" || true
