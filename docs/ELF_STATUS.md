# ELF_STATUS

## v48 - external init ELF scaffold

- user/init.S scaffold source
- user/init.ld scaffold linker script
- user/init.elf synthetic external ELF image
- src/loader/init_image.rs using include_bytes!
- ELF64/RISC-V header checks
- PT_LOAD metadata checks
