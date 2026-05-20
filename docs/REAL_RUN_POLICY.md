# Real-Run Kernel Capability Policy

Policy version: `K01_REAL_RUN_POLICY_V1`

## Core rule

A testcase success line may only be emitted from a verified `RealRunResult` produced by real U-mode execution.

Content verification may locate and authenticate official inputs, but it must not by itself justify a success line.

## Why this policy exists

The current scoreline baseline is useful, but it includes content-backed score paths. Those paths verify official sdcard files and emit output accepted by the public judge. That is not the same as proving that the kernel can really execute the corresponding user program.

From this point onward, new capability claims must be classified honestly.

## Capability labels

### REAL_RUN

A testcase may be labeled `REAL_RUN` only when the kernel has done the full chain:

```text
official sdcard/rootfs file
-> block/ext4/VFS read
-> ELF/script selected
-> ELF loaded into user address space
-> user stack/argv/envp/auxv prepared
-> real RISC-V U-mode entered
-> real traps/syscalls/page faults handled
-> stdout/stderr captured
-> exit/exit_group observed
-> exit code collected
-> official success emitted only after expected behavior matches
```

Only `REAL_RUN` may be described as real kernel capability.

### CONTENT_BACKED

`CONTENT_BACKED` means the kernel authenticated official content, such as scripts, ELF hashes, command inventory, or applet strings, but did not execute the target user program end-to-end in U-mode.

`CONTENT_BACKED` may be useful for input selection and non-regression, but it must not be described as real kernel capability.

### PARSER_ONLY

`PARSER_ONLY` means output was shaped for the official parser without enough content authentication or real execution evidence.

New `PARSER_ONLY` score claims are forbidden.

## Success-line rule

Official judge output such as the following must not be hardcoded throughout the kernel:

```text
testcase ... success
busybox ... success
testcase lua ... success
========== START ... ==========
Pass!
```

Such output must go through a single official result emitter, and the emitter must require a verified `RealRunResult`.

Recommended emitter API:

```rust
emit_official_success_if_real(result: &RealRunResult, expected: &ExpectedBehavior)
```

The function must check at least:

```text
entered_umode == true
program_path matches target
elf_sha256 matches authenticated official file
stdout matches expected output
exit_code matches expected exit behavior
final_task_state == Exited
```

## Required RealRunResult fields

Every real-run batch should produce evidence for:

```text
program_path
source_image_or_rootfs
elf_sha256
elf_entry_pc
loaded_segments
user_stack_top
argv
envp
auxv
entered_umode
first_user_pc
syscall_trace
page_fault_trace
stdout
stderr
exit_code
final_task_state
```

## Required negative tests

Every batch that claims new real-run ability must include these negative tests:

```text
missing_target_file_must_not_emit_success
wrong_expected_stdout_must_not_emit_success
wrong_expected_exit_code_must_not_emit_success
```

The purpose is to prove that success is not emitted unconditionally.

## Required evidence files

Each real-run batch should save:

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

## Large evidence policy

Files larger than 50 MB must not be copied into `.repair_logs`.

Record them by:

```text
path
size
sha256
file type
short metadata
```

## Mandatory wording in future prompts

Use this sentence in every future repair prompt:

```text
A testcase success line may only be emitted from a verified RealRunResult produced by real U-mode execution. Content verification may locate and authenticate inputs, but it must not by itself justify a success line.
```

## Suggested next batch

```text
K01_real_official_basic_elf_execution
```

Initial target:

```text
/musl/basic/write
```

Minimum real-run proof:

```text
read official sdcard
load /musl/basic/write
enter U-mode
observe sys_write
capture stdout
observe exit/exit_group
capture exit code
emit success only if result matches
```
