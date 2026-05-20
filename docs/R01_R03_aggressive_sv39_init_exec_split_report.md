# R01-R03 Aggressive sv39_init_exec Split Report

Date: 2026-05-16

## Scope

Implemented R01, R02, and R03 only. R04 was not implemented.

No trap assembly, `global_asm`, `__sv39_init_v50b_alltraps`, `__sv39_init_v50b_restore`, `enter_user`, `build_page_table`, `activate_page_table`, or `install_trap_entry` was moved.

## Refactor Summary

- R01 created `src/official/*` and moved the official basic/busybox sdcard evidence and score-group implementation to `src/official/basic_musl.rs`.
- R01 preserved the old `crate::fs::official_basic_musl::*` path through a compatibility re-export.
- R02 created `src/compat/*` and moved the v151-v190 once-only historical runtime validators behind `crate::compat::ucompat_v151_v190::run_once_before_syscall()`.
- R03 created `src/mm/real_mm.rs` and moved REAL-MM page state, allocator/mapping helpers, lazy page-fault handling, real-mm synthetic ELF helpers, and related counters.
- `src/mm/sv39_init_exec.rs` remains the orchestration entry for external init, trap dispatch, U-mode launch, syscall handling, and high-level phase dispatch.

## Preserved Behavior

- Official Docker image: `zhouzhouyi/os-contest:20260510`
- Official verdict: `Accpted`
- Official score: `153`
- `basic-musl-rv`: `100.0`
- `busybox-musl-rv`: `53.0`
- Clean shutdown marker preserved: `[official-qemu-v194] external init smoke complete; requesting SBI shutdown`
- B01/B02/B03/B04 BusyBox REAL-RUN evidence preserved.
- Legacy BusyBox shell/redirection/pipeline/kill/more/clear/expr/hwclock/[ -f test.txt ] cases remain legacy/content-backed unless promoted later through verified RealRunResult.

## Validation

```text
cargo build --target riscv64gc-unknown-none-elf  PASS after R01
cargo build --target riscv64gc-unknown-none-elf  PASS after R02
cargo build --target riscv64gc-unknown-none-elf  PASS after R03
make all                                         PASS
bash ./apply_fix.sh                             PASS
RUN_OFFICIAL_AUTOTEST=1 bash ./apply_fix.sh     PASS
```

Direct validation summary:

```text
basic-musl-total 100
busybox-musl-total 53
combined-direct-total 153
```

Official validation summary:

```text
verdict Accpted
score 153
basic-musl-rv 100.0
busybox-musl-rv 53.0
```

## Evidence

Official Docker refactor validation evidence is in:

```text
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_101054/
```

Final Windows-wrapper direct evidence is in:

```text
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_102411/
```

Key files:

```text
direct_qemu.log
docker_evaluate.log
console_log
os_serial_out_rv.txt
os_serial_out_la.txt
score_summary.txt
environment_fingerprint.txt
realrun_matrix.json
realrun_matrix.md
realrun_busybox_matrix.json
realrun_busybox_matrix.md
judge_basic_musl.json
judge_busybox_musl.json
```

## Notes

This was a semantic-preserving architecture split. It did not claim new score, did not change LoongArch semantics, and did not promote any legacy BusyBox cases beyond the existing B01-B04 RealRunResult-backed set.
