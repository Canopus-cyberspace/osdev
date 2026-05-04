#!/usr/bin/env bash
set -euo pipefail

KERNEL_ELF="${1:-target/riscv64gc-unknown-none-elf/debug/uestc-kernel}"
LOG_DIR="${QEMU_LOG_DIR:-.repair_logs}"
mkdir -p "$LOG_DIR"
SERIAL_LOG="${QEMU_SERIAL_LOG:-$LOG_DIR/qemu-serial.log}"

echo "[run-qemu] kernel = $KERNEL_ELF"
echo "[run-qemu] serial = $SERIAL_LOG"

qemu-system-riscv64 \
  -machine virt \
  -m 128M \
  -smp 1 \
  -bios default \
  -kernel "$KERNEL_ELF" \
  -display none \
  -serial "file:$SERIAL_LOG" \
  -monitor none \
  -no-reboot
