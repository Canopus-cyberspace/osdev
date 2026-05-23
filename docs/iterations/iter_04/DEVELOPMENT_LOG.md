# Iteration 04 Development Log

## Feature Discovery

- Feature: LoongArch `execve` for the real basic-musl case.
- Owning subsystems:
  - `syscall.rs`: syscall ABI, user pointer decoding, and errno returns.
  - `process.rs`: current process image replacement and register handoff.
  - `real_elf.rs`: ELF loading and user stack construction.
  - `user_mem.rs`: safe user pointer helpers.
  - `basic_runner.rs`: case enablement.
- Existing code searched and reused first: `read_user_cstr`, `copy_from_user`, `load_basic_case`, `build_stack`, `exec_current`, and the existing basic-musl case table.
- New source file considered: no. Existing focused modules already owned each responsibility.
- Future search terms: `read_exec_strings`, `ExecString`, `load_basic_case_with_args`, `save_exec_snapshot`, `restore_exec_snapshot`, `SYS_EXECVE`.

## Implementation Notes

`syscall_execve` now reads the executable path, argv vector, and envp vector from user memory using safe helpers. The vectors are bounded by small fixed limits suitable for the current basic-musl case and future small startup tests.

`real_elf.rs` gained `ExecString` and `load_basic_case_with_args`. The stack builder now copies argv/envp strings onto the new user stack, writes argc, argv, envp, and auxv, and keeps the stack aligned.

`process.rs` now saves an exec-specific user snapshot before loading the replacement image. On failure it restores that snapshot and returns `-ENOENT`; on success it updates ERA, user SP, and a0 so execution continues in the new program rather than returning to the old image.

## Validation Finding

The real `/musl/basic/execve` ELF successfully execs `/musl/basic/test_echo`, which prints the lines expected by the official judge:

```text
  I am test_echo.
execve success.
```

## Deferred Work

`clone`, `pipe`, and BusyBox launch remain separate future iterations.
