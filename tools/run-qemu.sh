#!/usr/bin/env bash
set -euo pipefail

KERNEL_ELF="${1:-target/riscv64gc-unknown-none-elf/debug/uestc-kernel}"
LOG_DIR=".repair_logs"
mkdir -p "${LOG_DIR}"
SERIAL_LOG="${LOG_DIR}/qemu_serial_latest.log"
QEMU_STDERR_LOG="${LOG_DIR}/qemu_stderr_latest.log"
rm -f "${SERIAL_LOG}" "${QEMU_STDERR_LOG}"

timeout 12s qemu-system-riscv64 \
  -machine virt \
  -m 128M \
  -smp 1 \
  -bios default \
  -kernel "${KERNEL_ELF}" \
  -display none \
  -serial "file:${SERIAL_LOG}" \
  -monitor none \
  -no-reboot \
  2>"${QEMU_STDERR_LOG}" || status=$?

status=${status:-0}
cat "${SERIAL_LOG}" || true

if [ "${status}" -ne 0 ] && [ "${status}" -ne 124 ]; then
  echo "[run-qemu] qemu failed with status ${status}" >&2
  cat "${QEMU_STDERR_LOG}" >&2 || true
  exit "${status}"
fi
