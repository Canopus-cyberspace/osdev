#!/usr/bin/env bash
set -euo pipefail
KERNEL_ELF="$1"
qemu-system-riscv64 \
  -machine virt \
  -m 128M \
  -smp 1 \
  -bios default \
  -kernel "$KERNEL_ELF" \
  -display none \
  -serial stdio \
  -monitor none \
  -no-reboot
