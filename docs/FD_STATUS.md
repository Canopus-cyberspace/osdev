# FD_STATUS

## v57 - read `/dev/zero` scaffold

Implemented:
- `/dev/null` open/write/close scaffold remains
- `/dev/zero` open/read/close scaffold
- `runtime_read_target`
- `RuntimeReadTarget::{Stdin, DevZero}`
- fd 4 reserved for `/dev/zero`

Still TODO:
- real per-process fd table
- dynamic fd allocation
- VFS-backed path resolution
- fstat/lseek/getdents64
