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
    return addi(A0, S0, 0) + li(A1, 4096) + li(A2, 1) + li(A7, 226) + ecall()

def syscall_madvise_current_s0():
    return addi(A0, S0, 0) + li(A1, 4096) + li(A2, 0) + li(A7, 233) + ecall()

def syscall_munmap_current_s0():
    return addi(A0, S0, 0) + li(A1, 4096) + li(A7, 215) + ecall()

def syscall_uname_spbuf():
    return addi(A0, 2, -768) + li(A7, 160) + ecall()

def syscall_clock_gettime_spbuf():
    return li(A0, 0) + addi(A1, 2, -896) + li(A7, 113) + ecall()

def syscall_gettimeofday_spbuf():
    return addi(A0, 2, -928) + li(A1, 0) + li(A7, 169) + ecall()

def syscall_set_tid_address_spword():
    return addi(A0, 2, -960) + li(A7, 96) + ecall()

def syscall_set_robust_list_spword():
    return addi(A0, 2, -976) + li(A1, 24) + li(A7, 99) + ecall()

def syscall_sysinfo_spbuf():
    return addi(A0, 2, -1152) + li(A7, 179) + ecall()

def syscall_prlimit64_spbuf():
    return li(A0, 0) + li(A1, 3) + li(A2, 0) + addi(A3, 2, -1184) + li(A7, 261) + ecall()

def syscall_getrandom_spbuf():
    return addi(A0, 2, -1216) + li(A1, 16) + li(A2, 0) + li(A7, 278) + ecall()

def syscall_getcwd_spbuf():
    return addi(A0, 2, -1344) + li(A1, 128) + li(A7, 17) + ecall()

def syscall_fcntl_stdout_getfl():
    # fcntl(1, F_GETFL=3, 0)
    return li(A0, 1) + li(A1, 3) + li(A2, 0) + li(A7, 25) + ecall()

def syscall_ioctl_stdout_winsz():
    # ioctl(1, TIOCGWINSZ=0x5413, sp-1376)
    return li(A0, 1) + li(A1, 0x5413) + addi(A2, 2, -1376) + li(A7, 29) + ecall()

def syscall_readlinkat(path_addr):
    # readlinkat(AT_FDCWD=-100, path, sp-1440, 64)
    return li(A0, -100) + load_abs(A1, path_addr) + addi(A2, 2, -1440) + li(A3, 64) + li(A7, 78) + ecall()

def syscall_umask():
    return li(A0, 0o22) + li(A7, 166) + ecall()

def syscall_chdir(path_addr):
    return load_abs(A0, path_addr) + li(A7, 49) + ecall()

def syscall0(num):
    return li(A7, num) + ecall()

def syscall1(num, arg0):
    return li(A0, arg0) + li(A7, num) + ecall()

def syscall_openat(path_addr, flags):
    return li(A0, -100) + load_abs(A1, path_addr) + li(A2, flags) + li(A3, 0) + li(A7, 56) + ecall()

def syscall_close_current_s0():
    return addi(A0, S0, 0) + li(A7, 57) + ecall()

messages = [
    b"hello from external init.elf v65 syscall write\n",
    b"external init fstat lseek passed\n",
    b"this write goes to devnull v65\n",
    b"external init openat close passed\n",
    b"external init devzero read passed\n",
    b"external init getdents64 dev passed\n",
    b"external init brk passed\n",
    b"external init mmap munmap passed\n",
    b"external init mprotect madvise passed\n",
    b"external init uname time passed\n",
    b"external init proc resource random passed\n",
    b"external init path tty fcntl passed\n",
    b"external init getpid returned 1\n",
    b"external init getppid returned 0\n",
    b"external init unsupported returned -38\n",
    b"/dev/null\x00",
    b"/dev/zero\x00",
    b"/dev\x00",
    b"/proc/self/exe\x00",
    b"/\x00",
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
    syscall_uname_spbuf() +
    syscall_clock_gettime_spbuf() +
    syscall_gettimeofday_spbuf() +
    syscall_write_const_fd(1, dummy, len(messages[9])) +
    syscall_set_tid_address_spword() +
    syscall_set_robust_list_spword() +
    syscall0(174) + syscall0(175) + syscall0(176) + syscall0(177) + syscall0(178) +
    syscall_sysinfo_spbuf() +
    syscall_prlimit64_spbuf() +
    syscall_getrandom_spbuf() +
    syscall_write_const_fd(1, dummy, len(messages[10])) +
    syscall_getcwd_spbuf() +
    syscall_fcntl_stdout_getfl() +
    syscall_ioctl_stdout_winsz() +
    syscall_readlinkat(dummy) +
    syscall_umask() +
    syscall_chdir(dummy) +
    syscall_write_const_fd(1, dummy, len(messages[11])) +
    syscall0(172) +
    syscall_write_const_fd(1, dummy, len(messages[12])) +
    syscall0(173) +
    syscall_write_const_fd(1, dummy, len(messages[13])) +
    syscall0(9999) +
    syscall_write_const_fd(1, dummy, len(messages[14])) +
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
    syscall_openat(msg_addrs[15], 1) +
    addi(S0, A0, 0) +
    addi(A0, S0, 0) + syscall_write_current_fd(msg_addrs[2], len(messages[2])) +
    syscall_close_current_s0() +
    syscall_write_const_fd(1, msg_addrs[3], len(messages[3])) +
    syscall_openat(msg_addrs[16], 0) +
    addi(S0, A0, 0) +
    syscall_read_current_fd_from_s0_spbuf(16) +
    syscall_close_current_s0() +
    syscall_write_const_fd(1, msg_addrs[4], len(messages[4])) +
    syscall_openat(msg_addrs[17], 0) +
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
    syscall_uname_spbuf() +
    syscall_clock_gettime_spbuf() +
    syscall_gettimeofday_spbuf() +
    syscall_write_const_fd(1, msg_addrs[9], len(messages[9])) +
    syscall_set_tid_address_spword() +
    syscall_set_robust_list_spword() +
    syscall0(174) + syscall0(175) + syscall0(176) + syscall0(177) + syscall0(178) +
    syscall_sysinfo_spbuf() +
    syscall_prlimit64_spbuf() +
    syscall_getrandom_spbuf() +
    syscall_write_const_fd(1, msg_addrs[10], len(messages[10])) +
    syscall_getcwd_spbuf() +
    syscall_fcntl_stdout_getfl() +
    syscall_ioctl_stdout_winsz() +
    syscall_readlinkat(msg_addrs[18]) +
    syscall_umask() +
    syscall_chdir(msg_addrs[19]) +
    syscall_write_const_fd(1, msg_addrs[11], len(messages[11])) +
    syscall0(172) +
    syscall_write_const_fd(1, msg_addrs[12], len(messages[12])) +
    syscall0(173) +
    syscall_write_const_fd(1, msg_addrs[13], len(messages[13])) +
    syscall0(9999) +
    syscall_write_const_fd(1, msg_addrs[14], len(messages[14])) +
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
print(f"[build-init-elf-v65] wrote {OUT} size={len(blob)} segment={len(segment)} entry={BASE:#x}")
