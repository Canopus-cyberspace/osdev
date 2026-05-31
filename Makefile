.DEFAULT_GOAL := all
SHELL := /bin/bash

TARGET ?= riscv64gc-unknown-none-elf
LOONGARCH_TARGET ?= loongarch64-unknown-none
CARGO ?= cargo
PYTHON ?= python3
CARGO_TARGET_DIR ?= target
KERNEL_ELF ?= $(CARGO_TARGET_DIR)/$(TARGET)/debug/source-riscv64-kernel
KERNEL_RV ?= kernel-rv
KERNEL_LA ?= kernel-la
LOONGARCH_KERNEL_ELF ?= $(CARGO_TARGET_DIR)/$(LOONGARCH_TARGET)/debug/source-loongarch64-kernel
MAKE_BUILD_LOG ?= .repair_logs/make_all_cargo_build.log
FORBIDDEN_BUILD_RE := matches any value|unreachable pattern|warning: unused variable:

.PHONY: all cargo-build loongarch-kernel official-artifacts clean

all: official-artifacts

cargo-build:
	@set -euo pipefail; \
	mkdir -p "$$(dirname "$(MAKE_BUILD_LOG)")"; \
	if [ -f /home/lenovo/miniconda3/etc/profile.d/conda.sh ]; then \
		source /home/lenovo/miniconda3/etc/profile.d/conda.sh; \
		conda activate osdev || true; \
	fi; \
	source "$$HOME/.cargo/env" 2>/dev/null || true; \
	export PATH="$$HOME/.cargo/bin:$$PATH"; \
	if command -v rustup >/dev/null 2>&1; then \
		rustup target add "$(TARGET)" || true; \
		rustup target add "$(LOONGARCH_TARGET)" || true; \
		rustup component add rust-src || true; \
	fi; \
	$(PYTHON) user/build_init_elf.py; \
	RUSTFLAGS="-C force-frame-pointers=yes $${RUSTFLAGS:-}" \
		$(CARGO) build --target "$(TARGET)" --target "$(LOONGARCH_TARGET)" 2>&1 | tee "$(MAKE_BUILD_LOG)"; \
	if grep -nE '$(FORBIDDEN_BUILD_RE)' "$(MAKE_BUILD_LOG)" >/dev/null; then \
		echo "[make] forbidden build output observed"; \
		grep -nE '$(FORBIDDEN_BUILD_RE)' "$(MAKE_BUILD_LOG)"; \
		exit 1; \
	fi

loongarch-kernel:
	@set -euo pipefail; \
	if [ ! -s "$(LOONGARCH_KERNEL_ELF)" ]; then \
		echo "[make] LoongArch kernel not found at $(LOONGARCH_KERNEL_ELF)"; \
		exit 1; \
	fi

official-artifacts: cargo-build loongarch-kernel
	@set -euo pipefail; \
	test -s "$(KERNEL_ELF)"; \
	test -s "$(LOONGARCH_KERNEL_ELF)"; \
	cp -f "$(KERNEL_ELF)" "$(KERNEL_RV)"; \
	cp -f "$(LOONGARCH_KERNEL_ELF)" "$(KERNEL_LA)"; \
	printf '[make] wrote %s from %s\n' "$(KERNEL_RV)" "$(KERNEL_ELF)"; \
	printf '[make] wrote %s from %s\n' "$(KERNEL_LA)" "$(LOONGARCH_KERNEL_ELF)"
