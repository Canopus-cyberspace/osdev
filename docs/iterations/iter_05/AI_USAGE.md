# Iteration 05 AI Usage

AI assistance was used to inspect existing LoongArch pipe scaffolding, identify missing endpoint ownership semantics, implement pipe refcounts in the fd table, enable the real basic-musl pipe case, and summarize validation evidence.

Accepted AI changes:

- `PipeState` table in `fd_table.rs`.
- Endpoint refcount updates for pipe close, dup, dup3, and fork snapshot/restore.
- `/musl/basic/pipe` case enablement.
- Iteration documentation and concise validation excerpts.

Rejected or deferred suggestions:

- Full blocking pipe semantics were deferred until a scheduler exists.
- `clone` and BusyBox launch were deferred.
- No fake pipe output or hard-coded success line was added.
- No `runtime_dispatch.rs` change was made.

Human/environment verification is still required for an updated official score because Docker was unavailable locally.
