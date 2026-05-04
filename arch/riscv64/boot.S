    .section .text.entry
    .globl _start

_start:
    la t0, boot_message
    call early_uart_puts

    la sp, boot_stack_top
    call rust_main

park:
    wfi
    j park

early_uart_puts:
    li t1, 0x10000000
1:
    lbu t2, 0(t0)
    beqz t2, 2f
    sb t2, 0(t1)
    addi t0, t0, 1
    j 1b
2:
    ret

    .section .rodata
boot_message:
    .asciz "[boot.S] entered v29 umode-trap\n"

    .section .boot_stack, "aw", @nobits
    .align 12
boot_stack:
    .space 4096 * 16
boot_stack_top:
