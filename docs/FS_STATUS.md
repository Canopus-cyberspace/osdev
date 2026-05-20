# FS_STATUS

## v68 - filesystem metadata scaffold

Implemented:
- mkdirat returns 0
- unlinkat returns 0
- faccessat returns 0
- newfstatat writes a minimal stat structure
- statx writes a minimal statx structure
- renameat2 returns 0

Still TODO:
- real inode namespace
- real directory mutations
- real permission checks
- real per-file metadata
- persistent filesystem backend
