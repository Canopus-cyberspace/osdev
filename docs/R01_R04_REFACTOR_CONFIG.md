# R01-R04 Refactor Configuration

## Current Stable Baseline

Project:

```text
/home/lenovo/projects/uestc-kernel
```

Preserved official scoreline:

```text
Docker: zhouzhouyi/os-contest:20260510
autotest HEAD: 500e7edcfb875409a0babe125d273ab30771d5ec
score: 153
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
```

Known evidence baseline:

```text
P07 official Docker baseline:
score: 152
basic-musl-rv: 100.0
busybox-musl-rv: 52.0

B01 direct RealRun baseline:
basic-musl-rv: 100.0 preserved
busybox-musl-rv: 53.0 preserved
```

B01 REAL-RUN BusyBox applets:

```text
/musl/busybox true
/musl/busybox echo "#### independent command test"
/musl/busybox pwd
/musl/busybox ls
/musl/busybox cat test.txt
```

Remaining BusyBox cases remain `LEGACY-CONTENT-BACKED` unless explicitly promoted with verified RealRunResult evidence.

---

## Goal

Refactor the oversized file:

```text
src/mm/sv39_init_exec.rs
```

into fixed architecture modules without semantic change.

This is a refactor governance/configuration plan, not a score-claim batch.

---

## Global Rules

All phases must obey:

```text
Do not change official score behavior.
Do not remove existing score paths.
Do not remove REAL-RUN gates.
Do not mark legacy/content-backed cases as REAL-RUN.
Do not modify LoongArch semantics.
Do not implement broad v195-v200 work.
Do not fake official output.
Do not remove v151k7-v194 markers.
Do not remove the official clean shutdown marker.
Do not copy large artifacts into .repair_logs.
```

Required marker:

```text
[official-qemu-v194] external init smoke complete; requesting SBI shutdown
```

Repair-log policy:

```text
.repair_logs must remain small-evidence-only.

Allowed:
docker_evaluate.log
console_log
os_serial_out_rv.txt
os_serial_out_la.txt
basic_musl_group.txt
busybox_musl_group.txt
judge_basic_musl.json
judge_busybox_musl.json
score_summary.txt
environment_fingerprint.txt
RealRun matrix/evidence text/json files

Forbidden:
raw sdcard images
sdcard release archives
kernel-rv / kernel-la binaries
ELF payload dumps
target trees
Docker data
full repository copies
large root artifact directories
```

Files larger than 50MB must be represented by path + size + sha256 + metadata.

---

## Target Module Layout

```text
src/
  official/
    mod.rs
    basic_musl.rs
    busybox.rs
    busybox_realrun.rs
    evidence.rs
    score_groups.rs

  compat/
    mod.rs
    ucompat_v151_v190.rs
    legacy_fd_vfs.rs

  mm/
    real_mm.rs
    sv39_init_exec.rs

  trap/
    riscv_asm.rs
    user_entry.rs
    context.rs
    handler.rs

  task/
    realrun.rs
```

Reuse existing project structure. Do not create a second parallel architecture.

---

## R01: Official Module Split

Create:

```text
src/official/mod.rs
src/official/basic_musl.rs
src/official/busybox.rs
src/official/busybox_realrun.rs
src/official/evidence.rs
src/official/score_groups.rs
```

Move only:

```text
basic-musl official group helpers
busybox official group helpers
busybox_cmd.txt / busybox_testcode.sh verification helpers
B01 BusyBox RealRunResult matrix and evidence helpers
negative checks
score/evidence formatting helpers
```

Do not move in R01:

```text
global_asm trap entry
TrapContext
enter_user
build_page_table
activate_page_table
real_mm allocator
page fault handler
syscall dispatch core
ucompat-v151-v190 historical bridge
```

Validation:

```bash
cargo build --target riscv64gc-unknown-none-elf
make all
bash ./apply_fix.sh
```

Required result:

```text
basic-musl judge remains 100
busybox-musl scoreline remains at least 53
B01 REAL-RUN evidence remains present
official clean shutdown marker remains present
```

---

## R02: Compat Module Split

Create:

```text
src/compat/mod.rs
src/compat/ucompat_v151_v190.rs
src/compat/legacy_fd_vfs.rs
```

Move:

```text
ucompat-v151 through ucompat-v190 once-only validators
active_once bridge
legacy fd/vfs helper functions
disabled direct intercept blocks
```

Leave a narrow call in syscall path:

```rust
crate::compat::ucompat_v151_v190::run_once_before_syscall();
```

Validation must preserve all historical markers, basic-musl 100, busybox scoreline, and clean shutdown.

---

## R03: Real-MM Module Split

Create:

```text
src/mm/real_mm.rs
```

Move:

```text
REAL_MM_PAGES
REAL_MM_PAGE_STATE
REAL_MM_MAPPINGS
real_mm_* helpers
handle_real_mm_page_fault
```

Validation must preserve brk/mmap/munmap/mprotect/page-fault behavior.

---

## R04: Trap/User Entry Split

Create:

```text
src/trap/riscv_asm.rs
src/trap/user_entry.rs
```

Move:

```text
global_asm!
TrapContext if not already in trap/context.rs
install_trap_entry
activate_page_table
enter_user
restore entry binding
```

This is the riskiest phase and must only run after R01-R03 are stable.

Validation:

```bash
cargo build --target riscv64gc-unknown-none-elf
make all
bash ./apply_fix.sh
RUN_OFFICIAL_AUTOTEST=1 bash ./apply_fix.sh
```

Required result: QEMU boots, U-mode enters, syscall trap works, page fault works, official clean shutdown remains, basic-musl-rv remains 100, busybox scoreline is preserved.

---

## Done Definition

A phase is complete only when:

```text
1. Code compiles.
2. make all passes.
3. apply_fix.sh passes.
4. basic-musl remains 100.
5. busybox scoreline is preserved.
6. B01 REAL-RUN evidence is preserved.
7. No legacy/content-backed case is mislabeled as REAL-RUN.
8. official clean shutdown marker remains.
9. .repair_logs remains small.
10. Report doc is produced.
```

---

## Failure Handling

If any phase fails:

```text
Do not continue to next phase.
Do not add new features.
Do not modify LoongArch.
Do not fake output.
Do not delete evidence.
```

If failure is caused by stale sdcard artifacts, remove generated raw/project-root artifacts only:

```bash
rm -f sdcard-rv.img sdcard-la.img
rm -f sdcard-rv.img.gz sdcard-la.img.gz
rm -f sdcard-rv.img.xz sdcard-la.img.xz
rm -f sdcard-rv.img.part sdcard-la.img.part

rm -f /home/lenovo/oscomp-official-env/testdata/sdcard-rv.img
rm -f /home/lenovo/oscomp-official-env/testdata/sdcard-la.img
```

Do not delete:

```text
/home/lenovo/oscomp-official-env/testdata/sdcard-rv.img.gz
/home/lenovo/oscomp-official-env/testdata/sdcard-la.img.gz
```
