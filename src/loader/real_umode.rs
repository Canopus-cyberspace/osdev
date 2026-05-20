#[derive(Copy, Clone)]
struct RealUmodeBranchPatch {
    index: usize,
    rs1: u32,
    rs2: u32,
    funct3: u32,
}

struct RealUmodeAsm {
    words: [u32; 192],
    len: usize,
    fail_index: usize,
    patches: [RealUmodeBranchPatch; 32],
    patch_len: usize,
}

impl RealUmodeAsm {
    const fn new() -> Self {
        Self {
            words: [0; 192],
            len: 0,
            fail_index: usize::MAX,
            patches: [RealUmodeBranchPatch {
                index: 0,
                rs1: 0,
                rs2: 0,
                funct3: 0,
            }; 32],
            patch_len: 0,
        }
    }

    fn emit(&mut self, word: u32) {
        if self.len < self.words.len() {
            self.words[self.len] = word;
            self.len += 1;
        }
    }

    fn addi(&mut self, rd: u32, rs1: u32, imm: isize) {
        self.emit(real_itype(imm, rs1, 0, rd, 0x13));
    }

    fn lui(&mut self, rd: u32, imm20: usize) {
        self.emit(((imm20 as u32) << 12) | (rd << 7) | 0x37);
    }

    fn load_abs(&mut self, rd: u32, value: usize) {
        let upper = (value + 0x800) >> 12;
        let lo = value as isize - ((upper as isize) << 12);
        self.lui(rd, upper);
        self.addi(rd, rd, lo);
    }

    fn li(&mut self, rd: u32, value: isize) {
        if (-2048..=2047).contains(&value) {
            self.addi(rd, 0, value);
        } else {
            self.load_abs(rd, value as usize);
        }
    }

    fn ld(&mut self, rd: u32, imm: isize, rs1: u32) {
        self.emit(real_itype(imm, rs1, 3, rd, 0x03));
    }

    fn lbu(&mut self, rd: u32, imm: isize, rs1: u32) {
        self.emit(real_itype(imm, rs1, 4, rd, 0x03));
    }

    fn sb(&mut self, rs2: u32, imm: isize, rs1: u32) {
        self.emit(real_stype(imm, rs2, rs1, 0));
    }

    fn add(&mut self, rd: u32, rs1: u32, rs2: u32) {
        self.emit((rs2 << 20) | (rs1 << 15) | (rd << 7) | 0x33);
    }

    fn branch_fail(&mut self, rs1: u32, rs2: u32, funct3: u32) {
        if self.patch_len < self.patches.len() {
            self.patches[self.patch_len] = RealUmodeBranchPatch {
                index: self.len,
                rs1,
                rs2,
                funct3,
            };
            self.patch_len += 1;
        }
        self.emit(0);
    }

    fn beq_fail(&mut self, rs1: u32, rs2: u32) {
        self.branch_fail(rs1, rs2, 0);
    }

    fn bne_fail(&mut self, rs1: u32, rs2: u32) {
        self.branch_fail(rs1, rs2, 1);
    }

    fn ecall(&mut self) {
        self.emit(0x73);
    }

    fn label_fail(&mut self) {
        self.fail_index = self.len;
    }

    fn finish(&mut self, out: &mut [u8], off: usize) -> usize {
        let mut i = 0usize;
        while i < self.patch_len {
            let patch = self.patches[i];
            let pc = REAL_UMODE_BASE + REAL_UMODE_CODE_OFF + patch.index * 4;
            let target = REAL_UMODE_BASE + REAL_UMODE_CODE_OFF + self.fail_index * 4;
            self.words[patch.index] = real_btype(
                target as isize - pc as isize,
                patch.rs2,
                patch.rs1,
                patch.funct3,
            );
            i += 1;
        }
        i = 0;
        while i < self.len {
            write_real_u32(out, off + i * 4, self.words[i]);
            i += 1;
        }
        off + self.len * 4
    }
}

fn real_itype(imm: isize, rs1: u32, funct3: u32, rd: u32, opcode: u32) -> u32 {
    (((imm as i32 as u32) & 0xfff) << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
}

fn real_stype(imm: isize, rs2: u32, rs1: u32, funct3: u32) -> u32 {
    let value = (imm as i32 as u32) & 0xfff;
    ((value >> 5) << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | ((value & 0x1f) << 7) | 0x23
}

fn real_btype(offset: isize, rs2: u32, rs1: u32, funct3: u32) -> u32 {
    let imm = (offset as i32 as u32) & 0x1fff;
    (((imm >> 12) & 1) << 31)
        | (((imm >> 5) & 0x3f) << 25)
        | (rs2 << 20)
        | (rs1 << 15)
        | (funct3 << 12)
        | (((imm >> 1) & 0xf) << 8)
        | (((imm >> 11) & 1) << 7)
        | 0x63
}

fn write_real_u16(out: &mut [u8], off: usize, value: u16) {
    let bytes = value.to_le_bytes();
    out[off] = bytes[0];
    out[off + 1] = bytes[1];
}

fn write_real_u32(out: &mut [u8], off: usize, value: u32) {
    let bytes = value.to_le_bytes();
    out[off] = bytes[0];
    out[off + 1] = bytes[1];
    out[off + 2] = bytes[2];
    out[off + 3] = bytes[3];
}

fn write_real_u64(out: &mut [u8], off: usize, value: usize) {
    let bytes = (value as u64).to_le_bytes();
    let mut i = 0usize;
    while i < 8 {
        out[off + i] = bytes[i];
        i += 1;
    }
}

fn read_real_u16(data: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([data[off], data[off + 1]])
}

fn read_real_u32(data: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]])
}

fn read_real_u64(data: &[u8], off: usize) -> usize {
    u64::from_le_bytes([
        data[off],
        data[off + 1],
        data[off + 2],
        data[off + 3],
        data[off + 4],
        data[off + 5],
        data[off + 6],
        data[off + 7],
    ]) as usize
}

fn real_umode_make_elf(
    out: &mut [u8; REAL_UMODE_ELF_CAP],
    message: &[u8],
    exit_code: isize,
    abi_check: bool,
) -> usize {
    out.fill(0);
    let mut asm = RealUmodeAsm::new();
    const ZERO: u32 = 0;
    const SP: u32 = 2;
    const T0: u32 = 5;
    const T1: u32 = 6;
    const T2: u32 = 7;
    const A0: u32 = 10;
    const A1: u32 = 11;
    const A2: u32 = 12;
    const A7: u32 = 17;
    const T3: u32 = 28;
    const T4: u32 = 29;

    if abi_check {
        asm.ld(T0, 0, SP);
        asm.li(T1, 2);
        asm.bne_fail(T0, T1);
        asm.ld(T2, 8, SP);
        asm.lbu(T3, 0, T2);
        asm.li(T4, b'/' as isize);
        asm.bne_fail(T3, T4);
        asm.ld(T2, 24, SP);
        asm.bne_fail(T2, ZERO);
        asm.ld(T2, 32, SP);
        asm.beq_fail(T2, ZERO);
        asm.ld(T2, 48, SP);
        asm.li(T4, REAL_UMODE_AT_PAGESZ as isize);
        asm.bne_fail(T2, T4);
    }

    asm.li(A0, 1);
    asm.load_abs(A1, REAL_UMODE_BASE + REAL_UMODE_MSG_OFF);
    asm.li(A2, message.len() as isize);
    asm.li(A7, crate::syscall::SYS_WRITE as isize);
    asm.ecall();
    asm.li(A7, crate::syscall::SYS_GETPID as isize);
    asm.ecall();
    asm.beq_fail(A0, ZERO);
    asm.li(A0, exit_code);
    asm.li(A7, crate::syscall::SYS_EXIT as isize);
    asm.ecall();
    asm.label_fail();
    asm.li(A0, 90 + (exit_code & 7));
    asm.li(A7, crate::syscall::SYS_EXIT as isize);
    asm.ecall();

    out[0] = 0x7f;
    out[1] = b'E';
    out[2] = b'L';
    out[3] = b'F';
    out[4] = 2;
    out[5] = 1;
    out[6] = 1;
    write_real_u16(out, 16, 2);
    write_real_u16(out, 18, 243);
    write_real_u32(out, 20, 1);
    write_real_u64(out, 24, REAL_UMODE_BASE + REAL_UMODE_CODE_OFF);
    write_real_u64(out, 32, 64);
    write_real_u16(out, 52, 64);
    write_real_u16(out, 54, 56);
    write_real_u16(out, 56, 1);
    write_real_u32(out, 64, 1);
    write_real_u32(out, 68, 5);
    write_real_u64(out, 72, 0);
    write_real_u64(out, 80, REAL_UMODE_BASE);
    write_real_u64(out, 88, REAL_UMODE_BASE);
    let code_end = asm.finish(out, REAL_UMODE_CODE_OFF);
    let mut msg_pos = REAL_UMODE_MSG_OFF;
    let mut i = 0usize;
    while i < message.len() && msg_pos < out.len() {
        out[msg_pos] = message[i];
        msg_pos += 1;
        i += 1;
    }
    let mut file_size = if msg_pos > code_end {
        msg_pos
    } else {
        code_end
    };
    file_size = (file_size + 7) & !7usize;
    write_real_u64(out, 96, file_size);
    write_real_u64(out, 104, PAGE_SIZE);
    write_real_u64(out, 112, PAGE_SIZE);
    file_size
}

fn real_umode_write_rootfs_file(path: &[u8], data: &[u8], mode: u16) -> isize {
    let fd = crate::fs::runtime::openat(
        crate::fs::runtime::AT_FDCWD,
        path,
        crate::fs::runtime::O_CREAT | crate::fs::runtime::O_TRUNC | crate::fs::runtime::O_RDWR,
        mode,
    );
    if fd < 0 {
        return fd;
    }
    let written = crate::fs::runtime::write(fd as usize, data);
    let close_ret = crate::fs::runtime::close(fd as usize);
    if written < 0 {
        return written;
    }
    if close_ret != 0 {
        return close_ret;
    }
    written
}

fn real_umode_install_program(
    path: &[u8],
    message: &[u8],
    exit_code: isize,
    abi_check: bool,
) -> isize {
    let mut elf = [0u8; REAL_UMODE_ELF_CAP];
    let len = real_umode_make_elf(&mut elf, message, exit_code, abi_check);
    let ret = real_umode_write_rootfs_file(path, &elf[..len], 0o755);
    if ret == len as isize {
        0
    } else {
        ret
    }
}

fn real_umode_install_rootfs_programs() -> isize {
    crate::fs::runtime::reset_for_integration();
    let mkdir_ret = crate::fs::runtime::mkdirat(crate::fs::runtime::AT_FDCWD, b"/umode", 0o755);
    if mkdir_ret != 0 && mkdir_ret != crate::fs::runtime::EEXIST {
        return mkdir_ret;
    }
    let installs = [
        (
            REAL_UMODE_V191_PATH,
            b"[ucompat-v191-user] write getpid exit\n".as_slice(),
            11isize,
            false,
        ),
        (
            REAL_UMODE_V192_A_PATH,
            b"[ucompat-v192-user] program=a\n".as_slice(),
            21isize,
            false,
        ),
        (
            REAL_UMODE_V192_B_PATH,
            b"[ucompat-v192-user] program=b\n".as_slice(),
            22isize,
            false,
        ),
        (
            REAL_UMODE_V192_C_PATH,
            b"[ucompat-v192-user] program=c\n".as_slice(),
            23isize,
            false,
        ),
        (
            REAL_UMODE_V193_PATH,
            b"[ucompat-v193-user] child fork-exec body\n".as_slice(),
            33isize,
            false,
        ),
        (
            REAL_UMODE_V194_PATH,
            b"[ucompat-v194-user] abi stack verified\n".as_slice(),
            44isize,
            true,
        ),
    ];
    let mut i = 0usize;
    while i < installs.len() {
        let ret =
            real_umode_install_program(installs[i].0, installs[i].1, installs[i].2, installs[i].3);
        if ret != 0 {
            return ret;
        }
        i += 1;
    }
    let mm_installs = [
        (
            REAL_UMODE_V197_PATH,
            b"[ucompat-v197-user] lazy heap mmap stack faults\n".as_slice(),
            47isize,
            REAL_MM_PROGRAM_LAZY,
        ),
        (
            REAL_UMODE_V198_RO_PATH,
            b"[ucompat-v198-user] readonly write fault\n".as_slice(),
            48isize,
            REAL_MM_PROGRAM_RO_FAULT,
        ),
        (
            REAL_UMODE_V198_UNMAP_PATH,
            b"[ucompat-v198-user] munmap access fault\n".as_slice(),
            49isize,
            REAL_MM_PROGRAM_UNMAP_FAULT,
        ),
        (
            REAL_UMODE_V200_PATH,
            b"[ucompat-v200-user] stress brk mmap faults\n".as_slice(),
            60isize,
            REAL_MM_PROGRAM_STRESS,
        ),
    ];
    i = 0;
    while i < mm_installs.len() {
        let ret = real_mm_install_program(
            mm_installs[i].0,
            mm_installs[i].1,
            mm_installs[i].2,
            mm_installs[i].3,
        );
        if ret != 0 {
            return ret;
        }
        i += 1;
    }
    let bad_len = b"not-an-elf".len() as isize;
    let bad_ret = real_umode_write_rootfs_file(REAL_UMODE_BAD_PATH, b"not-an-elf", 0o755);
    if bad_ret == bad_len {
        0
    } else {
        bad_ret
    }
}

fn real_umode_read_rootfs_file(
    path: &[u8],
    out: &mut [u8; REAL_UMODE_ELF_CAP],
) -> Result<usize, isize> {
    let fd = crate::fs::runtime::openat(crate::fs::runtime::AT_FDCWD, path, 0, 0);
    if fd < 0 {
        return Err(fd);
    }
    let read = crate::fs::runtime::read(fd as usize, out);
    let _ = crate::fs::runtime::close(fd as usize);
    if read < 0 {
        Err(read)
    } else {
        Ok(read as usize)
    }
}

fn real_umode_stack_write_usize(sp: usize, value: usize) -> Result<(), isize> {
    let base = USER_STACK_TOP - USER_STACK_SIZE;
    if sp < base || sp + 8 > USER_STACK_TOP {
        return Err(crate::fs::runtime::EFAULT);
    }
    let off = sp - base;
    let bytes = (value as u64).to_le_bytes();
    unsafe {
        let mut i = 0usize;
        while i < 8 {
            USER_STACK.0[off + i] = bytes[i];
            i += 1;
        }
    }
    Ok(())
}

fn real_umode_stack_copy_down(sp: &mut usize, src: &[u8]) -> Result<usize, isize> {
    let base = USER_STACK_TOP - USER_STACK_SIZE;
    if src.len() + 1 > USER_STACK_SIZE || *sp < base + src.len() + 1 {
        return Err(crate::fs::runtime::EFAULT);
    }
    *sp -= src.len() + 1;
    let va = *sp;
    let off = va - base;
    unsafe {
        let mut i = 0usize;
        while i < src.len() {
            USER_STACK.0[off + i] = src[i];
            i += 1;
        }
        USER_STACK.0[off + src.len()] = 0;
    }
    Ok(va)
}

fn real_umode_build_stack(
    path: &[u8],
    argc2: bool,
    envp: bool,
    entry: usize,
) -> Result<usize, isize> {
    unsafe {
        USER_STACK.0.fill(0);
    }
    let argv1 = b"arg194";
    let env0 = b"MODE=v194";
    let argc = if argc2 { 2usize } else { 1usize };
    let envc = if envp { 1usize } else { 0usize };
    let mut sp = USER_STACK_TOP;
    let env0_ptr = if envp {
        real_umode_stack_copy_down(&mut sp, env0)?
    } else {
        0
    };
    let argv1_ptr = if argc2 {
        real_umode_stack_copy_down(&mut sp, argv1)?
    } else {
        0
    };
    let argv0_ptr = real_umode_stack_copy_down(&mut sp, path)?;
    sp &= !15usize;
    let words = 1 + argc + 1 + envc + 1 + REAL_UMODE_STACK_AUXC * 2;
    let bytes = words * core::mem::size_of::<usize>();
    if sp < (USER_STACK_TOP - USER_STACK_SIZE) + bytes {
        return Err(crate::fs::runtime::EFAULT);
    }
    sp -= bytes;
    let mut pos = sp;
    real_umode_stack_write_usize(pos, argc)?;
    pos += 8;
    real_umode_stack_write_usize(pos, argv0_ptr)?;
    pos += 8;
    if argc2 {
        real_umode_stack_write_usize(pos, argv1_ptr)?;
        pos += 8;
    }
    real_umode_stack_write_usize(pos, 0)?;
    pos += 8;
    if envp {
        real_umode_stack_write_usize(pos, env0_ptr)?;
        pos += 8;
    }
    real_umode_stack_write_usize(pos, 0)?;
    pos += 8;
    real_umode_stack_write_usize(pos, REAL_UMODE_AT_PAGESZ)?;
    pos += 8;
    real_umode_stack_write_usize(pos, PAGE_SIZE)?;
    pos += 8;
    real_umode_stack_write_usize(pos, REAL_UMODE_AT_ENTRY)?;
    pos += 8;
    real_umode_stack_write_usize(pos, entry)?;
    pos += 8;
    real_umode_stack_write_usize(pos, REAL_UMODE_AT_NULL)?;
    pos += 8;
    real_umode_stack_write_usize(pos, 0)?;
    Ok(sp)
}

fn real_umode_build_stack_from_argv(
    argv: &[&[u8]],
    envp: &[&[u8]],
    entry: usize,
) -> Result<usize, isize> {
    if argv.is_empty()
        || argv.len() > crate::fs::runtime::EXEC_ARG_MAX
        || envp.len() > crate::fs::runtime::EXEC_ENV_MAX
    {
        return Err(crate::fs::runtime::EINVAL);
    }
    unsafe {
        USER_STACK.0.fill(0);
    }
    let mut argv_ptrs = [0usize; crate::fs::runtime::EXEC_ARG_MAX];
    let mut envp_ptrs = [0usize; crate::fs::runtime::EXEC_ENV_MAX];
    let mut sp = USER_STACK_TOP;
    let mut i = envp.len();
    while i > 0 {
        i -= 1;
        envp_ptrs[i] = real_umode_stack_copy_down(&mut sp, envp[i])?;
    }
    i = argv.len();
    while i > 0 {
        i -= 1;
        argv_ptrs[i] = real_umode_stack_copy_down(&mut sp, argv[i])?;
    }
    sp &= !15usize;
    let words = 1 + argv.len() + 1 + envp.len() + 1 + REAL_UMODE_STACK_AUXC * 2;
    let bytes = words * core::mem::size_of::<usize>();
    if sp < (USER_STACK_TOP - USER_STACK_SIZE) + bytes {
        return Err(crate::fs::runtime::EFAULT);
    }
    sp -= bytes;
    let mut pos = sp;
    real_umode_stack_write_usize(pos, argv.len())?;
    pos += 8;
    i = 0;
    while i < argv.len() {
        real_umode_stack_write_usize(pos, argv_ptrs[i])?;
        pos += 8;
        i += 1;
    }
    real_umode_stack_write_usize(pos, 0)?;
    pos += 8;
    i = 0;
    while i < envp.len() {
        real_umode_stack_write_usize(pos, envp_ptrs[i])?;
        pos += 8;
        i += 1;
    }
    real_umode_stack_write_usize(pos, 0)?;
    pos += 8;
    real_umode_stack_write_usize(pos, REAL_UMODE_AT_PAGESZ)?;
    pos += 8;
    real_umode_stack_write_usize(pos, PAGE_SIZE)?;
    pos += 8;
    real_umode_stack_write_usize(pos, REAL_UMODE_AT_ENTRY)?;
    pos += 8;
    real_umode_stack_write_usize(pos, entry)?;
    pos += 8;
    real_umode_stack_write_usize(pos, REAL_UMODE_AT_NULL)?;
    pos += 8;
    real_umode_stack_write_usize(pos, 0)?;
    Ok(sp)
}

fn real_umode_parse_load(
    elf: &[u8],
    len: usize,
) -> Result<(usize, usize, usize, usize, u32), isize> {
    if len < 120
        || elf[0] != 0x7f
        || elf[1] != b'E'
        || elf[2] != b'L'
        || elf[3] != b'F'
        || elf[4] != 2
        || elf[5] != 1
    {
        return Err(crate::fs::runtime::ENOEXEC);
    }
    if read_real_u16(elf, 16) != 2 || read_real_u16(elf, 18) != 243 || read_real_u32(elf, 20) != 1 {
        return Err(crate::fs::runtime::ENOEXEC);
    }
    let entry = read_real_u64(elf, 24);
    let phoff = read_real_u64(elf, 32);
    let phentsize = read_real_u16(elf, 54) as usize;
    let phnum = read_real_u16(elf, 56) as usize;
    if entry == 0 || phentsize < 56 || phnum == 0 {
        return Err(crate::fs::runtime::ENOEXEC);
    }
    let mut i = 0usize;
    while i < phnum {
        let off = phoff + i * phentsize;
        if off + 56 > len {
            return Err(crate::fs::runtime::ENOEXEC);
        }
        if read_real_u32(elf, off) == 1 {
            let flags = read_real_u32(elf, off + 4);
            let p_offset = read_real_u64(elf, off + 8);
            let p_vaddr = read_real_u64(elf, off + 16);
            let p_filesz = read_real_u64(elf, off + 32);
            let p_memsz = read_real_u64(elf, off + 40);
            if p_vaddr != REAL_UMODE_BASE
                || p_offset + p_filesz > len
                || p_memsz > PAGE_SIZE
                || p_filesz > PAGE_SIZE
            {
                return Err(crate::fs::runtime::ENOEXEC);
            }
            if entry < p_vaddr || entry >= p_vaddr + p_memsz {
                return Err(crate::fs::runtime::ENOEXEC);
            }
            return Ok((entry, p_offset, p_filesz, p_memsz, flags));
        }
        i += 1;
    }
    Err(crate::fs::runtime::ENOEXEC)
}

fn real_umode_prepare_from_rootfs(
    path: &[u8],
    argc2: bool,
    envp: bool,
) -> Result<RealUmodeLoad, isize> {
    let argv0 = crate::fs::runtime::RuntimeExecString::from_bytes(path)?;
    let argv1 = crate::fs::runtime::RuntimeExecString::from_bytes(b"arg194")?;
    let env0 = crate::fs::runtime::RuntimeExecString::from_bytes(b"MODE=v194")?;
    let ret = if argc2 && envp {
        crate::fs::runtime::execve_from_vfs(path, &[argv0, argv1], &[env0])
    } else {
        crate::fs::runtime::execve_from_vfs(path, &[argv0], &[])
    };
    if ret != 0 {
        return Err(ret);
    }
    let mut elf = [0u8; REAL_UMODE_ELF_CAP];
    let len = real_umode_read_rootfs_file(path, &mut elf)?;
    let (entry, p_offset, p_filesz, _p_memsz, _flags) = real_umode_parse_load(&elf, len)?;
    unsafe {
        REAL_UMODE_IMAGE.0.fill(0);
        let mut i = 0usize;
        while i < p_filesz {
            REAL_UMODE_IMAGE.0[i] = elf[p_offset + i];
            i += 1;
        }
        let image_pa = core::ptr::addr_of!(REAL_UMODE_IMAGE) as usize;
        map_user_4k(REAL_UMODE_BASE, image_pa, USER_TEXT_FLAGS);
        asm!("sfence.vma zero, zero");
        real_mm_reset_allocator_state();
        real_mm_clear_lazy_user_ptes();
        USER_HEAP.0.fill(0);
        USER_MMAP_AREA.0.fill(0);
        USER_BRK = USER_HEAP_START;
        USER_MMAP_ACTIVE = false;
    }
    let sp = real_umode_build_stack(path, argc2, envp, entry)?;
    Ok(RealUmodeLoad {
        entry,
        load_start: REAL_UMODE_BASE,
        file_size: len,
        stack_pointer: sp,
    })
}

fn real_umode_enter(
    cx: &mut TrapContext,
    path: &[u8],
    argc2: bool,
    envp: bool,
) -> Result<(), isize> {
    let load = real_umode_prepare_from_rootfs(path, argc2, envp)?;
    cx.regs = [0; 32];
    cx.regs[2] = load.stack_pointer;
    cx.sstatus = user_sstatus();
    cx.sepc = load.entry;
    crate::println!(
        "[ucompat-v191] rootfs exec path={} entry={:#x} sp={:#x} load={:#x} file_size={}",
        real_umode_path_label(path),
        load.entry,
        load.stack_pointer,
        load.load_start,
        load.file_size
    );
    Ok(())
}
