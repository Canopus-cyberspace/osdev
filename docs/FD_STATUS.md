# FD_STATUS

## v59 - getdents64 `/dev` scaffold

Implemented:
- `/dev` directory open/close scaffold
- fd 5 reserved for `/dev`
- `runtime_getdents_kind`
- `RuntimeFdKind::DevDir`
- getdents64 validates directory fd
- `/dev` entries exposed:
  - `.`
  - `..`
  - `null`
  - `zero`

Still TODO:
- real per-process fd table
- dynamic fd allocation
- VFS-backed path resolution
- real directory cursor offsets
- root directory and procfs/tmpfs
