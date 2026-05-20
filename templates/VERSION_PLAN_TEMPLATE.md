# <VERSION> Plan

## Baseline

`<BASELINE_VERSION>`

## Goal

`<ONE_SUBSYSTEM_GOAL>`

## Why this version is next

Explain dependency order.

## Files to inspect

- `<file>`

## Likely files to modify

- `<file>`

## Runtime path

```text
rust_main
-> mm::sv39_init_exec::run_external_init_elf_smoke
-> dispatch_runtime_syscall
-> RuntimeSyscallAction
```

## Implementation scope

- `<scope item>`

## Out of scope

- `<out of scope item>`

## Preserved markers

```text
[ucompat-v151k7] vfs_tree_dirfd_multiinode PASS
[ucompat-v154] fs_core_multi_feature PASS
[ucompat-v155] namespace_procfd_multi_feature PASS
[ucompat-v156] procfs_fd_observability PASS
[ucompat-v157] unified historical kernel integration PASS
```

Add newer markers if baseline is after v157.

## New marker

```text
<NEW_MARKER>
```

## Validation scenario

1. `<step>`
2. `<step>`

## Build/QEMU commands

```bash
source /home/lenovo/miniconda3/etc/profile.d/conda.sh
conda activate osdev
source ~/.cargo/env || true
export PATH="$HOME/.cargo/bin:$PATH"
rustup target add riscv64gc-unknown-none-elf || true

cargo build --target riscv64gc-unknown-none-elf
bash ./tools/run-qemu.sh
```

## Risks

- `<risk>`

## Rollback points

- `<rollback point>`
