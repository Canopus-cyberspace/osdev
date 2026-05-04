    .section .text.entry
    .globl _start

_start:
    la t1, boot_msg
1:
    lbu t2, 0(t1)
    beqz t2, 2f
    li t0, 0x10000000
    sb t2, 0(t0)
    addi t1, t1, 1
    j 1b

2:
    la sp, boot_stack_top
    call rust_main

park:
    wfi
    j park

    .section .rodata
boot_msg:
    .asciz "[boot.S] entered v22 serial phdr\n"

    .section .boot_stack, "aw", @nobits
    .align 12
boot_stack:
    .space 4096 * 16
boot_stack_top:
