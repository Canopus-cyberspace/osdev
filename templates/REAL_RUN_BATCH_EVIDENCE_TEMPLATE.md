# Real-Run Batch Evidence Template

Batch name:

```text
Kxx_real_...
```

Claimed capability:

```text
REAL_RUN / CONTENT_BACKED / PARSER_ONLY
```

Only `REAL_RUN` may be described as real kernel capability.

## Target

```text
program_path:
source_image_or_rootfs:
expected_stdout:
expected_exit_code:
```

## Real execution proof

```text
official sdcard/rootfs read: yes/no
ELF loaded: yes/no
elf_sha256:
elf_entry_pc:
entered_umode: yes/no
first_user_pc:
syscalls observed:
page faults observed:
stdout captured:
stderr captured:
exit_code:
final_task_state:
```

## Negative tests

```text
missing_target_file_must_not_emit_success: PASS/FAIL
wrong_expected_stdout_must_not_emit_success: PASS/FAIL
wrong_expected_exit_code_must_not_emit_success: PASS/FAIL
```

## Non-regression

```text
cargo build --target riscv64gc-unknown-none-elf:
make all:
direct QEMU:
official Docker:
official score:
basic-musl-rv:
busybox-musl-rv:
LoongArch behavior changed: yes/no
```

## Evidence files

```text
real_exec_program_path.txt
real_exec_source_image.txt
real_exec_elf_sha256.txt
real_exec_entry_pc.txt
real_exec_umode_entered.txt
real_exec_syscall_trace.txt
real_exec_page_fault_trace.txt
real_exec_stdout.txt
real_exec_stderr.txt
real_exec_exit_code.txt
real_exec_task_lifecycle.txt
negative_missing_file.log
negative_wrong_stdout.log
negative_wrong_exit_code.log
score_summary.txt
environment_fingerprint.txt
```
