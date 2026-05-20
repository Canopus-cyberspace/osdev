# Architecture Refactor Plan

## Current State

R01-R04 are complete as of 2026-05-16.

- `src/official/*` owns official sdcard evidence helpers, score-group framing, content-backed gates, and official score/evidence facades.
- `src/compat/*` owns v151-v190 once-only compatibility validators and the active-once legacy bridge.
- `src/mm/real_mm.rs` owns REAL-MM page state, allocator/mapping helpers, lazy page-fault handling, real-mm counters, and the real-mm synthetic ELF helpers.
- `src/trap/riscv_asm.rs` owns the RISC-V trap assembly symbols and trap-entry installation wrapper.
- `src/trap/user_entry.rs` owns page-table activation, U-mode entry, and user `sstatus` setup wrappers.
- `src/mm/sv39_init_exec.rs` remains the orchestration file for external init, trap handling, syscall dispatch, page-fault dispatch, and phase transitions.

## Completed Phases

| Phase | Status | Validation |
| --- | --- | --- |
| R01 official split | Complete | `cargo build --target riscv64gc-unknown-none-elf` |
| R02 compat split | Complete | `cargo build --target riscv64gc-unknown-none-elf` |
| R03 real-mm split | Complete | `cargo build`, `make all`, `apply_fix.sh`, official Docker |
| R04 trap/user-entry split | Complete | `cargo build`, `make all`, `apply_fix.sh`, official Docker |

## Guardrails

- Do not change official score behavior.
- Do not downgrade REAL-RUN cases or mark legacy/content-backed cases as REAL-RUN.
- Keep `.repair_logs` small-evidence-only.

## Next Step

Use `prompts/R05_syscall_dispatch_split_prompt.md` for the next bounded phase.
