#!/usr/bin/env bash
set -e

KERNEL_ELF="$1"

qemu-system-riscv64 \
  -machine virt \
  -nographic \
  -bios default \
  -kernel "${KERNEL_ELF}"
