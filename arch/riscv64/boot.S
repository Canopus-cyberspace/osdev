    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call boot_uart_marker
    call rust_main
1:
    wfi
    j 1b

boot_uart_marker:
    li t0, 0x10000000
    la t1, boot_msg
2:
    lbu t2, 0(t1)
    beqz t2, 3f
    sb t2, 0(t0)
    addi t1, t1, 1
    j 2b
3:
    ret

    .section .rodata
boot_msg:
    .asciz "[boot.S] entered v46e elf-loader\n"

    .section .boot_stack, "aw", @nobits
    .align 12
    .globl boot_stack
boot_stack:
    .space 65536
    .globl boot_stack_top
boot_stack_top:
