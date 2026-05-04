# SYSCALL_STATUS

## v53d

Status:
- syscall scaffold exists and compiles in `src/syscall/mod.rs`
- runtime self-test is temporarily compile-only
- external init ELF syscall path remains the primary QEMU regression

Runtime verified:
- write
- getpid
- getppid
- unsupported -> -38
- exit

Reason for compile-only scaffold:
- v53c showed runtime self-tests before external init can interfere with the stable external init smoke.
- Next step should connect the external init trap path to the central syscall dispatcher directly.
