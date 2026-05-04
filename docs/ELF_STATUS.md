# ELF_STATUS

## v49c

Status: PASS target

Implemented scaffold:

- ELF64 RISC-V header parser
- PT_LOAD program header parser
- synthetic ELF parser self-test
- external `user/init.elf` embedded by `include_bytes!`
- PT_LOAD copy into one kernel load page
- runtime markers verified through QEMU serial-file

Not implemented yet:

- real `execve`
- multi-segment ELF loading into process address space
- argc/argv/envp/auxv user stack construction
