    .section .user.text, "ax"
    .align 12
    .globl __user_v45_start
__user_v45_start:
    # write(1, msg_write, len)
    li a0, 1
    la a1, msg_write
    la t0, msg_write_end
    sub a2, t0, a1
    li a7, 64
    ecall

    # getpid()
    li a7, 172
    ecall
    li t0, 1
    bne a0, t0, pid_bad
pid_ok:
    li a0, 1
    la a1, msg_pid_ok
    la t0, msg_pid_ok_end
    sub a2, t0, a1
    li a7, 64
    ecall
    j getppid_test
pid_bad:
    li a0, 1
    la a1, msg_pid_bad
    la t0, msg_pid_bad_end
    sub a2, t0, a1
    li a7, 64
    ecall

getppid_test:
    # getppid()
    li a7, 173
    ecall
    bnez a0, ppid_bad
ppid_ok:
    li a0, 1
    la a1, msg_ppid_ok
    la t0, msg_ppid_ok_end
    sub a2, t0, a1
    li a7, 64
    ecall
    j enosys_test
ppid_bad:
    li a0, 1
    la a1, msg_ppid_bad
    la t0, msg_ppid_bad_end
    sub a2, t0, a1
    li a7, 64
    ecall

enosys_test:
    # unsupported syscall -> -38
    li a7, 9999
    ecall
    li t0, -38
    bne a0, t0, enosys_bad
enosys_ok:
    li a0, 1
    la a1, msg_enosys_ok
    la t0, msg_enosys_ok_end
    sub a2, t0, a1
    li a7, 64
    ecall
    j exit_test
enosys_bad:
    li a0, 1
    la a1, msg_enosys_bad
    la t0, msg_enosys_bad_end
    sub a2, t0, a1
    li a7, 64
    ecall

exit_test:
    li a0, 0
    li a7, 93
    ecall

1:
    wfi
    j 1b

    .align 3
msg_write:
    .ascii "hello from sv39 umode v45 syscall write\n"
msg_write_end:
msg_pid_ok:
    .ascii "umode getpid returned 1\n"
msg_pid_ok_end:
msg_pid_bad:
    .ascii "umode getpid unexpected\n"
msg_pid_bad_end:
msg_ppid_ok:
    .ascii "umode getppid returned 0\n"
msg_ppid_ok_end:
msg_ppid_bad:
    .ascii "umode getppid unexpected\n"
msg_ppid_bad_end:
msg_enosys_ok:
    .ascii "unsupported syscall returned -38\n"
msg_enosys_ok_end:
msg_enosys_bad:
    .ascii "unsupported syscall unexpected\n"
msg_enosys_bad_end:

    .align 12

    .section .user.text, "ax"
    .align 12
    .globl __user_v45_end
__user_v45_end:
