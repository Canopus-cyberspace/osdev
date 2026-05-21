# Iteration 05 Development Log

## Feature Discovery

- Feature: LoongArch pipe support for `/musl/basic/pipe`.
- Owning subsystem: `fd_table.rs`, because pipe state is fd-owned shared object state with close, dup, and fork interactions.
- Existing code searched and reused first: `create_pipe_pair`, `read_pipe`, `write_pipe`, `close_fd`, `dup_fd`, `dup3_fd`, `save_fd_snapshot`, and `restore_fd_snapshot_after_child`.
- New source file considered: no. `fd_table.rs` already owns fd allocation, fd kinds, close, dup, cwd, and fork fd snapshots.
- Future search terms: `PipeState`, `PIPES`, `create_pipe_pair`, `write_pipe`, `read_pipe`, `add_pipe_refs_for_current_table`, `release_pipe_refs_for_current_table`, `FD_PIPE_READ`, `FD_PIPE_WRITE`.

## Implementation Notes

The previous pipe path had a single global active flag and read/write positions. It could move bytes, but it did not model endpoint ownership. That was risky for fork-dependent use because a child can close one endpoint while the parent still owns its corresponding endpoint.

The new `PipeState` stores:

- active flag
- shared byte buffer
- read and write positions
- read endpoint reference count
- write endpoint reference count

Pipe fd entries use their inode field as a local pipe id. Duplicating a pipe fd increments the corresponding endpoint refcount. Closing a pipe fd decrements it. During fork, `save_fd_snapshot` increments pipe refs for the child copy, and `restore_fd_snapshot_after_child` releases any remaining child-side refs before restoring the parent fd table.

Read returns available data and then compacts or resets the buffer. If no data is available, it returns 0 instead of blocking. Write returns an fd error when no read endpoint is open.

## Case Enablement

`/musl/basic/pipe` is now enabled as a real ELF loaded from `sdcard-la.img` and entered in PLV3. The case output is produced by the user program.

## Deferred Work

`clone` remains disabled. More complete blocking semantics can be added when a real scheduler is introduced.
