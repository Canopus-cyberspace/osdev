# Iteration 16 Summary

## Goal

Promote only direct LoongArch BusyBox applets that do not depend on scratch files, redirection, pipelines, or shell-complex command parsing.

The baseline entering the iteration was:

```text
Official score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
LoongArch basic local smoke: attempted=32 completed=32 failed=none
LoongArch BusyBox smoke: completed=7 attempted=7 matched=7 failed=0 disabled=3
```

## Outcome

`src/arch/loongarch64/busybox_runner.rs` now promotes nine additional real BusyBox direct applets after local bounded execution:

- `basename /aaa/bbb`
- `printf "abc\n"`
- `uname`
- `dirname /aaa/bbb`
- `expr 1 + 1`
- `date`
- `uptime`
- `clear`
- `cal`

The existing five scoring commands remain:

- `true`
- `false`
- `pwd`
- `sh -c exit`
- `ls`

Smoke-only commands remain:

- `echo hello`
- `cat /musl/busybox_cmd.txt`

Disabled commands after local probing:

- `ash -c exit`: previous diagnostic fault at `ERA=0x1201b64b8`, `BADV=0x1201b64b8`, `ECODE=15`
- `which ls`: exited with code 1 during local probing
- `free`: exited with code 1 because `/proc/meminfo` is unavailable
- `sleep 1`: produced a controlled user fault during local probing

No scratch-FS, redirection, grep, pipeline, fd-table, syscall, or runtime dispatcher changes were made.

## Module Placement

The only source change is in `busybox_runner.rs`, which already owns BusyBox command classes, official markers, per-command execution, and result emission. A new file was considered unnecessary because this iteration changes only command classification and argv/official-name descriptors.

Existing helpers reused:

- `real_elf::load_user_elf_with_args`
- `real_elf::activate_current_user_mmu`
- `trap::enter_user_entry`
- `user::reset_case_state`
- `user::start_syscall_budget`
- `user::run_snapshot`
- `fd_table::set_cwd`
- `emit_official_result`

New helpers added: none.

Future search terms:

```text
BusyboxCommandClass::Scoring
basename /aaa/bbb
printf "abc\n"
dirname /aaa/bbb
expr 1 + 1
disabled=4
```

## Validation

Local validation:

```text
CPU_COUNT: 16
cargo build -j 16 --target riscv64gc-unknown-none-elf: passed
make -j 16 all: passed with the known local jobserver warning
make all: passed
kernel-rv: RISC-V ELF
kernel-la: LoongArch ELF
LoongArch local basic: attempted=32 completed=32 failed=none
LoongArch local BusyBox: completed=16 attempted=16 matched=16 failed=0 disabled=4
```

Official validation:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260522_123151/docker_evaluate.log
docker_evaluate.log size: 802896 bytes
Verdict: Accpted
Score: 269
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 14.0
```

