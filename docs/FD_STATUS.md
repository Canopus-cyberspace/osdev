# FD_STATUS

## v55 - fd-backed write scaffold

Implemented:
- `RuntimeWriteTarget`
- `runtime_write_target`
- stdout/stderr map to console
- fd 3 reserved as `/dev/null` scaffold target
- invalid fd returns `-EBADF` (`-9`)
- external init write now goes through fd-backed write routing

Still TODO:
- real per-process fd table
- openat/close connected to fd table
- read support
- fstat/lseek/getdents64
