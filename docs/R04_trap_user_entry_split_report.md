# R04 Trap/User Entry Split Report

Date: 2026-05-16

## Scope

Implemented R04 only. No feature work, BusyBox expansion, lua/libctest work, LoongArch semantic change, or official score behavior change was included.

## Refactor Summary

- Added `src/trap/riscv_asm.rs` for the RISC-V trap assembly block, `__sv39_init_v50b_alltraps`, `__sv39_init_v50b_restore`, `install_trap_entry`, and the narrow restore wrapper.
- Added `src/trap/user_entry.rs` for `activate_page_table`, `enter_user`, and `user_sstatus`.
- Updated `src/trap/mod.rs` to export the R04 modules.
- Updated `src/main.rs` to include the trap module.
- Kept `src/mm/sv39_init_exec.rs` as the high-level orchestration entry for external init, trap handling, syscall dispatch, page-fault dispatch, and official score flow.

## Preserved Behavior

- Trap ABI, register save/restore layout, `sscratch` stack handoff, `sstatus`/`sepc` restoration, and `sret` return path were preserved.
- U-mode entry behavior was preserved through the moved `enter_user` wrapper.
- Syscall trap handling remains in the existing `rust_sv39_init_v50b_trap_handler` path.
- Page-fault behavior remains delegated through the existing handler and `src/mm/real_mm.rs`.
- Official clean shutdown marker remains present:

```text
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
```

No official group logic, compat/ucompat logic, REAL-MM allocator/page-fault logic, BusyBox RealRun logic, or syscall feature logic was moved in this phase.

## Validation

```text
cargo build --target riscv64gc-unknown-none-elf  PASS
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

R04 direct and official Docker evidence is in:

```text
.repair_logs/B02_B04_real_busybox_broad_applet_execution_20260516_104247/
```

Official Docker log:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_20260516_104436/docker_evaluate.log
```

Key evidence files:

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

## Classification Notes

R04 was a semantic-preserving architecture split. It did not change testcase classifications.

- `basic-musl-rv` remains `100.0`.
- `busybox-musl-rv` remains `53.0`.
- B01-B04 BusyBox promoted applets remain REAL-RUN with RealRunResult evidence.
- Remaining legacy BusyBox shell/redirection/pipeline/kill/more/clear/expr/hwclock/[ -f test.txt ] cases remain legacy/content-backed unless promoted later with verified RealRunResult evidence.
