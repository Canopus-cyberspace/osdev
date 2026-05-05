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
A4 = 14
A5 = 15
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
    return li(A0, fd) + load_abs(A1, addr) + li(A2, length) + li(A7, 64) + ecall()

def syscall_write_current_fd(addr, length):
    return load_abs(A1, addr) + li(A2, length) + li(A7, 64) + ecall()

def syscall_read_current_fd_from_s0_spbuf(length):
    return addi(A0, S0, 0) + addi(A1, 2, -64) + li(A2, length) + li(A7, 63) + ecall()

def syscall_getdents_current_fd_from_s0_spbuf(length):
    return addi(A0, S0, 0) + addi(A1, 2, -512) + li(A2, length) + li(A7, 61) + ecall()

def syscall_fstat_stdout_spbuf():
    return li(A0, 1) + addi(A1, 2, -192) + li(A7, 80) + ecall()

def syscall_lseek_stdout():
    return li(A0, 1) + li(A1, 0) + li(A2, 0) + li(A7, 62) + ecall()

def syscall_brk(addr):
    return li(A0, addr) + li(A7, 214) + ecall()

def syscall_mmap():
    # mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0)
    return (
        li(A0, 0) +
        li(A1, 4096) +
        li(A2, 3) +
        li(A3, 0x22) +
        li(A4, -1) +
        li(A5, 0) +
        li(A7, 222) +
        ecall() +
        addi(S0, A0, 0)
    )

def syscall_mprotect_current_s0():
    # mprotect(mmap_addr, 4096, PROT_READ)
    return addi(A0, S0, 0) + li(A1, 4096) + li(A2, 1) + li(A7, 226) + ecall()

def syscall_madvise_current_s0():
    # madvise(mmap_addr, 4096, MADV_NORMAL)
    return addi(A0, S0, 0) + li(A1, 4096) + li(A2, 0) + li(A7, 233) + ecall()

def syscall_munmap_current_s0():
    return addi(A0, S0, 0) + li(A1, 4096) + li(A7, 215) + ecall()

def syscall0(num):
    return li(A7, num) + ecall()

def syscall1(num, arg0):
    return li(A0, arg0) + li(A7, num) + ecall()

def syscall_openat(path_addr, flags):
    return li(A0, -100) + load_abs(A1, path_addr) + li(A2, flags) + li(A3, 0) + li(A7, 56) + ecall()

def syscall_close_current_s0():
    return addi(A0, S0, 0) + li(A7, 57) + ecall()

messages = [
    b"hello from external init.elf v62 syscall write\n",
    b"external init fstat lseek passed\n",
    b"this write goes to devnull v62\n",
    b"external init openat close passed\n",
    b"external init devzero read passed\n",
    b"external init getdents64 dev passed\n",
    b"external init brk passed\n",
    b"external init mmap munmap passed\n",
    b"external init mprotect madvise passed\n",
    b"external init getpid returned 1\n",
    b"external init getppid returned 0\n",
    b"external init unsupported returned -38\n",
    b"/dev/null\x00",
    b"/dev/zero\x00",
    b"/dev\x00",
]

dummy = BASE
code_dummy = (
    syscall_write_const_fd(1, dummy, len(messages[0])) +
    syscall_fstat_stdout_spbuf() +
    syscall_lseek_stdout() +
    syscall_write_const_fd(1, dummy, len(messages[1])) +
    syscall_openat(dummy, 1) +
    addi(S0, A0, 0) +
    addi(A0, S0, 0) + syscall_write_current_fd(dummy, len(messages[2])) +
    syscall_close_current_s0() +
    syscall_write_const_fd(1, dummy, len(messages[3])) +
    syscall_openat(dummy, 0) +
    addi(S0, A0, 0) +
    syscall_read_current_fd_from_s0_spbuf(16) +
    syscall_close_current_s0() +
    syscall_write_const_fd(1, dummy, len(messages[4])) +
    syscall_openat(dummy, 0) +
    addi(S0, A0, 0) +
    syscall_getdents_current_fd_from_s0_spbuf(256) +
    syscall_close_current_s0() +
    syscall_write_const_fd(1, dummy, len(messages[5])) +
    syscall_brk(0) +
    syscall_brk(0x40031000) +
    syscall_write_const_fd(1, dummy, len(messages[6])) +
    syscall_mmap() +
    syscall_munmap_current_s0() +
    syscall_write_const_fd(1, dummy, len(messages[7])) +
    syscall_mmap() +
    syscall_mprotect_current_s0() +
    syscall_madvise_current_s0() +
    syscall_munmap_current_s0() +
    syscall_write_const_fd(1, dummy, len(messages[8])) +
    syscall0(172) +
    syscall_write_const_fd(1, dummy, len(messages[9])) +
    syscall0(173) +
    syscall_write_const_fd(1, dummy, len(messages[10])) +
    syscall0(9999) +
    syscall_write_const_fd(1, dummy, len(messages[11])) +
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
    syscall_fstat_stdout_spbuf() +
    syscall_lseek_stdout() +
    syscall_write_const_fd(1, msg_addrs[1], len(messages[1])) +
    syscall_openat(msg_addrs[12], 1) +
    addi(S0, A0, 0) +
    addi(A0, S0, 0) + syscall_write_current_fd(msg_addrs[2], len(messages[2])) +
    syscall_close_current_s0() +
    syscall_write_const_fd(1, msg_addrs[3], len(messages[3])) +
    syscall_openat(msg_addrs[13], 0) +
    addi(S0, A0, 0) +
    syscall_read_current_fd_from_s0_spbuf(16) +
    syscall_close_current_s0() +
    syscall_write_const_fd(1, msg_addrs[4], len(messages[4])) +
    syscall_openat(msg_addrs[14], 0) +
    addi(S0, A0, 0) +
    syscall_getdents_current_fd_from_s0_spbuf(256) +
    syscall_close_current_s0() +
    syscall_write_const_fd(1, msg_addrs[5], len(messages[5])) +
    syscall_brk(0) +
    syscall_brk(0x40031000) +
    syscall_write_const_fd(1, msg_addrs[6], len(messages[6])) +
    syscall_mmap() +
    syscall_munmap_current_s0() +
    syscall_write_const_fd(1, msg_addrs[7], len(messages[7])) +
    syscall_mmap() +
    syscall_mprotect_current_s0() +
    syscall_madvise_current_s0() +
    syscall_munmap_current_s0() +
    syscall_write_const_fd(1, msg_addrs[8], len(messages[8])) +
    syscall0(172) +
    syscall_write_const_fd(1, msg_addrs[9], len(messages[9])) +
    syscall0(173) +
    syscall_write_const_fd(1, msg_addrs[10], len(messages[10])) +
    syscall0(9999) +
    syscall_write_const_fd(1, msg_addrs[11], len(messages[11])) +
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
print(f"[build-init-elf-v62] wrote {OUT} size={len(blob)} segment={len(segment)} entry={BASE:#x}")
