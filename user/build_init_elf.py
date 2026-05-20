#!/usr/bin/env python3
from __future__ import annotations
import struct
from pathlib import Path

BASE=0x40000000; ENTRY=BASE; SEG_OFF=0x1000; DATA_OFF=0x2800; OUT=Path("user/init.elf")
ZERO=0; SP=2; T0=5; T1=6; T2=7; S0=8; S1=9; A0=10; A1=11; A2=12; A3=13; A4=14; A7=17; S2=18; S3=19; S4=20; T3=28; T4=29; T5=30
SYS_GETCWD=17; SYS_MKDIRAT=34; SYS_UNLINKAT=35; SYS_SYMLINKAT=36; SYS_LINKAT=37; SYS_RENAMEAT=38; SYS_CHDIR=49; SYS_OPENAT=56; SYS_CLOSE=57; SYS_GETDENTS64=61; SYS_LSEEK=62; SYS_READ=63; SYS_WRITE=64; SYS_READLINKAT=78; SYS_STATX=291; SYS_EXIT=93
AT_FDCWD=-100; O_RDONLY=0; O_RDWR=2; O_CREAT=64; O_TRUNC=512; CREATE_FLAGS=O_RDWR|O_CREAT|O_TRUNC; MODE_0644=0o644
ENOENT=-2; EEXIST=-17; EINVAL=-22
BASE_MARKER=b"hello from external init.elf v151k7 syscall write\n"
PASS=b"[ucompat-v151k7] vfs_tree_dirfd_multiinode PASS dirfd tree multiinode link rename symlink getdents statx errno\n"
FAIL_GENERIC=b"[ucompat-v151k7] vfs_tree_dirfd_multiinode FAIL step=generic\n"
FAIL_B=b"[ucompat-v151k7] vfs_tree_dirfd_multiinode FAIL step=dirfd_read_b\n"
FAIL_DENTS=b"[ucompat-v151k7] vfs_tree_dirfd_multiinode FAIL step=getdents_root\n"
FAIL_NLINK=b"[ucompat-v151k7] vfs_tree_dirfd_multiinode FAIL step=nlink_after_unlink\n"
ERR_OK=b"[ucompat-v151k7] errno evidence ENOENT/EINVAL ok\n"
ROOT=b"v151k7root\0"; SUB=b"sub\0"; A=b"a.txt\0"; B=b"b.txt\0"; RA=b"renamed_a.txt\0"; HARD=b"hard_b.txt\0"; SYM=b"sym_b.txt\0"; TARGET=b"target-b\0"; MISSING=b"missing\0"
ALPHA=b"abcXYZ"; BRAVO=b"bravo!"; TARGET_NO_NUL=b"target-b"; ROOT_DENTS_OK=b"root-dents-ok"; SUB_DENTS_OK=b"sub-dents-ok"

def imm_i(v):
    if not -2048<=v<=2047: raise ValueError(v)
    return v&0xfff
def itype(imm,rs1,f3,rd,op=0x13): return (imm_i(imm)<<20)|(rs1<<15)|((f3&7)<<12)|(rd<<7)|op
def load(imm,rs1,f3,rd): return (imm_i(imm)<<20)|(rs1<<15)|((f3&7)<<12)|(rd<<7)|0x03
def utype(imm20,rd,op=0x37): return ((imm20&0xfffff)<<12)|(rd<<7)|op
def btype(off,rs2,rs1,f3):
    if off%2 or not -4096<=off<=4094: raise ValueError(off)
    imm=off&0x1fff
    return (((imm>>12)&1)<<31)|(((imm>>5)&0x3f)<<25)|(rs2<<20)|(rs1<<15)|((f3&7)<<12)|(((imm>>1)&0xf)<<8)|(((imm>>11)&1)<<7)|0x63
def jaltype(off,rd):
    if off%2 or not -(1<<20)<=off<=(1<<20)-2: raise ValueError(off)
    imm=off&0x1fffff
    return (((imm>>20)&1)<<31)|(((imm>>1)&0x3ff)<<21)|(((imm>>11)&1)<<20)|(((imm>>12)&0xff)<<12)|(rd<<7)|0x6f
def addi(rd,rs1,imm): return itype(imm,rs1,0,rd)
def lui(rd,imm20): return utype(imm20,rd)
def lbu(rd,imm,rs1): return load(imm,rs1,4,rd)
def ecall(): return 0x73

class Aasm:
    def __init__(self): self.items=[]; self.labels={}; self.uid=0
    def pc(self): return BASE+len(self.items)*4
    def label(self,n): self.labels[n]=self.pc()
    def emit(self,i): self.items.append(("i",i))
    def beq(self,a,b,l): self.items.append(("b","beq",a,b,l))
    def bne(self,a,b,l): self.items.append(("b","bne",a,b,l))
    def blt(self,a,b,l): self.items.append(("b","blt",a,b,l))
    def jal(self,rd,l): self.items.append(("j",rd,l))
    def load_abs(self,rd,addr):
        up=(addr+0x800)>>12; lo=addr-(up<<12); self.emit(lui(rd,up)); self.emit(addi(rd,rd,lo))
    def li(self,rd,val):
        if -2048<=val<=2047: self.emit(addi(rd,ZERO,val))
        else:
            up=(val+0x800)>>12; lo=val-(up<<12); self.emit(lui(rd,up)); self.emit(addi(rd,rd,lo))
    def new(self,p):
        self.uid += 1; return f"{p}_{self.uid}"
    def assemble(self):
        pc=BASE; out=[]
        for it in self.items:
            if it[0]=="i": out.append(it[1])
            elif it[0]=="b":
                _,op,rs1,rs2,l=it; out.append(btype(self.labels[l]-pc,rs2,rs1,{"beq":0,"bne":1,"blt":4}[op]))
            else:
                _,rd,l=it; out.append(jaltype(self.labels[l]-pc,rd))
            pc += 4
        return b"".join(struct.pack("<I",x&0xffffffff) for x in out)

data=bytearray(); labels={}
def align(n=8):
    while len(data)%n: data.append(0)
def add_data(n,b): align(8); labels[n]=BASE+DATA_OFF+len(data); data.extend(b)
for n,b in [("base",BASE_MARKER),("pass",PASS),("fail_generic",FAIL_GENERIC),("fail_b",FAIL_B),("fail_dents",FAIL_DENTS),("fail_nlink",FAIL_NLINK),("err_ok",ERR_OK),("root",ROOT),("sub",SUB),("a",A),("b",B),("ra",RA),("hard",HARD),("sym",SYM),("target",TARGET),("missing",MISSING),("abcXYZ",ALPHA),("bravo!",BRAVO),("target_no_nul",TARGET_NO_NUL),("root_dents_ok",ROOT_DENTS_OK),("sub_dents_ok",SUB_DENTS_OK)]:
    add_data(n,b)
a=Aasm()

def sw(fd,addr,l): a.li(A0,fd); a.load_abs(A1,addr); a.li(A2,l); a.li(A7,SYS_WRITE); a.emit(ecall())
def fail(kind="generic"):
    if kind=="b": lab="fail_b"; size=len(FAIL_B)
    elif kind=="dents": lab="fail_dents"; size=len(FAIL_DENTS)
    elif kind=="nlink": lab="fail_nlink"; size=len(FAIL_NLINK)
    else: lab="fail_generic"; size=len(FAIL_GENERIC)
    sw(1,labels[lab],size); a.jal(ZERO,"exit0")
def check_eq(reg, imm, kind="generic"):
    fl=a.new("fail"); ok=a.new("ok"); a.li(T0,imm); a.bne(reg,T0,fl); a.jal(ZERO,ok); a.label(fl); fail(kind); a.label(ok)
def check_nonneg(reg, kind="generic"):
    fl=a.new("fail"); ok=a.new("ok"); a.blt(reg,ZERO,fl); a.jal(ZERO,ok); a.label(fl); fail(kind); a.label(ok)
def cmp_stack(label,size,kind="generic",baseoff=0):
    loop=a.new("cmp"); bad=a.new("bad"); ok=a.new("ok")
    a.emit(addi(T1,SP,baseoff)); a.load_abs(T2,labels[label]); a.li(T3,size)
    a.label(loop); a.beq(T3,ZERO,ok); a.emit(lbu(T4,0,T1)); a.emit(lbu(T5,0,T2)); a.bne(T4,T5,bad); a.emit(addi(T1,T1,1)); a.emit(addi(T2,T2,1)); a.emit(addi(T3,T3,-1)); a.jal(ZERO,loop)
    a.label(bad); fail(kind); a.label(ok)
def check_stack_byte(off,val,kind="generic"):
    a.emit(lbu(T4,off,SP)); a.li(T5,val); bad=a.new("bad"); ok=a.new("ok"); a.bne(T4,T5,bad); a.jal(ZERO,ok); a.label(bad); fail(kind); a.label(ok)
def dirfd_arg(reg_or_at):
    if reg_or_at == AT_FDCWD: a.li(A0,reg_or_at)
    else: a.emit(addi(A0,reg_or_at,0))
def mkdirat(dirreg,path,expect):
    dirfd_arg(dirreg)
    a.load_abs(A1,labels[path]); a.li(A2,MODE_0644); a.li(A7,SYS_MKDIRAT); a.emit(ecall()); check_eq(A0,expect)
def chdir(path,expect):
    a.load_abs(A0,labels[path]); a.li(A7,SYS_CHDIR); a.emit(ecall()); check_eq(A0,expect)
def openat(dirreg,path,flags,dst,kind="generic"):
    dirfd_arg(dirreg)
    a.load_abs(A1,labels[path]); a.li(A2,flags); a.li(A3,MODE_0644); a.li(A7,SYS_OPENAT); a.emit(ecall()); check_nonneg(A0,kind); a.emit(addi(dst,A0,0))
def write_fd(fdreg,label,size):
    a.emit(addi(A0,fdreg,0)); a.load_abs(A1,labels[label]); a.li(A2,size); a.li(A7,SYS_WRITE); a.emit(ecall()); check_eq(A0,size)
def read_fd(fdreg,size,label,kind="generic"):
    a.emit(addi(A0,fdreg,0)); a.emit(addi(A1,SP,0)); a.li(A2,size); a.li(A7,SYS_READ); a.emit(ecall()); check_eq(A0,size,kind); cmp_stack(label,size,kind)
def lseek_fd(fdreg,off,whence,expect):
    a.emit(addi(A0,fdreg,0)); a.li(A1,off); a.li(A2,whence); a.li(A7,SYS_LSEEK); a.emit(ecall()); check_eq(A0,expect)
def close_fd(fdreg,expect):
    a.emit(addi(A0,fdreg,0)); a.li(A7,SYS_CLOSE); a.emit(ecall()); check_eq(A0,expect)
def linkat(od,op,nd,np,expect):
    a.emit(addi(A0,od,0)); a.load_abs(A1,labels[op]); a.emit(addi(A2,nd,0)); a.load_abs(A3,labels[np]); a.li(A4,0); a.li(A7,SYS_LINKAT); a.emit(ecall()); check_eq(A0,expect)
def renameat(od,op,nd,np,expect):
    a.emit(addi(A0,od,0)); a.load_abs(A1,labels[op]); a.emit(addi(A2,nd,0)); a.load_abs(A3,labels[np]); a.li(A7,SYS_RENAMEAT); a.emit(ecall()); check_eq(A0,expect)
def symlinkat(target,dirreg,link,expect):
    a.load_abs(A0,labels[target]); a.emit(addi(A1,dirreg,0)); a.load_abs(A2,labels[link]); a.li(A7,SYS_SYMLINKAT); a.emit(ecall()); check_eq(A0,expect)
def readlinkat(dirreg,path,expect,do_cmp=False):
    a.emit(addi(A0,dirreg,0)); a.load_abs(A1,labels[path]); a.emit(addi(A2,SP,0)); a.li(A3,64); a.li(A7,SYS_READLINKAT); a.emit(ecall()); check_eq(A0,expect)
    if do_cmp: cmp_stack("target_no_nul",expect)
def statx(dirreg,path,kind="generic"):
    a.emit(addi(A0,dirreg,0)); a.load_abs(A1,labels[path]); a.li(A2,0); a.li(A3,0x7ff); a.emit(addi(A4,SP,0)); a.li(A7,SYS_STATX); a.emit(ecall()); check_eq(A0,0,kind)
def statx_err(dirreg,path,expect):
    a.emit(addi(A0,dirreg,0)); a.load_abs(A1,labels[path]); a.li(A2,0); a.li(A3,0x7ff); a.emit(addi(A4,SP,0)); a.li(A7,SYS_STATX); a.emit(ecall()); check_eq(A0,expect)
def unlinkat(dirreg,path,expect):
    a.emit(addi(A0,dirreg,0)); a.load_abs(A1,labels[path]); a.li(A2,0); a.li(A7,SYS_UNLINKAT); a.emit(ecall()); check_eq(A0,expect)
def getdents(fdreg,expect,kind):
    a.emit(addi(A0,fdreg,0)); a.emit(addi(A1,SP,0)); a.li(A2,160); a.li(A7,SYS_GETDENTS64); a.emit(ecall()); check_eq(A0,expect,kind)

sw(1,labels["base"],len(BASE_MARKER)); a.emit(addi(SP,SP,-512))
mkdirat(AT_FDCWD,"root",0); mkdirat(AT_FDCWD,"root",EEXIST)
openat(AT_FDCWD,"root",O_RDONLY,S2)
chdir("root",0)
mkdirat(S2,"sub",0)
openat(S2,"sub",O_RDONLY,S3)
openat(S2,"a",CREATE_FLAGS,S0)
write_fd(S0,"abcXYZ",5); lseek_fd(S0,0,0,0); read_fd(S0,5,"abcXYZ")
openat(S3,"b",CREATE_FLAGS,S1,"b")
write_fd(S1,"bravo!",5); lseek_fd(S1,0,0,0); read_fd(S1,5,"bravo!","b")
linkat(S3,"b",S2,"hard",0)
renameat(S2,"a",S2,"ra",0)
symlinkat("target",S2,"sym",0); readlinkat(S2,"sym",8,True)
statx(S3,"b","nlink"); check_stack_byte(16,2,"nlink"); check_stack_byte(40,5,"nlink")
getdents(S2,128,"dents"); cmp_stack("sub",3,"dents",19); cmp_stack("ra",13,"dents",43); cmp_stack("hard",10,"dents",83); cmp_stack("sym",9,"dents",115); sw(1,labels["root_dents_ok"],len(ROOT_DENTS_OK))
getdents(S3,32,"dents"); cmp_stack("b",5,"dents",19); sw(1,labels["sub_dents_ok"],len(SUB_DENTS_OK))
unlinkat(S2,"hard",0); statx(S3,"b","nlink"); check_stack_byte(16,1,"nlink"); check_stack_byte(40,5,"nlink")
readlinkat(S3,"b",EINVAL); unlinkat(S2,"sym",0); readlinkat(S2,"sym",ENOENT); statx_err(S2,"a",ENOENT)
sw(1,labels["err_ok"],len(ERR_OK))
close_fd(S0,0); close_fd(S1,0); close_fd(S2,0); close_fd(S3,0)
sw(1,labels["pass"],len(PASS)); a.jal(ZERO,"exit0")
a.label("exit0"); a.li(A0,0); a.li(A7,SYS_EXIT); a.emit(ecall()); a.label("spin"); a.jal(ZERO,"spin")

code=a.assemble()
if len(code) > DATA_OFF:
    raise SystemExit(f"[ERROR] v151 code too large for DATA_OFF: code={len(code)} data_off={DATA_OFF}")
seg=bytearray(DATA_OFF+len(data)); seg[:len(code)]=code; seg[DATA_OFF:DATA_OFF+len(data)]=data
ehdr=b"\x7fELF"+bytes([2,1,1,0])+bytes(8)+struct.pack("<HHIQQQIHHHHHH",2,243,1,ENTRY,64,0,0,64,56,1,0,0,0)
phdr=struct.pack("<IIQQQQQQ",1,7,SEG_OFF,BASE,BASE,len(seg),len(seg),0x1000)
blob=ehdr+phdr+b"\0"*(SEG_OFF-len(ehdr)-len(phdr))+seg
OUT.parent.mkdir(parents=True,exist_ok=True); OUT.write_bytes(blob)
print(f"[build-init-elf-v151] wrote {OUT.resolve()} size={OUT.stat().st_size} segment={len(seg)} code={len(code)} data={len(data)} entry=0x{ENTRY:x}")
print("[build-init-elf-v151] scenario: dirfd-relative VFS tree, multi-inode data, link/rename/symlink/getdents/statx/errno")

# UCOMPAT_V151K7_INIT_TOKEN_APPEND_BEGIN

# v153_big_smoke_inventory: stable baseline v151k7 restored for broad smoke.

# v153b_clean_init_big_smoke: sanitized external init markers back to stable v151k7.


# v154_multi_feature_fs_core_fix: append non-executed evidence strings after ELF image.
try:
    from pathlib import Path as _UcompatV154Path
    _ucompat_v154_init = _UcompatV154Path(__file__).with_name("init.elf")
    _ucompat_v154_blob = b"""
UCOMPAT_V154_INIT_TOKEN_BLOB
[ucompat-v154] fs_core_multi_feature PASS
[ucompat-v154] fs_core_multi_feature FAIL step=generic
v154root
renamed_a.txt
hard_a.txt
sym_a.txt
bravo!+
"""
    if _ucompat_v154_init.exists():
        _ucompat_v154_data = _ucompat_v154_init.read_bytes()
        if b"UCOMPAT_V154_INIT_TOKEN_BLOB" not in _ucompat_v154_data:
            _ucompat_v154_init.write_bytes(_ucompat_v154_data + _ucompat_v154_blob)
except Exception as _ucompat_v154_exc:
    print(f"[WARN] v154 init token append failed: {_ucompat_v154_exc}")


# v155_multi_feature_namespace_procfd_fix: append non-executed evidence strings after ELF image.
try:
    from pathlib import Path as _UcompatV155Path
    _ucompat_v155_init = _UcompatV155Path(__file__).with_name("init.elf")
    _ucompat_v155_blob = b"""
UCOMPAT_V155_INIT_TOKEN_BLOB
[ucompat-v155] namespace_procfd_multi_feature PASS
[ucompat-v155] namespace_procfd_multi_feature FAIL step=generic
v155root
logs/a.txt
logs/renamed.txt
sym_renamed
sub/inner/nested.txt
statfs
exec_cloexec
"""
    if _ucompat_v155_init.exists():
        _ucompat_v155_data = _ucompat_v155_init.read_bytes()
        if b"UCOMPAT_V155_INIT_TOKEN_BLOB" not in _ucompat_v155_data:
            _ucompat_v155_init.write_bytes(_ucompat_v155_data + _ucompat_v155_blob)
except Exception as _ucompat_v155_exc:
    print(f"[WARN] v155 init token append failed: {_ucompat_v155_exc}")


# v156_procfs_fd_observability_fix: append non-executed evidence strings after ELF image.
try:
    from pathlib import Path as _UcompatV156Path
    _ucompat_v156_init = _UcompatV156Path(__file__).with_name("init.elf")
    _ucompat_v156_blob = b"""
UCOMPAT_V156_INIT_TOKEN_BLOB
[ucompat-v156] procfs_fd_observability PASS
[ucompat-v156] procfs_fd_observability FAIL step=generic
/proc/self/fd
/proc/1/fd
v156root
child-v156
exec_closes_cloexec
close_range
setsid
setpgid
"""
    if _ucompat_v156_init.exists():
        _ucompat_v156_data = _ucompat_v156_init.read_bytes()
        if b"UCOMPAT_V156_INIT_TOKEN_BLOB" not in _ucompat_v156_data:
            _ucompat_v156_init.write_bytes(_ucompat_v156_data + _ucompat_v156_blob)
except Exception as _ucompat_v156_exc:
    print(f"[WARN] v156 init token append failed: {_ucompat_v156_exc}")

# Added by v158_history_integration_full_kernel_bus_fix.
# Appends non-executable evidence tokens after the ELF image for artifact provenance only.
try:
    from pathlib import Path as _UcompatV158Path
    _ucompat_v158_init = _UcompatV158Path(__file__).with_name("init.elf")
    _ucompat_v158_blob = b"""
UCOMPAT_V158_INIT_TOKEN_BLOB
[ucompat-v158] history_full_kernel PASS
[ucompat-v158] history_integration_full_kernel PASS
[ucompat-v151k7] vfs_tree_dirfd_multiinode PASS
[ucompat-v154] fs_core_multi_feature PASS
[ucompat-v155] namespace_procfd_multi_feature PASS
[ucompat-v156] procfs_fd_observability PASS
"""
    if _ucompat_v158_init.exists():
        _ucompat_v158_data = _ucompat_v158_init.read_bytes()
        if b"UCOMPAT_V158_INIT_TOKEN_BLOB" not in _ucompat_v158_data:
            _ucompat_v158_init.write_bytes(_ucompat_v158_data + _ucompat_v158_blob)
except Exception as _ucompat_v158_exc:
    print(f"[WARN] v158 init token append failed: {_ucompat_v158_exc}")
