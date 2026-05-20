.DEFAULT_GOAL := all
SHELL := /bin/bash

TARGET ?= riscv64gc-unknown-none-elf
LOONGARCH_TARGET ?= loongarch64-unknown-none
CARGO ?= cargo
RUSTC ?= rustc
PYTHON ?= python3
CARGO_TARGET_DIR ?= target
KERNEL_ELF ?= $(CARGO_TARGET_DIR)/$(TARGET)/debug/uestc-kernel
KERNEL_RV ?= kernel-rv
KERNEL_LA ?= kernel-la
LOONGARCH_KERNEL_SRC ?= src/arch/loongarch64/kernel.rs
LOONGARCH_LINKER ?= src/arch/loongarch64/linker.ld
MAKE_BUILD_LOG ?= .repair_logs/make_all_cargo_build.log
LOONGARCH_BUILD_LOG ?= .repair_logs/make_all_loongarch_build.log
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
		rustup component add rust-src || true; \
	fi; \
	$(PYTHON) user/build_init_elf.py; \
	$(CARGO) build --target "$(TARGET)" 2>&1 | tee "$(MAKE_BUILD_LOG)"; \
	if grep -nE '$(FORBIDDEN_BUILD_RE)' "$(MAKE_BUILD_LOG)" >/dev/null; then \
		echo "[make] forbidden build output observed"; \
		grep -nE '$(FORBIDDEN_BUILD_RE)' "$(MAKE_BUILD_LOG)"; \
		exit 1; \
	fi

loongarch-kernel:
	@set -euo pipefail; \
	mkdir -p "$$(dirname "$(LOONGARCH_BUILD_LOG)")"; \
	source "$$HOME/.cargo/env" 2>/dev/null || true; \
	export PATH="$$HOME/.cargo/bin:$$PATH"; \
	if command -v rustup >/dev/null 2>&1; then \
		rustup target add "$(LOONGARCH_TARGET)" || true; \
	fi; \
	$(RUSTC) --target "$(LOONGARCH_TARGET)" "$(LOONGARCH_KERNEL_SRC)" \
		-C panic=abort \
		-C opt-level=z \
		-C linker=rust-lld \
		-C link-arg=-T$(LOONGARCH_LINKER) \
		-C link-arg=--no-relax \
		-o "$(KERNEL_LA)" 2>&1 | tee "$(LOONGARCH_BUILD_LOG)"; \
	if grep -nE '$(FORBIDDEN_BUILD_RE)' "$(LOONGARCH_BUILD_LOG)" >/dev/null; then \
		echo "[make] forbidden LoongArch build output observed"; \
		grep -nE '$(FORBIDDEN_BUILD_RE)' "$(LOONGARCH_BUILD_LOG)"; \
		exit 1; \
	fi

official-artifacts: cargo-build loongarch-kernel
	@set -euo pipefail; \
	test -s "$(KERNEL_ELF)"; \
	test -s "$(KERNEL_LA)"; \
	cp -f "$(KERNEL_ELF)" "$(KERNEL_RV)"; \
	printf '[make] wrote %s from %s\n' "$(KERNEL_RV)" "$(KERNEL_ELF)"; \
	printf '[make] wrote %s from %s\n' "$(KERNEL_LA)" "$(LOONGARCH_KERNEL_SRC)"

clean:
	@rm -f "$(KERNEL_RV)" "$(KERNEL_LA)"
	@$(CARGO) clean || rm -rf target

.PHONY: full-regression-smoke

full-regression-smoke:
	bash ./tools/run_full_regression_smoke.sh

.PHONY: v94-baseline-snapshot
v94-baseline-snapshot:
	@bash tools/run_v94_baseline_snapshot.sh

.PHONY: fd-vfs-semantic-smoke-v95
fd-vfs-semantic-smoke-v95:
	@bash tools/run_fd_vfs_semantic_smoke_v95.sh

.PHONY: usercopy-iovec-semantic-smoke-v96
usercopy-iovec-semantic-smoke-v96:
	bash tools/run_usercopy_iovec_semantic_smoke_v96.sh

.PHONY: mmap-brk-semantic-smoke-v97
mmap-brk-semantic-smoke-v97:
	bash ./tools/run_mmap_brk_semantic_smoke_v97.sh

.PHONY: process-lifecycle-semantic-smoke-v98
process-lifecycle-semantic-smoke-v98:
	bash tools/run_process_lifecycle_semantic_smoke_v98.sh

.PHONY: pipe-dup-event-poll-semantic-smoke-v99
pipe-dup-event-poll-semantic-smoke-v99:
	bash tools/run_pipe_dup_event_poll_semantic_smoke_v99.sh

.PHONY: futex-scheduler-semantic-smoke-v100b
futex-scheduler-semantic-smoke-v100b:
	@bash tools/run_futex_scheduler_semantic_smoke_v100b.sh

.PHONY: execve-argv-envp-auxv-semantic-smoke-v101
execve-argv-envp-auxv-semantic-smoke-v101:
	bash tools/run_execve_argv_envp_auxv_semantic_smoke_v101.sh

.PHONY: execve-argv-envp-auxv-semantic-smoke-v101b
execve-argv-envp-auxv-semantic-smoke-v101b:
	python3 tools/execve_argv_envp_auxv_semantic_guard_v101b.py --manifest .repair_logs/execve_argv_envp_auxv_semantic_manifest_v101b_make.json --marker "hello from external init.elf v101 syscall write"

signal-semantic-smoke-v102:
	bash tools/run_signal_semantic_smoke_v102.sh

.PHONY: socket-loopback-semantic-smoke-v103
socket-loopback-semantic-smoke-v103:
	bash tools/run_socket_loopback_semantic_smoke_v103.sh

timerfd-epoll-waitable-semantic-smoke-v104:
	bash ./tools/run_timerfd_epoll_waitable_semantic_smoke_v104.sh "$(CURDIR)"

.PHONY: syscall-conformance-mini-suite-v105
syscall-conformance-mini-suite-v105:
	bash tools/run_syscall_conformance_mini_suite_v105.sh

.PHONY: vfs-user-behavior-smoke-v106
vfs-user-behavior-smoke-v106:
	bash tools/run_vfs_user_behavior_smoke_v106.sh

.PHONY: process-user-behavior-smoke-v107
process-user-behavior-smoke-v107:
	bash tools/run_process_user_behavior_smoke_v107.sh

mmap-user-behavior-smoke-v108:
	bash tools/run_mmap_user_behavior_smoke_v108.sh

signal-user-behavior-smoke-v109:
	bash tools/run_signal_user_behavior_smoke_v109.sh

.PHONY: socket-user-behavior-smoke-v110
socket-user-behavior-smoke-v110:
	bash ./tools/run_socket_user_behavior_smoke_v110.sh

.PHONY: waitable-fd-user-behavior-smoke-v111
waitable-fd-user-behavior-smoke-v111:
	bash ./tools/run_waitable_fd_user_behavior_smoke_v111.sh

.PHONY: futex-user-behavior-smoke-v112
futex-user-behavior-smoke-v112:
	bash tools/run_futex_user_behavior_smoke_v112.sh

.PHONY: fs-metadata-user-behavior-smoke-v114
fs-metadata-user-behavior-smoke-v114:
	bash tools/run_fs_metadata_user_behavior_smoke_v114.sh

.PHONY: ipc-user-behavior-smoke-v113
ipc-user-behavior-smoke-v113:
	bash tools/run_ipc_user_behavior_smoke_v113.sh

.PHONY: identity-time-resource-user-behavior-smoke-v115
identity-time-resource-user-behavior-smoke-v115:
	bash tools/run_identity_time_resource_user_behavior_smoke_v115.sh

identity-time-resource-user-behavior-smoke-v115b:
	bash tools/run_identity_time_resource_user_behavior_smoke_v115b.sh

fcntl-tty-fdflags-user-behavior-smoke-v116:
	bash tools/run_fcntl_tty_fdflags_user_behavior_smoke_v116.sh

path-cwd-user-behavior-smoke-v117:
	bash tools/run_path_cwd_user_behavior_smoke_v117.sh

vfs-sync-truncate-user-behavior-smoke-v118:
	bash tools/run_vfs_sync_truncate_user_behavior_smoke_v118.sh

mount-statfs-user-behavior-smoke-v119:
	bash tools/run_mount_statfs_user_behavior_smoke_v119.sh

process-observe-runtime-user-behavior-smoke-v120:
	bash tools/run_process_observe_runtime_user_behavior_smoke_v120.sh

async-vector-io-user-behavior-smoke-v121:
	bash tools/run_async_vector_io_user_behavior_smoke_v121.sh

security-permission-user-behavior-smoke-v122:
	bash tools/run_security_permission_user_behavior_smoke_v122.sh

socket-network-api-user-behavior-smoke-v123:
	bash tools/run_socket_network_api_user_behavior_smoke_v123.sh

sched-resource-affinity-user-behavior-smoke-v124:
	bash tools/run_sched_resource_affinity_user_behavior_smoke_v124.sh

userland-compat-regression-matrix-v125:
	bash tools/run_userland_compat_regression_matrix_v125.sh

userland-behavior-harness-v126:
	bash tools/run_userland_behavior_harness_v126.sh

external-init-conformance-runner-plan-v127:
	bash tools/run_external_init_conformance_runner_plan_v127.sh

external-init-conformance-runner-scaffold-v128:
	bash tools/run_external_init_conformance_runner_scaffold_v128.sh

external-init-conformance-marker-protocol-v129:
	bash tools/run_external_init_conformance_marker_protocol_v129.sh

external-init-conformance-marker-protocol-v129b:
	bash tools/run_external_init_conformance_marker_protocol_v129b.sh

external-init-conformance-evidence-parser-v130:
	bash tools/run_external_init_conformance_evidence_parser_v130.sh

external-init-conformance-evidence-aggregator-v131:
	bash tools/run_external_init_conformance_evidence_aggregator_v131.sh

vfs-open-rw-lseek-real-conformance-v132:
	bash tools/run_vfs_open_rw_lseek_real_conformance_v132.sh

external-init-vfs-open-rw-lseek-real-runner-v133:
	bash tools/run_external_init_vfs_open_rw_lseek_real_runner_v133.sh

vfs-open-rw-lseek-step-diagnostic-v133b:
	bash tools/run_vfs_open_rw_lseek_step_diagnostic_v133b.sh

vfs-offset-overwrite-real-conformance-v134:
	bash tools/run_vfs_offset_overwrite_real_conformance_v134.sh

vfs-offset-overwrite-shortpath-v134b:
	bash tools/run_vfs_offset_overwrite_shortpath_v134b.sh

openat-matrix-diagnostic-v134c:
	bash tools/run_openat_matrix_diagnostic_v134c.sh

openat-entry-return-diagnostic-v134d:
	bash tools/run_openat_entry_return_diagnostic_v134d.sh

openat-errno-classification-v135:
	bash tools/run_openat_errno_classification_v135.sh

openat-errno-guard-repair-v135b:
	bash tools/run_openat_errno_guard_repair_v135b.sh

openat-ocreat-source-locator-v136:
	bash tools/run_openat_ocreat_source_locator_v136.sh

openat-ocreat-auto-patch-v137:
	bash tools/run_openat_ocreat_auto_patch_v137.sh

openat-ocreat-context-dump-v137b:
	python3 tools/openat_ocreat_context_dump_v137b.py .repair_logs/openat_ocreat_context_v137b_manual.txt .repair_logs/openat_ocreat_context_v137b_manual.json
