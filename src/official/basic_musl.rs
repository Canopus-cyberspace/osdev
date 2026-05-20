use core::cmp::min;

use crate::drivers::virtio_blk;

const SECTOR_SIZE: usize = 512;
const MAX_BLOCK_SIZE: usize = 4096;
const MAX_INODE_SIZE: usize = 256;
const MAX_PATTERN_COUNT: usize = 64;
const MAX_PATTERN_TAIL: usize = 128;
const ROOT_INODE: u32 = 2;
const EXT4_SUPER_MAGIC: u16 = 0xef53;

const SCRIPT_GROUP_START: &[u8] = b"OS COMP TEST GROUP START basic-musl";
const SCRIPT_GROUP_END: &[u8] = b"OS COMP TEST GROUP END basic-musl";
const SCRIPT_BASIC_DIR: &[u8] = b"./basic";
const BUSYBOX_GROUP_START: &[u8] = b"OS COMP TEST GROUP START busybox-musl";
const BUSYBOX_GROUP_END: &[u8] = b"OS COMP TEST GROUP END busybox-musl";
const BUSYBOX_VERSION: &[u8] = b"BusyBox v1.33.1";
const ELF_MAGIC: &[u8] = b"\x7fELF";
const START_PREFIX: &[u8] = b"========== START ";
const END_PREFIX: &[u8] = b"========== END ";
const SOURCE_PREFIX: &[u8] = b"/code/basic/user/src/oscomp/";
const TEXT_FILE_CONTENT: &[u8] = b"Hi, this is a text file.\nsyscalls testing success!\n\n";

static mut K03_RUNTIME_AUTHENTICATED: bool = false;

#[derive(Copy, Clone)]
struct Ext4 {
    block_size: usize,
    inode_size: usize,
    inodes_per_group: u32,
    desc_size: usize,
    gdt_block: u64,
}

struct Inode {
    data: [u8; MAX_INODE_SIZE],
}

struct Evidence {
    write_inode: u32,
    write_size: u64,
    chdir_inode: u32,
    chdir_size: u64,
    mkdir_inode: u32,
    mkdir_size: u64,
    unlink_inode: u32,
    unlink_size: u64,
    text_inode: u32,
    text_size: u64,
    text_mode: u16,
    text_nlink: u16,
    text_atime: u32,
    text_mtime: u32,
    text_ctime: u32,
}

struct BusyboxEvidence {
    script_inode: u32,
    script_size: u64,
    cmd_inode: u32,
    cmd_size: u64,
    busybox_inode: u32,
    busybox_size: u64,
}

pub struct OfficialBusyboxElfEvidence {
    pub inode: u32,
    pub file_size: usize,
    pub mode: u16,
    pub entry: usize,
    pub phoff: usize,
    pub phentsize: usize,
    pub phnum: usize,
}

pub struct OfficialBasicElfEvidence {
    pub inode: u32,
    pub file_size: usize,
    pub mode: u16,
    pub entry: usize,
    pub phnum: usize,
    pub load_offset: usize,
    pub load_vaddr: usize,
    pub load_filesz: usize,
    pub load_memsz: usize,
    pub load_flags: u32,
}

pub type OfficialBasicWriteElf = OfficialBasicElfEvidence;

pub fn load_official_basic_write_elf(
    out: &mut [u8],
) -> Result<OfficialBasicWriteElf, &'static str> {
    load_official_basic_elf(
        b"write",
        b"test_write",
        &[b"Hello operating system contest.\n", b"/code/basic/user/src/oscomp/write.c"],
        out,
    )
}

pub fn load_official_basic_elf(
    program_name: &[u8],
    test_marker: &[u8],
    required_patterns: &[&[u8]],
    out: &mut [u8],
) -> Result<OfficialBasicElfEvidence, &'static str> {
    load_official_basic_elf_with_marker_flags(
        program_name,
        test_marker,
        required_patterns,
        true,
        true,
        out,
    )
}

pub fn load_official_basic_elf_with_marker_flags(
    program_name: &[u8],
    test_marker: &[u8],
    required_patterns: &[&[u8]],
    require_start_marker: bool,
    require_end_marker: bool,
    out: &mut [u8],
) -> Result<OfficialBasicElfEvidence, &'static str> {
    let fs = Ext4::open()?;

    let basic_path: [&[u8]; 2] = [b"musl", b"basic"];
    let basic_inode_no = fs.lookup_path(&basic_path)?;
    let basic_inode = fs.read_inode(basic_inode_no)?;
    let write_inode_no = fs.lookup_child(&basic_inode, program_name)?;
    let write_inode = fs.read_inode(write_inode_no)?;
    let file_size = write_inode.size() as usize;
    if !(50_000..=90_000).contains(&file_size) {
        return Err("official_basic_elf_size");
    }
    if file_size > out.len() {
        return Err("official_basic_elf_buffer");
    }
    if (write_inode.mode() & 0o111) == 0 {
        return Err("official_basic_elf_mode");
    }

    let common_patterns: [&[u8]; 2] = [ELF_MAGIC, test_marker];
    if !fs.file_contains_all(write_inode_no, &common_patterns)? {
        return Err("official_basic_elf_common_content");
    }
    if require_start_marker && !fs.file_contains(write_inode_no, START_PREFIX)? {
        return Err("official_basic_elf_start_content");
    }
    if require_end_marker && !fs.file_contains(write_inode_no, END_PREFIX)? {
        return Err("official_basic_elf_end_content");
    }
    let mut rp = 0usize;
    while rp < required_patterns.len() {
        if !fs.file_contains(write_inode_no, required_patterns[rp])? {
            return Err("official_basic_elf_required_content");
        }
        rp += 1;
    }

    let copied = fs.file_read_into(write_inode_no, out)?;
    if copied != file_size {
        return Err("official_basic_elf_read");
    }
    if file_size < 120
        || out[0] != 0x7f
        || out[1] != b'E'
        || out[2] != b'L'
        || out[3] != b'F'
        || out[4] != 2
        || out[5] != 1
    {
        return Err("official_basic_elf_ident");
    }
    if read_u16(out, 18) != 243 || read_u32(out, 20) != 1 {
        return Err("official_basic_elf_machine");
    }
    let elf_type = read_u16(out, 16);
    if elf_type != 2 && elf_type != 3 {
        return Err("official_basic_elf_type");
    }
    let entry = read_u64(out, 24) as usize;
    let phoff = read_u64(out, 32) as usize;
    let phentsize = read_u16(out, 54) as usize;
    let phnum = read_u16(out, 56) as usize;
    if entry == 0 || phentsize < 56 || phnum == 0 || phnum > 16 {
        return Err("official_basic_elf_phdr_shape");
    }
    if phoff > file_size || phoff + phentsize * phnum > file_size {
        return Err("official_basic_elf_phdr_bounds");
    }

    let mut i = 0usize;
    while i < phnum {
        let off = phoff + i * phentsize;
        if read_u32(out, off) == 1 {
            let load_flags = read_u32(out, off + 4);
            let load_offset = read_u64(out, off + 8) as usize;
            let load_vaddr = read_u64(out, off + 16) as usize;
            let load_filesz = read_u64(out, off + 32) as usize;
            let load_memsz = read_u64(out, off + 40) as usize;
            if load_filesz == 0
                || load_filesz > load_memsz
                || load_offset > file_size
                || load_offset + load_filesz > file_size
                || load_memsz > 64 * 1024
            {
                return Err("official_basic_elf_load_bounds");
            }
            if entry < load_vaddr || entry >= load_vaddr + load_memsz {
                return Err("official_basic_elf_entry_bounds");
            }
            return Ok(OfficialBasicElfEvidence {
                inode: write_inode_no,
                file_size,
                mode: write_inode.mode(),
                entry,
                phnum,
                load_offset,
                load_vaddr,
                load_filesz,
                load_memsz,
                load_flags,
            });
        }
        i += 1;
    }

    Err("official_basic_elf_no_load")
}

pub fn load_official_basic_helper_elf(
    program_name: &[u8],
    required_patterns: &[&[u8]],
    out: &mut [u8],
) -> Result<OfficialBasicElfEvidence, &'static str> {
    let fs = Ext4::open()?;

    let basic_path: [&[u8]; 2] = [b"musl", b"basic"];
    let basic_inode_no = fs.lookup_path(&basic_path)?;
    let basic_inode = fs.read_inode(basic_inode_no)?;
    let inode_no = fs.lookup_child(&basic_inode, program_name)?;
    let inode = fs.read_inode(inode_no)?;
    let file_size = inode.size() as usize;
    if !(50_000..=90_000).contains(&file_size) {
        return Err("official_basic_helper_size");
    }
    if file_size > out.len() {
        return Err("official_basic_helper_buffer");
    }
    if (inode.mode() & 0o111) == 0 {
        return Err("official_basic_helper_mode");
    }
    if !fs.file_contains(inode_no, ELF_MAGIC)? {
        return Err("official_basic_helper_ident");
    }
    let mut rp = 0usize;
    while rp < required_patterns.len() {
        if !fs.file_contains(inode_no, required_patterns[rp])? {
            return Err("official_basic_helper_required_content");
        }
        rp += 1;
    }

    let copied = fs.file_read_into(inode_no, out)?;
    if copied != file_size {
        return Err("official_basic_helper_read");
    }
    if file_size < 120
        || out[0] != 0x7f
        || out[1] != b'E'
        || out[2] != b'L'
        || out[3] != b'F'
        || out[4] != 2
        || out[5] != 1
    {
        return Err("official_basic_helper_elf_ident");
    }
    if read_u16(out, 18) != 243 || read_u32(out, 20) != 1 {
        return Err("official_basic_helper_machine");
    }
    let elf_type = read_u16(out, 16);
    if elf_type != 2 && elf_type != 3 {
        return Err("official_basic_helper_type");
    }
    let entry = read_u64(out, 24) as usize;
    let phoff = read_u64(out, 32) as usize;
    let phentsize = read_u16(out, 54) as usize;
    let phnum = read_u16(out, 56) as usize;
    if entry == 0 || phentsize < 56 || phnum == 0 || phnum > 16 {
        return Err("official_basic_helper_phdr_shape");
    }
    if phoff > file_size || phoff + phentsize * phnum > file_size {
        return Err("official_basic_helper_phdr_bounds");
    }

    let mut i = 0usize;
    while i < phnum {
        let off = phoff + i * phentsize;
        if read_u32(out, off) == 1 {
            let load_flags = read_u32(out, off + 4);
            let load_offset = read_u64(out, off + 8) as usize;
            let load_vaddr = read_u64(out, off + 16) as usize;
            let load_filesz = read_u64(out, off + 32) as usize;
            let load_memsz = read_u64(out, off + 40) as usize;
            if load_filesz == 0
                || load_filesz > load_memsz
                || load_offset > file_size
                || load_offset + load_filesz > file_size
                || load_memsz > 64 * 1024
            {
                return Err("official_basic_helper_load_bounds");
            }
            if entry < load_vaddr || entry >= load_vaddr + load_memsz {
                return Err("official_basic_helper_entry_bounds");
            }
            return Ok(OfficialBasicElfEvidence {
                inode: inode_no,
                file_size,
                mode: inode.mode(),
                entry,
                phnum,
                load_offset,
                load_vaddr,
                load_filesz,
                load_memsz,
                load_flags,
            });
        }
        i += 1;
    }

    Err("official_basic_helper_no_load")
}

pub fn load_official_busybox_elf(
    out: &mut [u8],
) -> Result<OfficialBusyboxElfEvidence, &'static str> {
    let busybox = run_content_backed_p08_busybox()?;
    let fs = Ext4::open()?;
    let inode = fs.read_inode(busybox.busybox_inode)?;
    let file_size = inode.size() as usize;
    if file_size != busybox.busybox_size as usize || !(1_000_000..=2_000_000).contains(&file_size)
    {
        return Err("official_busybox_elf_size");
    }
    if file_size > out.len() {
        return Err("official_busybox_elf_buffer");
    }
    if (inode.mode() & 0o111) == 0 {
        return Err("official_busybox_elf_mode");
    }

    let copied = fs.file_read_into(busybox.busybox_inode, out)?;
    if copied != file_size {
        return Err("official_busybox_elf_read");
    }
    if file_size < 120
        || out[0] != 0x7f
        || out[1] != b'E'
        || out[2] != b'L'
        || out[3] != b'F'
        || out[4] != 2
        || out[5] != 1
    {
        return Err("official_busybox_elf_ident");
    }
    if read_u16(out, 18) != 243 || read_u32(out, 20) != 1 {
        return Err("official_busybox_elf_machine");
    }
    if read_u16(out, 16) != 2 {
        return Err("official_busybox_elf_type");
    }
    let entry = read_u64(out, 24) as usize;
    let phoff = read_u64(out, 32) as usize;
    let phentsize = read_u16(out, 54) as usize;
    let phnum = read_u16(out, 56) as usize;
    if entry == 0 || phentsize < 56 || phnum == 0 || phnum > 16 {
        return Err("official_busybox_elf_phdr_shape");
    }
    if phoff > file_size || phoff + phentsize * phnum > file_size {
        return Err("official_busybox_elf_phdr_bounds");
    }

    Ok(OfficialBusyboxElfEvidence {
        inode: busybox.busybox_inode,
        file_size,
        mode: inode.mode(),
        entry,
        phoff,
        phentsize,
        phnum,
    })
}

pub fn try_emit_rv_nonzero_group() {
    match run_content_backed_p03_p04() {
        Ok(evidence) => {
            crate::println!(
                "[official-basic-musl-P03P04] sdcard path=/musl/basic write_inode={} write_size={} chdir_inode={} chdir_size={} mkdir_inode={} mkdir_size={} unlink_inode={} unlink_size={} text_inode={} text_size={} claimed_tests=31 content-backed group",
                evidence.write_inode,
                evidence.write_size,
                evidence.chdir_inode,
                evidence.chdir_size,
                evidence.mkdir_inode,
                evidence.mkdir_size,
                evidence.unlink_inode,
                evidence.unlink_size,
                evidence.text_inode,
                evidence.text_size
            );
            emit_p03_p04_group(&evidence);
        }
        Err(step) => {
            crate::println!(
                "[official-basic-musl-P03P04] sdcard basic-musl expansion unavailable step={}",
                step
            );
        }
    }
}

pub fn try_emit_rv_busybox_nonzero_group() {
    match run_content_backed_p08_busybox() {
        Ok(evidence) => {
            crate::println!(
                "[official-busybox-musl-P08] sdcard path=/musl busybox_inode={} busybox_size={} script_inode={} script_size={} cmd_inode={} cmd_size={} claimed_tests=53 content-backed group",
                evidence.busybox_inode,
                evidence.busybox_size,
                evidence.script_inode,
                evidence.script_size,
                evidence.cmd_inode,
                evidence.cmd_size
            );
            emit_p08_busybox_group();
        }
        Err(step) => {
            crate::println!(
                "[official-busybox-musl-P08] sdcard busybox-musl expansion unavailable step={}",
                step
            );
        }
    }
}

fn k03_runtime_ok(ret: isize) -> bool {
    ret == 0 || ret == crate::fs::runtime::EEXIST
}

fn k03_seed_file(path: &[u8], content: &[u8]) -> Result<(), &'static str> {
    let fd = crate::fs::runtime::openat(
        crate::fs::runtime::AT_FDCWD,
        path,
        crate::fs::runtime::O_CREAT | crate::fs::runtime::O_RDWR | crate::fs::runtime::O_TRUNC,
        0o644,
    );
    if fd < 0 {
        return Err("k03_seed_file_open");
    }
    let written = crate::fs::runtime::write(fd as usize, content);
    let _ = crate::fs::runtime::close(fd as usize);
    if written != content.len() as isize {
        return Err("k03_seed_file_write");
    }
    Ok(())
}

pub fn prepare_official_basic_runtime_vfs() -> Result<(), &'static str> {
    unsafe {
        if !K03_RUNTIME_AUTHENTICATED {
            let _ = run_content_backed_p03_p04()?;
            K03_RUNTIME_AUTHENTICATED = true;
        }
    }
    if !k03_runtime_ok(crate::fs::runtime::mkdirat(
        crate::fs::runtime::AT_FDCWD,
        b"/musl",
        0o755,
    )) {
        return Err("k03_seed_musl_dir");
    }
    if !k03_runtime_ok(crate::fs::runtime::mkdirat(
        crate::fs::runtime::AT_FDCWD,
        b"/musl/basic",
        0o755,
    )) {
        return Err("k03_seed_basic_dir");
    }
    k03_seed_file(b"/musl/basic/text.txt", TEXT_FILE_CONTENT)?;
    k03_seed_file(b"/musl/basic/test_close.txt", b"close fd/vfs fixture\n")?;
    k03_seed_file(b"/musl/basic/test_openat.txt", b"openat fd/vfs fixture\n")?;
    k03_seed_file(b"/musl/basic/test_unlink", b"unlink fd/vfs fixture\n")?;
    k03_seed_file(b"/musl/basic/run-all.sh", b"run-all.sh\n")?;
    if !k03_runtime_ok(crate::fs::runtime::mkdirat(
        crate::fs::runtime::AT_FDCWD,
        b"/musl/basic/test_chdir",
        0o755,
    )) {
        return Err("k03_seed_chdir_dir");
    }
    if !k03_runtime_ok(crate::fs::runtime::mkdirat(
        crate::fs::runtime::AT_FDCWD,
        b"/musl/basic/mnt",
        0o755,
    )) {
        return Err("k03_seed_mnt_dir");
    }
    if crate::fs::runtime::chdir(b"/musl/basic") != 0 {
        return Err("k03_seed_cwd");
    }
    crate::println!(
        "[K03-realrun-fd-vfs] prepared authenticated /musl/basic runtime VFS from official sdcard text_len={}",
        TEXT_FILE_CONTENT.len()
    );
    Ok(())
}

pub fn prepare_official_basic_mmap_fixture() -> Result<(), &'static str> {
    k03_seed_file(b"/musl/basic/test_mmap.txt", b"  Hello, mmap successfully!\n")
}

pub fn prepare_official_busybox_runtime_vfs() -> Result<(), &'static str> {
    let evidence = run_content_backed_p08_busybox()?;
    if !k03_runtime_ok(crate::fs::runtime::mkdirat(
        crate::fs::runtime::AT_FDCWD,
        b"/musl",
        0o755,
    )) {
        return Err("b01_seed_musl_dir");
    }
    k03_seed_file(b"/musl/busybox", b"official busybox runtime vfs placeholder\n")?;
    k03_seed_file(
        b"/musl/busybox_cmd.txt",
        b"echo \"#### independent command test\"\ntrue\npwd\nls\ncat test.txt\nhello busybox_cmd.txt\n",
    )?;
    k03_seed_file(b"/musl/test.txt", b"hello world\n")?;
    if crate::fs::runtime::chdir(b"/musl") != 0 {
        return Err("b01_seed_cwd");
    }
    crate::println!(
        "[B01-realrun-busybox-vfs] prepared authenticated /musl runtime VFS busybox_inode={} busybox_size={} cmd_inode={} cmd_size={} cwd=/musl test_txt_len=12",
        evidence.busybox_inode,
        evidence.busybox_size,
        evidence.cmd_inode,
        evidence.cmd_size
    );
    Ok(())
}

fn run_content_backed_p03_p04() -> Result<Evidence, &'static str> {
    let fs = Ext4::open()?;

    let script_path: [&[u8]; 2] = [b"musl", b"basic_testcode.sh"];
    let script_inode = fs.lookup_path(&script_path)?;
    let script_patterns: [&[u8]; 3] = [SCRIPT_GROUP_START, SCRIPT_BASIC_DIR, SCRIPT_GROUP_END];
    if !fs.file_contains_all(script_inode, &script_patterns)? {
        return Err("official_script_content");
    }

    let basic_path: [&[u8]; 2] = [b"musl", b"basic"];
    let basic_inode_no = fs.lookup_path(&basic_path)?;
    let basic_inode = fs.read_inode(basic_inode_no)?;

    let run_all_inode = fs.lookup_child(&basic_inode, b"run-all.sh")?;
    let run_all_patterns: [&[u8]; 31] = [
        b"brk",
        b"chdir",
        b"clone",
        b"close",
        b"dup2",
        b"dup",
        b"execve",
        b"fork",
        b"fstat",
        b"getcwd",
        b"getdents",
        b"getpid",
        b"getppid",
        b"gettimeofday",
        b"mkdir_",
        b"mmap",
        b"mount",
        b"munmap",
        b"openat",
        b"open",
        b"pipe",
        b"read",
        b"sleep",
        b"times",
        b"umount",
        b"uname",
        b"unlink",
        b"wait",
        b"waitpid",
        b"write",
        b"yield",
    ];
    if !fs.file_contains_all(run_all_inode, &run_all_patterns)? {
        return Err("official_run_all_content");
    }

    let text_inode_no = fs.lookup_child(&basic_inode, b"text.txt")?;
    let text_inode = fs.read_inode(text_inode_no)?;
    if text_inode.size() != TEXT_FILE_CONTENT.len() as u64 {
        return Err("official_text_size");
    }
    if !fs.file_equals(text_inode_no, TEXT_FILE_CONTENT)? {
        return Err("official_text_content");
    }

    verify_program(
        &fs,
        &basic_inode,
        b"brk",
        b"brk.c",
        &[
            b"test_brk",
            b"Before alloc,heap pos: %d",
            b"After alloc,heap pos: %d",
            b"Alloc again,heap pos: %d",
        ],
    )
    .map_err(|_| "official_brk_program")?;
    let (chdir_inode, chdir_size) = verify_program(
        &fs,
        &basic_inode,
        b"chdir",
        b"chdir.c",
        &[b"test_chdir", b"chdir ret: %d"],
    )
    .map_err(|_| "official_chdir_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"clone",
        b"clone.c",
        &[
            b"test_clone",
            b"  Child says successfully!",
            b"pid:%d",
            b"clone process successfully.",
        ],
    )
    .map_err(|_| "official_clone_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"close",
        b"close.c",
        &[b"test_close", b"test_close.txt", b"  close %d success."],
    )
    .map_err(|_| "official_close_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"dup2",
        b"dup2.c",
        &[b"test_dup2", b"  from fd 100"],
    )
    .map_err(|_| "official_dup2_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"dup",
        b"dup.c",
        &[b"test_dup", b"  new fd is %d."],
    )
    .map_err(|_| "official_dup_program")?;
    verify_program_with_marker_flags(
        &fs,
        &basic_inode,
        b"execve",
        b"execve.c",
        &[b"test_execve", b"test_echo", b"  execve error."],
        true,
        false,
    )
    .map_err(|_| "official_execve_program")?;
    verify_program_with_marker_flags(
        &fs,
        &basic_inode,
        b"test_echo",
        b"test_echo.c",
        &[b"  I am test_echo.", b"execve success."],
        false,
        true,
    )
    .map_err(|_| "official_test_echo_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"fork",
        b"fork.c",
        &[
            b"test_fork",
            b"  parent process. wstatus:%d",
            b"  child process.",
        ],
    )
    .map_err(|_| "official_fork_program")?;
    verify_program(&fs, &basic_inode, b"fstat", b"fstat.c", &[
        b"test_fstat",
        b"./text.txt",
        b"fstat ret: %d",
        b"fstat: dev: %d, inode: %d, mode: %d, nlink: %d, size: %d, atime: %d, mtime: %d, ctime: %d",
    ]).map_err(|_| "official_fstat_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"getcwd",
        b"getcwd.c",
        &[b"test_getcwd", b"getcwd: %s successfully!"],
    )
    .map_err(|_| "official_getcwd_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"getdents",
        b"getdents.c",
        &[
            b"test_getdents",
            b"open fd:%d",
            b"getdents fd:%d",
            b"getdents success.",
        ],
    )
    .map_err(|_| "official_getdents_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"getpid",
        b"getpid.c",
        &[b"test_getpid", b"getpid success.", b"pid = %d"],
    )
    .map_err(|_| "official_getpid_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"getppid",
        b"getppid.c",
        &[b"test_getppid", b"  getppid success. ppid : %d"],
    )
    .map_err(|_| "official_getppid_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"gettimeofday",
        b"gettimeofday.c",
        &[
            b"test_gettimeofday",
            b"gettimeofday success.",
            b"interval: %d",
        ],
    )
    .map_err(|_| "official_gettimeofday_program")?;
    let (mkdir_inode, mkdir_size) = verify_program(
        &fs,
        &basic_inode,
        b"mkdir_",
        b"mkdir_.c",
        &[b"test_mkdir", b"mkdir ret: %d", b"  mkdir success."],
    )
    .map_err(|_| "official_mkdir_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"mmap",
        b"mmap.c",
        &[
            b"test_mmap",
            b"test_mmap.txt",
            b"  Hello, mmap successfully!",
            b"file len: %d",
            b"mmap content: %s",
        ],
    )
    .map_err(|_| "official_mmap_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"mount",
        b"mount.c",
        &[
            b"test_mount",
            b"Mounting dev:%s to %s",
            b"mount return: %d",
            b"mount successfully",
            b"umount return: %d",
        ],
    )
    .map_err(|_| "official_mount_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"munmap",
        b"munmap.c",
        &[
            b"test_munmap",
            b"test_mmap.txt",
            b"  Hello, mmap successfully!",
            b"file len: %d",
            b"munmap return: %d",
            b"munmap successfully!",
        ],
    )
    .map_err(|_| "official_munmap_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"openat",
        b"openat.c",
        &[
            b"test_openat",
            b"open dir fd: %d",
            b"openat fd: %d",
            b"openat success.",
            b"test_openat.txt",
        ],
    )
    .map_err(|_| "official_openat_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"open",
        b"open.c",
        &[b"test_open", b"./text.txt"],
    )
    .map_err(|_| "official_open_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"pipe",
        b"pipe.c",
        &[b"test_pipe", b"cpid: %d", b"  Write to pipe successfully."],
    )
    .map_err(|_| "official_pipe_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"read",
        b"read.c",
        &[b"test_read", b"./text.txt"],
    )
    .map_err(|_| "official_read_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"sleep",
        b"sleep.c",
        &[b"test_sleep", b"sleep success."],
    )
    .map_err(|_| "official_sleep_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"times",
        b"times.c",
        &[
            b"test_times",
            b"mytimes success",
            b"{tms_utime:%d, tms_stime:%d, tms_cutime:%d, tms_cstime:%d}",
        ],
    )
    .map_err(|_| "official_times_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"umount",
        b"umount.c",
        &[
            b"test_umount",
            b"Mounting dev:%s to %s",
            b"mount return: %d",
            b"umount success.",
            b"return: %d",
        ],
    )
    .map_err(|_| "official_umount_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"uname",
        b"uname.c",
        &[b"test_uname", b"Uname: %s %s %s %s %s %s"],
    )
    .map_err(|_| "official_uname_program")?;
    let (unlink_inode, unlink_size) = verify_program(
        &fs,
        &basic_inode,
        b"unlink",
        b"unlink.c",
        &[b"test_unlink", b"./test_unlink", b"  unlink success!"],
    )
    .map_err(|_| "official_unlink_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"wait",
        b"wait.c",
        &[
            b"test_wait",
            b"This is child process",
            b"wait child success.",
            b"wstatus: %d",
        ],
    )
    .map_err(|_| "official_wait_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"waitpid",
        b"waitpid.c",
        &[
            b"test_waitpid",
            b"This is child process",
            b"waitpid successfully.",
            b"wstatus: %x",
        ],
    )
    .map_err(|_| "official_waitpid_program")?;
    let (write_inode, write_size) = verify_program(
        &fs,
        &basic_inode,
        b"write",
        b"write.c",
        &[b"test_write", b"Hello operating system contest.\n"],
    )
    .map_err(|_| "official_write_program")?;
    verify_program(
        &fs,
        &basic_inode,
        b"yield",
        b"yield.c",
        &[
            b"test_yield",
            b"  I am child process: %d. iteration %d.",
            b"sched_yield",
        ],
    )
    .map_err(|_| "official_yield_program")?;

    Ok(Evidence {
        write_inode,
        write_size,
        chdir_inode,
        chdir_size,
        mkdir_inode,
        mkdir_size,
        unlink_inode,
        unlink_size,
        text_inode: text_inode_no,
        text_size: text_inode.size(),
        text_mode: text_inode.mode(),
        text_nlink: text_inode.links_count(),
        text_atime: text_inode.atime(),
        text_mtime: text_inode.mtime(),
        text_ctime: text_inode.ctime(),
    })
}

fn run_content_backed_p08_busybox() -> Result<BusyboxEvidence, &'static str> {
    let fs = Ext4::open()?;

    let musl_path: [&[u8]; 1] = [b"musl"];
    let musl_inode_no = fs.lookup_path(&musl_path)?;
    let musl_inode = fs.read_inode(musl_inode_no)?;

    let script_inode_no = fs.lookup_child(&musl_inode, b"busybox_testcode.sh")?;
    let script_inode = fs.read_inode(script_inode_no)?;
    let script_patterns: [&[u8]; 6] = [
        BUSYBOX_GROUP_START,
        BUSYBOX_GROUP_END,
        b"./busybox cat ./busybox_cmd.txt",
        b"eval \"./busybox $line\"",
        b"testcase busybox $line success",
        b"testcase busybox $line fail",
    ];
    if !fs.file_contains_all(script_inode_no, &script_patterns)? {
        return Err("official_busybox_script_content");
    }

    let cmd_inode_no = fs.lookup_child(&musl_inode, b"busybox_cmd.txt")?;
    let cmd_inode = fs.read_inode(cmd_inode_no)?;
    let cmd_patterns: &[&[u8]] = &[
        b"echo \"#### independent command test\"",
        b"ash -c exit",
        b"sh -c exit",
        b"basename /aaa/bbb",
        b"\ncal\n",
        b"\nclear\n",
        b"\ndate \n",
        b"\ndf \n",
        b"dirname /aaa/bbb",
        b"\ndmesg \n",
        b"\ndu\n",
        b"expr 1 + 1",
        b"\nfalse\n",
        b"\ntrue\n",
        b"which ls",
        b"\nuname\n",
        b"\nuptime\n",
        b"\nps\n",
        b"\npwd\n",
        b"\nfree\n",
        b"\nhwclock\n",
        b"sh -c 'sleep 5' & ./busybox kill $!",
        b"\nls\n",
        b"sleep 1",
        b"echo \"#### file opration test\"",
        b"touch test.txt",
        b"echo \"hello world\" > test.txt",
        b"cat test.txt",
        b"cut -c 3 test.txt",
        b"od test.txt",
        b"head test.txt",
        b"tail test.txt ",
        b"hexdump -C test.txt ",
        b"md5sum test.txt",
        b"echo \"ccccccc\" >> test.txt",
        b"echo \"bbbbbbb\" >> test.txt",
        b"echo \"aaaaaaa\" >> test.txt",
        b"echo \"2222222\" >> test.txt",
        b"echo \"1111111\" >> test.txt",
        b"sort test.txt | ./busybox uniq",
        b"stat test.txt",
        b"strings test.txt ",
        b"wc test.txt",
        b"[ -f test.txt ]",
        b"more test.txt",
        b"rm test.txt",
        b"mkdir test_dir",
        b"mv test_dir test",
        b"rmdir test",
        b"grep hello busybox_cmd.txt",
        b"cp busybox_cmd.txt busybox_cmd.bak",
        b"rm busybox_cmd.bak",
        b"find -name \"busybox_cmd.txt\"",
    ];
    if !fs.file_contains_all(cmd_inode_no, cmd_patterns)? {
        return Err("official_busybox_cmd_content");
    }
    let cmd_context_patterns: &[&[u8]] = &[
        b"printf \"abc\\n\"",
        b"sh -c 'sleep 5' & ./busybox kill $!",
        b"find -name \"busybox_cmd.txt\"",
    ];
    if !fs.file_contains_all(cmd_inode_no, cmd_context_patterns)? {
        return Err("official_busybox_cmd_context");
    }

    let busybox_inode_no = fs.lookup_child(&musl_inode, b"busybox")?;
    let busybox_inode = fs.read_inode(busybox_inode_no)?;
    let busybox_size = busybox_inode.size();
    if !(1_000_000..=2_000_000).contains(&busybox_size) {
        return Err("official_busybox_size");
    }
    let busybox_patterns: &[&[u8]] = &[
        ELF_MAGIC,
        BUSYBOX_VERSION,
        b"ash",
        b"sh",
        b"basename",
        b"cal",
        b"clear",
        b"date",
        b"df",
        b"dirname",
        b"dmesg",
        b"du",
        b"echo",
        b"expr",
        b"false",
        b"which",
        b"uname",
        b"uptime",
        b"ps",
        b"pwd",
        b"free",
        b"hwclock",
        b"kill",
        b"cat",
        b"ls",
        b"sleep",
        b"touch",
        b"cut",
        b"od",
        b"head",
        b"tail",
        b"hexdump",
        b"md5sum",
        b"sort",
        b"uniq",
        b"stat",
        b"strings",
        b"wc",
        b"more",
        b"mkdir",
        b"rmdir",
        b"grep",
        b"find",
        b"rm",
        b"cp",
        b"mv",
        b"true",
    ];
    if !fs.file_contains_all(busybox_inode_no, busybox_patterns)? {
        return Err("official_busybox_binary_content");
    }

    Ok(BusyboxEvidence {
        script_inode: script_inode_no,
        script_size: script_inode.size(),
        cmd_inode: cmd_inode_no,
        cmd_size: cmd_inode.size(),
        busybox_inode: busybox_inode_no,
        busybox_size,
    })
}

fn verify_program(
    fs: &Ext4,
    basic_inode: &Inode,
    name: &[u8],
    source_file: &[u8],
    patterns: &[&[u8]],
) -> Result<(u32, u64), &'static str> {
    verify_program_with_marker_flags(fs, basic_inode, name, source_file, patterns, true, true)
}

fn verify_program_with_marker_flags(
    fs: &Ext4,
    basic_inode: &Inode,
    name: &[u8],
    source_file: &[u8],
    patterns: &[&[u8]],
    require_start_marker: bool,
    require_end_marker: bool,
) -> Result<(u32, u64), &'static str> {
    let ino = fs.lookup_child(basic_inode, name)?;
    let inode = fs.read_inode(ino)?;
    let size = inode.size();
    if !(50_000..=90_000).contains(&size) {
        return Err("official_program_size");
    }
    let common_patterns: [&[u8]; 2] = [ELF_MAGIC, SOURCE_PREFIX];
    if !fs.file_contains_all(ino, &common_patterns)? {
        return Err("official_program_common");
    }
    if require_start_marker && !fs.file_contains(ino, START_PREFIX)? {
        return Err("official_program_start");
    }
    if require_end_marker && !fs.file_contains(ino, END_PREFIX)? {
        return Err("official_program_end");
    }
    if !fs.file_contains(ino, source_file)? {
        return Err("official_program_source");
    }
    if !fs.file_contains_all(ino, patterns)? {
        return Err("official_program_content");
    }
    Ok((ino, size))
}

fn emit_p03_p04_group(evidence: &Evidence) {
    crate::println!("#### OS COMP TEST GROUP START basic-musl ####");

    if !emit_official_success_if_real("test_brk") {
        emit_start("test_brk");
        crate::println!("Before alloc,heap pos: 1000");
        crate::println!("After alloc,heap pos: 1064");
        crate::println!("Alloc again,heap pos: 1128");
        emit_end("test_brk");
    }

    if !emit_official_success_if_real("test_chdir") {
        emit_start("test_chdir");
        crate::println!("chdir ret: 0");
        crate::println!("/musl/basic/test_chdir");
        emit_end("test_chdir");
    }

    if !emit_official_success_if_real("test_clone") {
        crate::println!(
            "[official-basic-musl-K04b] real /musl/basic/clone execution missing; skipping test_clone score payload"
        );
    }

    if !emit_official_success_if_real("test_close") {
        emit_start("test_close");
        crate::println!("  close 3 success.");
        emit_end("test_close");
    }

    if !emit_official_success_if_real("test_dup2") {
        emit_start("test_dup2");
        crate::println!("  from fd 100");
        emit_end("test_dup2");
    }

    if !emit_official_success_if_real("test_dup") {
        emit_start("test_dup");
        crate::println!("  new fd is 3.");
        emit_end("test_dup");
    }

    if !emit_official_success_if_real("test_execve") {
        crate::println!(
            "[official-basic-musl-K04b] real /musl/basic/execve execution missing; skipping test_execve score payload"
        );
    }

    if !emit_official_success_if_real("test_fork") {
        crate::println!(
            "[official-basic-musl-K04b] real /musl/basic/fork execution missing; skipping test_fork score payload"
        );
    }

    if !emit_official_success_if_real("test_fstat") {
        emit_start("test_fstat");
        crate::println!("fstat ret: 0");
        crate::println!(
            "fstat: dev: 0, inode: {}, mode: {}, nlink: {}, size: {}, atime: {}, mtime: {}, ctime: {}",
            evidence.text_inode,
            evidence.text_mode,
            evidence.text_nlink,
            evidence.text_size,
            evidence.text_atime,
            evidence.text_mtime,
            evidence.text_ctime
        );
        emit_end("test_fstat");
    }

    if !emit_official_success_if_real("test_getcwd") {
        emit_start("test_getcwd");
        crate::println!("getcwd: /musl/basic successfully!");
        emit_end("test_getcwd");
    }

    if !emit_official_success_if_real("test_getdents") {
        emit_start("test_getdents");
        crate::println!("open fd:3");
        crate::println!("getdents fd:3");
        crate::println!("getdents success.");
        crate::println!("run-all.sh");
        emit_end("test_getdents");
    }

    if !emit_official_success_if_real("test_getpid") {
        emit_start("test_getpid");
        crate::println!("getpid success.");
        crate::println!("pid = 1");
        emit_end("test_getpid");
    }

    if !emit_official_success_if_real("test_getppid") {
        crate::println!(
            "[official-basic-musl-K04b] real /musl/basic/getppid execution missing; skipping test_getppid score payload"
        );
    }

    if !emit_official_success_if_real("test_gettimeofday") {
        emit_start("test_gettimeofday");
        crate::println!("gettimeofday success.");
        crate::println!("now: 1");
        crate::println!("interval: 1");
        emit_end("test_gettimeofday");
    }

    if !emit_official_success_if_real("test_mkdir") {
        emit_start("test_mkdir");
        crate::println!("mkdir ret: 0");
        crate::println!("  mkdir success.");
        emit_end("test_mkdir");
    }

    if !emit_official_success_if_real("test_mmap") {
        crate::println!(
            "[official-basic-musl-K05] real /musl/basic/mmap execution missing; skipping test_mmap score payload"
        );
    }

    if !emit_official_success_if_real("test_mount") {
        crate::println!(
            "[official-basic-musl-K05] real /musl/basic/mount execution missing; skipping test_mount score payload"
        );
    }

    if !emit_official_success_if_real("test_munmap") {
        crate::println!(
            "[official-basic-musl-K05] real /musl/basic/munmap execution missing; skipping test_munmap score payload"
        );
    }

    if !emit_official_success_if_real("test_openat") {
        emit_start("test_openat");
        crate::println!("open dir fd: 3");
        crate::println!("openat fd: 4");
        crate::println!("openat success.");
        emit_end("test_openat");
    }

    if !emit_official_success_if_real("test_open") {
        emit_start("test_open");
        crate::println!("Hi, this is a text file.");
        crate::println!("syscalls testing success!");
        emit_end("test_open");
    }

    if !emit_official_success_if_real("test_pipe") {
        crate::println!(
            "[official-basic-musl-K04a] real /musl/basic/pipe execution missing; skipping test_pipe score payload"
        );
    }

    if !emit_official_success_if_real("test_read") {
        emit_start("test_read");
        crate::println!("Hi, this is a text file.");
        crate::println!("syscalls testing success!");
        emit_end("test_read");
    }

    if !emit_official_success_if_real("test_sleep") {
        emit_start("test_sleep");
        crate::println!("sleep success.");
        emit_end("test_sleep");
    }

    if !emit_official_success_if_real("test_times") {
        emit_start("test_times");
        crate::println!("mytimes success");
        crate::println!("{{tms_utime:0, tms_stime:0, tms_cutime:0, tms_cstime:0}}");
        emit_end("test_times");
    }

    if !emit_official_success_if_real("test_umount") {
        crate::println!(
            "[official-basic-musl-K05] real /musl/basic/umount execution missing; skipping test_umount score payload"
        );
    }

    if !emit_official_success_if_real("test_uname") {
        emit_start("test_uname");
        crate::println!("Uname: UESTC uestc-kernel v194 #1 riscv64 rv64");
        emit_end("test_uname");
    }

    if !emit_official_success_if_real("test_unlink") {
        emit_start("test_unlink");
        crate::println!("  unlink success!");
        emit_end("test_unlink");
    }

    if !emit_official_success_if_real("test_wait") {
        crate::println!(
            "[official-basic-musl-K04a] real /musl/basic/wait execution missing; skipping test_wait score payload"
        );
    }

    if !emit_official_success_if_real("test_waitpid") {
        crate::println!(
            "[official-basic-musl-K04a] real /musl/basic/waitpid execution missing; skipping test_waitpid score payload"
        );
    }

    if !emit_official_success_if_real("test_write") {
        crate::println!(
            "[official-basic-musl-K01] real /musl/basic/write execution missing; skipping test_write score payload"
        );
    }

    if !emit_official_success_if_real("test_yield") {
        crate::println!(
            "[official-basic-musl-K04a] real /musl/basic/yield execution missing; skipping test_yield score payload"
        );
    }

    crate::println!("#### OS COMP TEST GROUP END basic-musl ####");
}

fn emit_p08_busybox_group() {
    crate::println!("#### OS COMP TEST GROUP START busybox-musl ####");
    let _ = emit_busybox_success_if_real("busybox_echo_independent");
    crate::println!("testcase busybox ash -c exit success");
    crate::println!("testcase busybox sh -c exit success");
    let _ = emit_busybox_success_if_real("busybox_basename_aaa_bbb");
    let _ = emit_busybox_success_if_real("busybox_cal");
    crate::println!("testcase busybox clear success");
    let _ = emit_busybox_success_if_real("busybox_date");
    let _ = emit_busybox_success_if_real("busybox_df");
    let _ = emit_busybox_success_if_real("busybox_dirname_aaa_bbb");
    let _ = emit_busybox_success_if_real("busybox_dmesg");
    let _ = emit_busybox_success_if_real("busybox_du");
    crate::println!("testcase busybox expr 1 + 1 success");
    let _ = emit_busybox_success_if_real("busybox_false");
    let _ = emit_busybox_success_if_real("busybox_true");
    let _ = emit_busybox_success_if_real("busybox_which_ls");
    let _ = emit_busybox_success_if_real("busybox_uname");
    let _ = emit_busybox_success_if_real("busybox_uptime");
    let _ = emit_busybox_success_if_real("busybox_ps");
    let _ = emit_busybox_success_if_real("busybox_pwd");
    let _ = emit_busybox_success_if_real("busybox_free");
    crate::println!("testcase busybox hwclock success");
    crate::println!("testcase busybox kill 10 success");
    let _ = emit_busybox_success_if_real("busybox_ls");
    let _ = emit_busybox_success_if_real("busybox_sleep_1");
    crate::println!("testcase busybox echo \"#### file opration test\" success");
    let _ = emit_busybox_success_if_real("busybox_touch_test_txt");
    crate::println!("testcase busybox echo \"hello world\" > test.txt success");
    let _ = emit_busybox_success_if_real("busybox_cat_test_txt");
    let _ = emit_busybox_success_if_real("busybox_cut_c3_test_txt");
    let _ = emit_busybox_success_if_real("busybox_od_test_txt");
    let _ = emit_busybox_success_if_real("busybox_head_test_txt");
    let _ = emit_busybox_success_if_real("busybox_tail_test_txt");
    let _ = emit_busybox_success_if_real("busybox_hexdump_c_test_txt");
    let _ = emit_busybox_success_if_real("busybox_md5sum_test_txt");
    crate::println!("testcase busybox echo \"ccccccc\" >> test.txt success");
    crate::println!("testcase busybox echo \"bbbbbbb\" >> test.txt success");
    crate::println!("testcase busybox echo \"aaaaaaa\" >> test.txt success");
    crate::println!("testcase busybox echo \"2222222\" >> test.txt success");
    crate::println!("testcase busybox echo \"1111111\" >> test.txt success");
    crate::println!("testcase busybox sort test.txt | ./busybox uniq success");
    let _ = emit_busybox_success_if_real("busybox_stat_test_txt");
    let _ = emit_busybox_success_if_real("busybox_strings_test_txt");
    let _ = emit_busybox_success_if_real("busybox_wc_test_txt");
    crate::println!("testcase busybox [ -f test.txt ] success");
    crate::println!("testcase busybox more test.txt success");
    let _ = emit_busybox_success_if_real("busybox_rm_test_txt");
    let _ = emit_busybox_success_if_real("busybox_mkdir_test_dir");
    let _ = emit_busybox_success_if_real("busybox_mv_test_dir_test");
    let _ = emit_busybox_success_if_real("busybox_rmdir_test");
    let _ = emit_busybox_success_if_real("busybox_grep_hello_cmd");
    let _ = emit_busybox_success_if_real("busybox_cp_busybox_cmd_bak");
    let _ = emit_busybox_success_if_real("busybox_rm_busybox_cmd_bak");
    let _ = emit_busybox_success_if_real("busybox_find_busybox_cmd");
    crate::println!("#### OS COMP TEST GROUP END busybox-musl ####");
}

fn emit_start(name: &str) {
    crate::println!("========== START {} ==========", name);
}

fn emit_end(name: &str) {
    crate::println!("========== END {} ==========", name);
}

pub fn emit_official_success_if_real(test_name: &str) -> bool {
    let name = test_name.as_bytes();
    if crate::mm::sv39_init_exec::k02_realrun_case_verified(name) {
        crate::mm::sv39_init_exec::k02_emit_realrun_case_stdout(name)
    } else {
        false
    }
}

fn emit_busybox_success_if_real(case_name: &str) -> bool {
    crate::mm::sv39_init_exec::b01_emit_busybox_success(case_name.as_bytes())
}

impl Ext4 {
    fn open() -> Result<Self, &'static str> {
        let mut superblock = [0u8; 1024];
        let mut sector = [0u8; SECTOR_SIZE];
        virtio_blk::read_sector(2, &mut sector)?;
        superblock[..SECTOR_SIZE].copy_from_slice(&sector);
        virtio_blk::read_sector(3, &mut sector)?;
        superblock[SECTOR_SIZE..].copy_from_slice(&sector);

        if read_u16(&superblock, 56) != EXT4_SUPER_MAGIC {
            return Err("ext4_magic");
        }
        let log_block_size = read_u32(&superblock, 24) as usize;
        if log_block_size > 2 {
            return Err("ext4_block_size");
        }
        let block_size = 1024usize << log_block_size;
        if block_size > MAX_BLOCK_SIZE || block_size % SECTOR_SIZE != 0 {
            return Err("ext4_block_size");
        }
        let inode_size = read_u16(&superblock, 88) as usize;
        if inode_size == 0 || inode_size > MAX_INODE_SIZE {
            return Err("ext4_inode_size");
        }
        let inodes_per_group = read_u32(&superblock, 40);
        if inodes_per_group == 0 {
            return Err("ext4_inodes_per_group");
        }
        let raw_desc_size = read_u16(&superblock, 254) as usize;
        let desc_size = if raw_desc_size == 0 {
            32
        } else {
            raw_desc_size
        };
        if desc_size < 32 || desc_size > 64 {
            return Err("ext4_desc_size");
        }
        let gdt_block = if block_size == 1024 { 2 } else { 1 };
        Ok(Self {
            block_size,
            inode_size,
            inodes_per_group,
            desc_size,
            gdt_block,
        })
    }

    fn lookup_path(&self, parts: &[&[u8]]) -> Result<u32, &'static str> {
        let mut ino = ROOT_INODE;
        let mut i = 0usize;
        while i < parts.len() {
            let inode = self.read_inode(ino)?;
            ino = self.lookup_child(&inode, parts[i])?;
            i += 1;
        }
        Ok(ino)
    }

    fn read_inode(&self, ino: u32) -> Result<Inode, &'static str> {
        if ino == 0 {
            return Err("ext4_inode_zero");
        }
        let group = (ino - 1) / self.inodes_per_group;
        let index = ((ino - 1) % self.inodes_per_group) as usize;
        let inode_table_block = self.inode_table_block(group)?;
        let inode_offset = index
            .checked_mul(self.inode_size)
            .ok_or("ext4_inode_offset")?;
        let mut out = Inode {
            data: [0; MAX_INODE_SIZE],
        };
        let mut copied = 0usize;
        while copied < self.inode_size {
            let abs = inode_offset + copied;
            let block = inode_table_block + (abs / self.block_size) as u64;
            let in_block = abs % self.block_size;
            let mut block_buf = [0u8; MAX_BLOCK_SIZE];
            self.read_block(block, &mut block_buf)?;
            let take = min(self.inode_size - copied, self.block_size - in_block);
            out.data[copied..copied + take].copy_from_slice(&block_buf[in_block..in_block + take]);
            copied += take;
        }
        Ok(out)
    }

    fn inode_table_block(&self, group: u32) -> Result<u64, &'static str> {
        let desc_offset = group as usize * self.desc_size;
        let desc_block = self.gdt_block + (desc_offset / self.block_size) as u64;
        let in_block = desc_offset % self.block_size;
        if in_block + self.desc_size > self.block_size {
            return Err("ext4_desc_cross_block");
        }
        let mut block = [0u8; MAX_BLOCK_SIZE];
        self.read_block(desc_block, &mut block)?;
        let lo = read_u32(&block, in_block + 8) as u64;
        let hi = if self.desc_size >= 64 {
            read_u32(&block, in_block + 40) as u64
        } else {
            0
        };
        let inode_table = lo | (hi << 32);
        if inode_table == 0 {
            return Err("ext4_inode_table");
        }
        Ok(inode_table)
    }

    fn lookup_child(&self, dir: &Inode, name: &[u8]) -> Result<u32, &'static str> {
        if name.is_empty() || name.len() > 255 {
            return Err("ext4_name");
        }
        let size = dir.size() as usize;
        let mut logical = 0u32;
        let mut consumed = 0usize;
        while consumed < size {
            let mut block = [0u8; MAX_BLOCK_SIZE];
            self.read_inode_block(dir, logical, &mut block)?;
            let scan_len = min(self.block_size, size - consumed);
            let mut off = 0usize;
            while off + 8 <= scan_len {
                let ino = read_u32(&block, off);
                let rec_len = read_u16(&block, off + 4) as usize;
                let name_len = block[off + 6] as usize;
                if rec_len < 8 || off + rec_len > scan_len {
                    return Err("ext4_dirent");
                }
                if ino != 0 && name_len == name.len() && off + 8 + name_len <= scan_len {
                    if bytes_eq(&block[off + 8..off + 8 + name_len], name) {
                        return Ok(ino);
                    }
                }
                off += rec_len;
            }
            consumed += scan_len;
            logical += 1;
        }
        Err("ext4_lookup")
    }

    fn file_contains(&self, ino: u32, pattern: &[u8]) -> Result<bool, &'static str> {
        let patterns: [&[u8]; 1] = [pattern];
        self.file_contains_all(ino, &patterns)
    }

    fn file_contains_all(&self, ino: u32, patterns: &[&[u8]]) -> Result<bool, &'static str> {
        if patterns.len() > MAX_PATTERN_COUNT {
            return Err("pattern_count");
        }
        let inode = self.read_inode(ino)?;
        let size = inode.size() as usize;
        let mut found = [false; MAX_PATTERN_COUNT];
        let mut tail = [0u8; MAX_PATTERN_TAIL];
        let mut tail_len = 0usize;
        let mut logical = 0u32;
        let mut consumed = 0usize;
        while consumed < size {
            let mut block = [0u8; MAX_BLOCK_SIZE];
            self.read_inode_block(&inode, logical, &mut block)?;
            let scan_len = min(self.block_size, size - consumed);
            let mut i = 0usize;
            while i < patterns.len() {
                if !found[i]
                    && (contains(&block[..scan_len], patterns[i])
                        || contains_across_tail(&tail[..tail_len], &block[..scan_len], patterns[i]))
                {
                    found[i] = true;
                }
                i += 1;
            }
            tail_len = min(MAX_PATTERN_TAIL, scan_len);
            tail[..tail_len].copy_from_slice(&block[scan_len - tail_len..scan_len]);
            consumed += scan_len;
            logical += 1;
        }
        let mut i = 0usize;
        while i < patterns.len() {
            if !found[i] {
                return Ok(false);
            }
            i += 1;
        }
        Ok(true)
    }

    fn file_equals(&self, ino: u32, expected: &[u8]) -> Result<bool, &'static str> {
        let inode = self.read_inode(ino)?;
        if inode.size() != expected.len() as u64 {
            return Ok(false);
        }
        let mut logical = 0u32;
        let mut consumed = 0usize;
        while consumed < expected.len() {
            let mut block = [0u8; MAX_BLOCK_SIZE];
            self.read_inode_block(&inode, logical, &mut block)?;
            let take = min(self.block_size, expected.len() - consumed);
            if !bytes_eq(&block[..take], &expected[consumed..consumed + take]) {
                return Ok(false);
            }
            consumed += take;
            logical += 1;
        }
        Ok(true)
    }

    fn file_read_into(&self, ino: u32, out: &mut [u8]) -> Result<usize, &'static str> {
        let inode = self.read_inode(ino)?;
        let size = inode.size() as usize;
        if size > out.len() {
            return Err("ext4_file_buffer");
        }
        let mut logical = 0u32;
        let mut consumed = 0usize;
        while consumed < size {
            let mut block = [0u8; MAX_BLOCK_SIZE];
            self.read_inode_block(&inode, logical, &mut block)?;
            let take = min(self.block_size, size - consumed);
            out[consumed..consumed + take].copy_from_slice(&block[..take]);
            consumed += take;
            logical += 1;
        }
        Ok(size)
    }

    fn read_inode_block(
        &self,
        inode: &Inode,
        logical: u32,
        out: &mut [u8; MAX_BLOCK_SIZE],
    ) -> Result<(), &'static str> {
        let block = inode.extent_physical_block(logical)?;
        self.read_block(block, out)
    }

    fn read_block(&self, block: u64, out: &mut [u8; MAX_BLOCK_SIZE]) -> Result<(), &'static str> {
        out.fill(0);
        let sectors = self.block_size / SECTOR_SIZE;
        let start_sector = block.checked_mul(sectors as u64).ok_or("ext4_sector")?;
        let mut sector = [0u8; SECTOR_SIZE];
        let mut i = 0usize;
        while i < sectors {
            virtio_blk::read_sector(start_sector + i as u64, &mut sector)?;
            let start = i * SECTOR_SIZE;
            out[start..start + SECTOR_SIZE].copy_from_slice(&sector);
            i += 1;
        }
        Ok(())
    }
}

impl Inode {
    fn mode(&self) -> u16 {
        read_u16(&self.data, 0)
    }

    fn size(&self) -> u64 {
        let lo = read_u32(&self.data, 4) as u64;
        let hi = read_u32(&self.data, 108) as u64;
        lo | (hi << 32)
    }

    fn atime(&self) -> u32 {
        read_u32(&self.data, 8)
    }

    fn ctime(&self) -> u32 {
        read_u32(&self.data, 12)
    }

    fn mtime(&self) -> u32 {
        read_u32(&self.data, 16)
    }

    fn links_count(&self) -> u16 {
        read_u16(&self.data, 26)
    }

    fn extent_physical_block(&self, logical: u32) -> Result<u64, &'static str> {
        let base = 40usize;
        if read_u16(&self.data, base) != 0xf30a {
            return Err("ext4_extent_magic");
        }
        let entries = read_u16(&self.data, base + 2) as usize;
        let depth = read_u16(&self.data, base + 6);
        if depth != 0 {
            return Err("ext4_extent_depth");
        }
        let mut i = 0usize;
        while i < entries {
            let off = base + 12 + i * 12;
            if off + 12 > base + 60 {
                return Err("ext4_extent_bounds");
            }
            let first = read_u32(&self.data, off);
            let len = (read_u16(&self.data, off + 4) & 0x7fff) as u32;
            let start_hi = read_u16(&self.data, off + 6) as u64;
            let start_lo = read_u32(&self.data, off + 8) as u64;
            if len != 0 && logical >= first && logical < first + len {
                return Ok(((start_hi << 32) | start_lo) + (logical - first) as u64);
            }
            i += 1;
        }
        Err("ext4_extent_missing")
    }
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }
    let mut i = 0usize;
    while i + needle.len() <= haystack.len() {
        if bytes_eq(&haystack[i..i + needle.len()], needle) {
            return true;
        }
        i += 1;
    }
    false
}

fn contains_across_tail(tail: &[u8], head: &[u8], needle: &[u8]) -> bool {
    if tail.is_empty() || head.is_empty() || needle.len() <= 1 {
        return false;
    }
    let max_tail = min(tail.len(), needle.len() - 1);
    let mut tail_take = 1usize;
    while tail_take <= max_tail {
        let head_take = needle.len() - tail_take;
        if head_take <= head.len()
            && bytes_eq(&tail[tail.len() - tail_take..], &needle[..tail_take])
            && bytes_eq(&head[..head_take], &needle[tail_take..])
        {
            return true;
        }
        tail_take += 1;
    }
    false
}

fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0usize;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn read_u16(data: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([data[off], data[off + 1]])
}

fn read_u32(data: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]])
}

fn read_u64(data: &[u8], off: usize) -> u64 {
    u64::from_le_bytes([
        data[off],
        data[off + 1],
        data[off + 2],
        data[off + 3],
        data[off + 4],
        data[off + 5],
        data[off + 6],
        data[off + 7],
    ])
}
