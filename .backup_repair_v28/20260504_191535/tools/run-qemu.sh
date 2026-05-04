#!/usr/bin/env bash
set -euo pipefail

KERNEL_ELF="${1:-target/riscv64gc-unknown-none-elf/debug/uestc-kernel}"

mkdir -p .repair_logs
LOG_FILE=".repair_logs/qemu_run_$(date +%Y%m%d_%H%M%S).log"

echo "[INFO] QEMU serial log: ${LOG_FILE}"

set +e
timeout 12s qemu-system-riscv64 \
  -machine virt \
  -m 128M \
  -smp 1 \
  -bios default \
  -kernel "${KERNEL_ELF}" \
  -display none \
  -serial file:"${LOG_FILE}" \
  -monitor none \
  -no-reboot
STATUS=$?
set -e

if [ "${STATUS}" -ne 0 ] && [ "${STATUS}" -ne 124 ]; then
  echo "[ERROR] QEMU failed with status ${STATUS}"
  echo "[ERROR] serial log: ${LOG_FILE}"
  exit "${STATUS}"
fi

cat "${LOG_FILE}"
