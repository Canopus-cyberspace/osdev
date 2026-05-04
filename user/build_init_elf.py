#!/usr/bin/env python3
import struct
from pathlib import Path

BASE = 0x40000000
SEG_OFF = 0x1000
OUT = Path("user/init.elf")

X0=0
A0=10
A1=11
A2=12
A7=17

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

def syscall_write(addr, length):
    return (
        li(A0, 1) +
        load_abs(A1, addr) +
        li(A2, length) +
        li(A7, 64) +
        ecall()
    )

def syscall0(num):
    return li(A7, num) + ecall()

def syscall1(num, arg0):
    return li(A0, arg0) + li(A7, num) + ecall()

messages = [
    b"hello from external init.elf v50 syscall write\n",
    b"external init getpid returned 1\n",
    b"external init getppid returned 0\n",
    b"external init unsupported returned -38\n",
]

dummy_addr = BASE
code_dummy = (
    syscall_write(dummy_addr, len(messages[0])) +
    syscall0(172) +
    syscall_write(dummy_addr, len(messages[1])) +
    syscall0(173) +
    syscall_write(dummy_addr, len(messages[2])) +
    syscall0(9999) +
    syscall_write(dummy_addr, len(messages[3])) +
    syscall1(93, 0) +
    jal_zero_zero()
)
code_len = len(code_dummy)

msg_addrs = []
off = code_len
for m in messages:
    msg_addrs.append(BASE + off)
    off += len(m)

code = (
    syscall_write(msg_addrs[0], len(messages[0])) +
    syscall0(172) +
    syscall_write(msg_addrs[1], len(messages[1])) +
    syscall0(173) +
    syscall_write(msg_addrs[2], len(messages[2])) +
    syscall0(9999) +
    syscall_write(msg_addrs[3], len(messages[3])) +
    syscall1(93, 0) +
    jal_zero_zero()
)
segment = code + b"".join(messages)

e_ident = b"\x7fELF" + bytes([2, 1, 1, 0, 0]) + bytes(7)

ET_EXEC = 2
EM_RISCV = 243
EV_CURRENT = 1
EF_RISCV_RVC = 0x1
EF_RISCV_FLOAT_ABI_DOUBLE = 0x4
FLAGS = EF_RISCV_RVC | EF_RISCV_FLOAT_ABI_DOUBLE

ehdr = struct.pack(
    "<16sHHIQQQIHHHHHH",
    e_ident,
    ET_EXEC,
    EM_RISCV,
    EV_CURRENT,
    BASE,
    64,
    0,
    FLAGS,
    64,
    56,
    1,
    0,
    0,
    0,
)

PT_LOAD = 1
PF_X = 1
PF_R = 4
p_memsz = ((len(segment) + 0xfff) // 0x1000) * 0x1000
phdr = struct.pack(
    "<IIQQQQQQ",
    PT_LOAD,
    PF_R | PF_X,
    SEG_OFF,
    BASE,
    BASE,
    len(segment),
    p_memsz,
    0x1000,
)

blob = ehdr + phdr
blob += bytes(SEG_OFF - len(blob))
blob += segment

OUT.write_bytes(blob)
print(f"[build-init-elf] wrote {OUT} size={len(blob)} segment={len(segment)} entry={BASE:#x}")
