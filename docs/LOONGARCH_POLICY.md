# LoongArch Policy

## Current status

`kernel-la` is not a real LoongArch64 kernel.

Do not claim LoongArch support until:

- `kernel-la` is a valid LoongArch64 ELF;
- `qemu-system-loongarch64 -kernel kernel-la ...` loads it;
- it prints boot diagnostics;
- it handles at least one syscall path;
- it can run at least one official or smoke user program.

## Strategy

Prioritize RISC-V score first. Start LoongArch only after the RISC-V basic/libc/busybox path is stable enough.
