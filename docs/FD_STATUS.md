# FD_STATUS

## v56 - openat/close `/dev/null` scaffold

Implemented:
- runtime `/dev/null` open state
- `runtime_open_devnull`
- `runtime_close_fd`
- `runtime_write_target` checks fd state
- fd 1/2 route to console
- fd 3 routes to `/dev/null` only after openat
- invalid fd returns `-EBADF`

Still TODO:
- real per-process fd table
- dynamic fd allocation
- VFS-backed path resolution
- read/fstat/lseek/getdents64
