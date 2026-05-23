# Iteration 11 Summary

## Focus

This iteration closed the LoongArch basic-musl scoring gap from the previous official `98.0` by fixing the real `waitpid` status semantics.

The previous successful official log showed exactly one LoongArch basic-musl miss:

```text
test_waitpid: LoongArch 0/4
```

The LoongArch output was:

```text
========== START test_waitpid ==========
This is child process
waitpid error.
========== END test_waitpid ==========
```

RISC-V output and the judge expected:

```text
waitpid successfully.
wstatus: 3
```

## Source Changes

- Reused `src/arch/loongarch64/process.rs` for process lifecycle and wait status semantics.
- Reused `user_mem::copy_to_user` for wait status writeback.
- Reused `src/arch/loongarch64/user_mmu.rs`, `real_elf.rs`, and `trap.rs` for a narrow LoongArch PLV3 entry/return synchronization fix needed to keep BusyBox stable after the now-successful waitpid path.

No new source file was needed. Process wait status belongs in `process.rs`; LoongArch MMU/TLB synchronization belongs in `user_mmu.rs` and the PLV3 return hook in `trap.rs`.

## Behavior

`sys_exit` child recording now stores only the low 8-bit exit code, and `wait4` writes Linux-compatible exited-child status:

```text
status = (exit_code & 0xff) << 8
```

This lets the real `/musl/basic/waitpid` program observe `WEXITSTATUS(status) == 3`.

## Result

Local smoke:

```text
waitpid successfully.
wstatus: 3
[loongarch64-basic] attempted=32 completed=32 failed=none
[loongarch64-busybox] smoke completed=7 attempted=7 matched=7 failed=0
```

Official validation:

```text
Verdict: Accpted
Score: 260
basic-musl-rv: 100.0
busybox-musl-rv: 53.0
basic-musl-la: 102.0
busybox-musl-la: 5.0
```

The official `test_waitpid` row is now full credit:

```text
test_waitpid: 4/4 for LoongArch
```

The official log contained no `Failed to load ELF`, `panic`, `timeout`, `ENOSYS`, or `user fault` marker.
