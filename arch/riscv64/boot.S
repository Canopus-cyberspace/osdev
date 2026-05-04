    .section .text.entry
    .globl _start

_start:
    li t0, 0x10000000
    la t1, boot_msg
1:
    lb t2, 0(t1)
    beqz t2, 2f
    sb t2, 0(t0)
    addi t1, t1, 1
    j 1b
2:
    la sp, boot_stack_top
    call rust_main

park:
    wfi
    j park

boot_msg:
    .asciz "[boot.S] entered v24 serial-file\n"

    .section .boot_stack, "aw", @nobits
    .align 12
boot_stack:
    .space 4096 * 16
boot_stack_top:
