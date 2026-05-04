    .section .text.entry
    .globl _start

_start:
    la a0, boot_msg
    call uart_puts

    la sp, boot_stack_top
    call rust_main

park:
    wfi
    j park

uart_puts:
    li t0, 0x10000000
1:
    lbu t1, 0(a0)
    beqz t1, 2f
    sb t1, 0(t0)
    addi a0, a0, 1
    j 1b
2:
    ret

    .section .rodata
boot_msg:
    .asciz "[boot.S] entered v19\n"

    .section .boot_stack, "aw", @nobits
    .align 12
boot_stack:
    .space 4096 * 16
boot_stack_top:
