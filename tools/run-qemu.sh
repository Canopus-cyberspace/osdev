#!/usr/bin/env bash
set -euo pipefail

KERNEL_ELF="$1"
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOG_DIR="$PROJECT_ROOT/.repair_logs"
mkdir -p "$LOG_DIR"
SERIAL_LOG="$LOG_DIR/make_run.serial.log"
STDERR_LOG="$LOG_DIR/make_run.stderr.log"
rm -f "$SERIAL_LOG" "$STDERR_LOG"

echo "[run-qemu] serial log: $SERIAL_LOG"

qemu-system-riscv64 \
  -machine virt \
  -m 128M \
  -smp 1 \
  -bios default \
  -kernel "$KERNEL_ELF" \
  -display none \
  -serial file:"$SERIAL_LOG" \
  -monitor none \
  -no-reboot \
  2>"$STDERR_LOG"
