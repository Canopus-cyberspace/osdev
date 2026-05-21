# Iteration 04 AI Usage

AI assistance was used to inspect the LoongArch `execve` path, implement safe argv/envp decoding, extend ELF stack construction, enable the real basic-musl `execve` case, and summarize validation evidence.

Accepted AI changes:

- Safe `read_exec_strings` implementation in `syscall.rs`.
- `ExecString` and argv/envp-aware stack construction in `real_elf.rs`.
- Exec-specific snapshot/restore helpers for failed execve.
- Case enablement for `/musl/basic/execve`.
- Iteration documentation and concise test excerpts.

Rejected or deferred suggestions:

- `clone`, `pipe`, and BusyBox launch were deferred.
- No fake `test_execve` output or hard-coded success line was added.
- No `runtime_dispatch.rs` change was made.

Human/environment verification is still required for an updated official score because Docker was unavailable locally.
