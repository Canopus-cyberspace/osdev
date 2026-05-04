    .section .text
    .globl _start

_start:
    li a0, 1
    la a1, msg_hello
    la t1, msg_hello_end
    la t2, msg_hello
    sub a2, t1, t2
    li a7, 64
    ecall

    li a7, 172
    ecall

    li t0, 1
    bne a0, t0, getpid_bad

getpid_ok:
    li a0, 1
    la a1, msg_ok
    la t1, msg_ok_end
    la t2, msg_ok
    sub a2, t1, t2
    li a7, 64
    ecall
    j user_exit

getpid_bad:
    li a0, 1
    la a1, msg_bad
    la t1, msg_bad_end
    la t2, msg_bad
    sub a2, t1, t2
    li a7, 64
    ecall

user_exit:
    li a0, 0
    li a7, 93
    ecall

user_loop:
    j user_loop

msg_hello:
    .ascii "hello from user image syscall write\n"
msg_hello_end:

msg_ok:
    .ascii "getpid returned 1\n"
msg_ok_end:

msg_bad:
    .ascii "getpid returned unexpected value\n"
msg_bad_end:
