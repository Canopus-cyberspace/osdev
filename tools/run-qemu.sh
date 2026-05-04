#!/usr/bin/env bash
set -euo pipefail

KERNEL_ELF="${1:?usage: run-qemu.sh <kernel-elf>}"
PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
LOG_DIR="${PROJECT_DIR}/.repair_logs"
mkdir -p "${LOG_DIR}"
TS="$(date +%Y%m%d_%H%M%S)"
SERIAL_LOG="${LOG_DIR}/make_run_${TS}.serial.log"
STDERR_LOG="${LOG_DIR}/make_run_${TS}.stderr.log"

echo "[INFO] QEMU serial log: ${SERIAL_LOG}"
echo "[INFO] QEMU stderr log: ${STDERR_LOG}"

qemu-system-riscv64 \
  -machine virt \
  -m 128M \
  -smp 1 \
  -bios default \
  -kernel "${KERNEL_ELF}" \
  -display none \
  -serial "file:${SERIAL_LOG}" \
  -monitor none \
  -no-reboot \
  2>"${STDERR_LOG}"
