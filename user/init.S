/* v48 scaffold external init source. */
    .section .text.entry,"ax",@progbits
    .globl _start
_start:
    li a7, 64
    li a0, 1
    la a1, msg
    la t0, msg_end
    sub a2, t0, a1
    ecall
    li a7, 93
    li a0, 0
    ecall
1:  j 1b

    .section .rodata,"a",@progbits
msg:
    .ascii "hello from external init v48\n"
msg_end:
