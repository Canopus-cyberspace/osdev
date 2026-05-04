    .section .text.entry
    .globl _start

_start:
    la sp, boot_stack_top
    call rust_main

park:
    wfi
    j park

    .section .bss.stack
    .align 12
boot_stack:
    .space 4096 * 16
boot_stack_top: