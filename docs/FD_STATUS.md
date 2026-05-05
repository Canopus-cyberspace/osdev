# FD_STATUS

## v58 - fstat/lseek scaffold

Implemented:
- `/dev/null` open/write/close scaffold remains
- `/dev/zero` open/read/close scaffold remains
- `runtime_fd_kind`
- `runtime_fstat_result`
- `runtime_lseek_result`
- character devices return `-ESPIPE` for lseek
- fstat validates fd and writes a minimal stat buffer

Still TODO:
- real per-process fd table
- dynamic fd allocation
- VFS-backed path resolution
- complete stat layout
- getdents64
