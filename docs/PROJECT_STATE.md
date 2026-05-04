# PROJECT_STATE

## Current Branch

feature/restore-umode

## Completed Milestones

- v24: stable full mechanism skeleton passed
- v25: warning cleanup and QEMU serial-file runner passed
- v28: minimal U-mode syscall smoke test passed
- v29: U-mode trap path refactor passed
- v30c: U-mode syscall matrix passed
- v31: user-copy abstraction added, and sys_write uses copy_from_user

## Verified Capabilities

- QEMU boots through OpenSBI
- boot.S enters rust_main
- serial-file logging works
- frame allocator test passes
- page table map/translate test passes
- direct user-copy helper test passes
- U-mode ecall works
- syscall write works through copy_from_user path
- syscall getpid works
- unsupported syscall returns -38 ENOSYS
- syscall exit path works

## Current Constraints

- Sv39 is not enabled
- copy_from_user/copy_to_user currently use direct addresses
- ELF loader is not implemented
- VFS is stub
- process/thread/scheduler are stub
- rootfs is not implemented
- signal/futex/timer are stub

## Current QEMU Method

- Use serial file logging
- Logs are stored in `.repair_logs/`

## Next Planned Step

v32:
- add UserAddressSpace scaffold
- separate kernel page-table test from future user address-space test
- keep Sv39 disabled
- keep U-mode syscall matrix passing
