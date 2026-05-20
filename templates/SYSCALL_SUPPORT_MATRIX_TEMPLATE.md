# Syscall Support Matrix

| Syscall | Official suites | Status | Semantics level | Notes |
|---|---|---|---|---|
| getpid | basic-musl |  |  |  |
| getppid | basic-musl |  |  |  |
| uname | basic-musl |  |  |  |
| getcwd | basic-musl/busybox |  |  |  |
| open/openat | basic-musl/busybox/ltp |  |  |  |
| read/write | all |  |  |  |
| fstat/stat/getdents | basic/busybox |  |  |  |
| mmap/brk | libc/LTP |  |  |  |
| fork/exec/wait | busybox/LTP |  |  |  |
| signal | libc/LTP |  |  |  |
| futex | libc/LTP |  |  |  |

Semantics levels:

- unsupported
- marker-only
- content-backed
- canonical-runtime
- real-user-execution
- official-scored
