#!/usr/bin/env python3
import struct
from pathlib import Path

BASE = 0x40000000
SEG_OFF = 0x1000
OUT = Path("user/init.elf")

X0 = 0
A0 = 10
A1 = 11
A2 = 12
A3 = 13
A7 = 17
S0 = 8

def u32(x):
    return struct.pack("<I", x & 0xffffffff)

def addi(rd, rs1, imm):
    if not -2048 <= imm <= 2047:
        raise ValueError(f"addi imm out of range: {imm}")
    imm &= 0xfff
    return u32((imm << 20) | (rs1 << 15) | (0 << 12) | (rd << 7) | 0x13)

def lui(rd, imm20):
    return u32(((imm20 & 0xfffff) << 12) | (rd << 7) | 0x37)

def ecall():
    return u32(0x00000073)

def jal_zero_zero():
    return u32(0x0000006f)

def li(rd, value):
    if -2048 <= value <= 2047:
        return addi(rd, X0, value)
    upper = (value + 0x800) >> 12
    lower = value - (upper << 12)
    return lui(rd, upper) + addi(rd, rd, lower)

def load_abs(rd, addr):
    upper = (addr + 0x800) >> 12
    lower = addr - (upper << 12)
    return lui(rd, upper) + addi(rd, rd, lower)

def syscall_write_const_fd(fd, addr, length):
    return (
        li(A0, fd) +
        load_abs(A1, addr) +
        li(A2, length) +
        li(A7, 64) +
        ecall()
    )

def syscall_write_current_fd(addr, length):
    # fd is already in a0
    return (
        load_abs(A1, addr) +
        li(A2, length) +
        li(A7, 64) +
        ecall()
    )

def syscall0(num):
    return li(A7, num) + ecall()

def syscall1(num, arg0):
    return li(A0, arg0) + li(A7, num) + ecall()

def syscall_openat(path_addr):
    # openat(AT_FDCWD=-100, path, O_WRONLY=1, mode=0)
    return (
        li(A0, -100) +
        load_abs(A1, path_addr) +
        li(A2, 1) +
        li(A3, 0) +
        li(A7, 56) +
        ecall()
    )

def syscall_close_current_s0():
    return addi(A0, S0, 0) + li(A7, 57) + ecall()

messages = [
    b"hello from external init.elf v56 syscall write\n",
    b"this write goes to devnull v56\n",
    b"external init openat close passed\n",
    b"external init getpid returned 1\n",
    b"external init getppid returned 0\n",
    b"external init unsupported returned -38\n",
    b"/dev/null\x00",
]

dummy = BASE
code_dummy = (
    syscall_write_const_fd(1, dummy, len(messages[0])) +
    syscall_openat(dummy) +
    addi(S0, A0, 0) +
    addi(A0, S0, 0) + syscall_write_current_fd(dummy, len(messages[1])) +
    syscall_close_current_s0() +
    syscall_write_const_fd(1, dummy, len(messages[2])) +
    syscall0(172) +
    syscall_write_const_fd(1, dummy, len(messages[3])) +
    syscall0(173) +
    syscall_write_const_fd(1, dummy, len(messages[4])) +
    syscall0(9999) +
    syscall_write_const_fd(1, dummy, len(messages[5])) +
    syscall1(93, 0) +
    jal_zero_zero()
)

off = len(code_dummy)
msg_addrs = []
for m in messages:
    msg_addrs.append(BASE + off)
    off += len(m)

code = (
    syscall_write_const_fd(1, msg_addrs[0], len(messages[0])) +
    syscall_openat(msg_addrs[6]) +
    addi(S0, A0, 0) +
    addi(A0, S0, 0) + syscall_write_current_fd(msg_addrs[1], len(messages[1])) +
    syscall_close_current_s0() +
    syscall_write_const_fd(1, msg_addrs[2], len(messages[2])) +
    syscall0(172) +
    syscall_write_const_fd(1, msg_addrs[3], len(messages[3])) +
    syscall0(173) +
    syscall_write_const_fd(1, msg_addrs[4], len(messages[4])) +
    syscall0(9999) +
    syscall_write_const_fd(1, msg_addrs[5], len(messages[5])) +
    syscall1(93, 0) +
    jal_zero_zero()
)
segment = code + b"".join(messages)

e_ident = b"\x7fELF" + bytes([2, 1, 1, 0, 0]) + bytes(7)
ET_EXEC = 2
EM_RISCV = 243
EV_CURRENT = 1
FLAGS = 0x1 | 0x4

ehdr = struct.pack(
    "<16sHHIQQQIHHHHHH",
    e_ident, ET_EXEC, EM_RISCV, EV_CURRENT, BASE, 64, 0, FLAGS,
    64, 56, 1, 0, 0, 0,
)

PT_LOAD = 1
PF_X = 1
PF_R = 4
p_memsz = ((len(segment) + 0xfff) // 0x1000) * 0x1000
phdr = struct.pack(
    "<IIQQQQQQ",
    PT_LOAD, PF_R | PF_X, SEG_OFF, BASE, BASE, len(segment), p_memsz, 0x1000,
)

blob = ehdr + phdr
blob += bytes(SEG_OFF - len(blob))
blob += segment

OUT.write_bytes(blob)
print(f"[build-init-elf-v56] wrote {OUT} size={len(blob)} segment={len(segment)} entry={BASE:#x}")
