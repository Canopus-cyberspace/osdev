#![allow(dead_code)]

use crate::fd::{FdTable, FileKind, RuntimeReadTarget, RuntimeWriteTarget};
use crate::loader::process_image::build_init_process_info;
use crate::loader::user_stack::build_initial_user_stack_dry_run;
use crate::process::{make_init_process, make_zombie};

pub const SYS_OPENAT: usize = 56;
pub const SYS_CLOSE: usize = 57;
pub const SYS_GETDENTS64: usize = 61;
pub const SYS_LSEEK: usize = 62;
pub const SYS_READ: usize = 63;
pub const SYS_WRITE: usize = 64;
pub const SYS_FSTAT: usize = 80;
pub const SYS_EXIT: usize = 93;
pub const SYS_GETPID: usize = 172;
pub const SYS_GETPPID: usize = 173;
pub const SYS_BRK: usize = 214;
pub const SYS_CLONE: usize = 220;
pub const SYS_EXECVE: usize = 221;
pub const SYS_WAIT4: usize = 260;

pub const ENOSYS: isize = -38;
pub const EBADF: isize = -9;
pub const ENOENT: isize = -2;
pub const EINVAL: isize = -22;
pub const ESPIPE: isize = -29;
pub const ENOTDIR: isize = -20;

#[derive(Copy, Clone, Debug)]
pub struct SyscallFrame {
    pub id: usize,
    pub args: [usize; 6],
}

impl SyscallFrame {
    pub const fn new(id: usize, args: [usize; 6]) -> Self { Self { id, args } }
}

pub fn dispatch_scaffold(frame: SyscallFrame) -> isize {
    match frame.id {
        SYS_GETPID => 1,
        SYS_GETPPID => 0,
        SYS_EXECVE => sys_execve_scaffold(frame.args[0], frame.args[1], frame.args[2]),
        SYS_WAIT4 => sys_wait4_scaffold(),
        SYS_CLONE => sys_clone_scaffold(),
        _ => ENOSYS,
    }
}

pub fn sys_execve_scaffold(_path: usize, _argv: usize, _envp: usize) -> isize {
    let info = match build_init_process_info() { Ok(info) => info, Err(_) => return EINVAL };
    let stack = match build_initial_user_stack_dry_run() { Ok(stack) => stack, Err(_) => return EINVAL };
    if info.entry() == 0 || stack.initial_sp == 0 { return EINVAL; }
    0
}

pub const fn sys_wait4_scaffold() -> isize { ENOSYS }
pub const fn sys_clone_scaffold() -> isize { ENOSYS }

#[derive(Copy, Clone, Debug)]
pub struct RuntimeSyscallArgs {
    pub id: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
}

impl RuntimeSyscallArgs {
    pub const fn new(id: usize, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> Self {
        Self { id, a0, a1, a2, a3, a4, a5 }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RuntimeSyscallAction {
    Return(isize),
    Write { fd: usize, user_ptr: usize, len: usize, target: RuntimeWriteTarget },
    Read { fd: usize, user_ptr: usize, len: usize, target: RuntimeReadTarget },
    OpenAt { dirfd: isize, user_path: usize, flags: usize, mode: usize },
    Close { fd: usize },
    FStat { fd: usize, user_stat: usize },
    LSeek { fd: usize, offset: isize, whence: usize },
    GetDents64 { fd: usize, user_dirent: usize, len: usize },
    Brk { addr: usize },
    Exit { code: isize },
}

pub fn dispatch_runtime_syscall(args: RuntimeSyscallArgs) -> RuntimeSyscallAction {
    match args.id {
        SYS_OPENAT => RuntimeSyscallAction::OpenAt {
            dirfd: args.a0 as isize,
            user_path: args.a1,
            flags: args.a2,
            mode: args.a3,
        },
        SYS_CLOSE => RuntimeSyscallAction::Close { fd: args.a0 },
        SYS_GETDENTS64 => RuntimeSyscallAction::GetDents64 {
            fd: args.a0,
            user_dirent: args.a1,
            len: args.a2,
        },
        SYS_LSEEK => RuntimeSyscallAction::LSeek { fd: args.a0, offset: args.a1 as isize, whence: args.a2 },
        SYS_READ => match crate::fd::runtime_read_target(args.a0) {
            Ok(target) => RuntimeSyscallAction::Read { fd: args.a0, user_ptr: args.a1, len: args.a2, target },
            Err(err) => RuntimeSyscallAction::Return(err),
        },
        SYS_WRITE => match crate::fd::runtime_write_target(args.a0) {
            Ok(target) => RuntimeSyscallAction::Write { fd: args.a0, user_ptr: args.a1, len: args.a2, target },
            Err(err) => RuntimeSyscallAction::Return(err),
        },
        SYS_FSTAT => RuntimeSyscallAction::FStat { fd: args.a0, user_stat: args.a1 },
        SYS_BRK => RuntimeSyscallAction::Brk { addr: args.a0 },
        SYS_EXIT => RuntimeSyscallAction::Exit { code: args.a0 as isize },
        SYS_GETPID => RuntimeSyscallAction::Return(1),
        SYS_GETPPID => RuntimeSyscallAction::Return(0),
        _ => RuntimeSyscallAction::Return(ENOSYS),
    }
}

pub fn self_test() {
    crate::println!("[syscall-dispatch-v59] self-test begin");

    match dispatch_runtime_syscall(RuntimeSyscallArgs::new(SYS_GETDENTS64, 5, 0x4001_fe00, 256, 0, 0, 0)) {
        RuntimeSyscallAction::GetDents64 { fd, user_dirent, len } => {
            crate::println!("[syscall-dispatch-v59] getdents fd = {}", fd);
            crate::println!("[syscall-dispatch-v59] getdents buf = {:#x}", user_dirent);
            crate::println!("[syscall-dispatch-v59] getdents len = {}", len);
        }
        _ => panic!("[syscall-dispatch-v59] getdents dispatch failed"),
    }

    let getpid = dispatch_runtime_syscall(RuntimeSyscallArgs::new(SYS_GETPID, 0, 0, 0, 0, 0, 0));
    let getppid = dispatch_runtime_syscall(RuntimeSyscallArgs::new(SYS_GETPPID, 0, 0, 0, 0, 0, 0));
    let unsupported = dispatch_runtime_syscall(RuntimeSyscallArgs::new(9999, 0, 0, 0, 0, 0, 0));
    let exit = dispatch_runtime_syscall(RuntimeSyscallArgs::new(SYS_EXIT, 0, 0, 0, 0, 0, 0));

    assert_eq!(getpid, RuntimeSyscallAction::Return(1));
    assert_eq!(getppid, RuntimeSyscallAction::Return(0));
    assert_eq!(unsupported, RuntimeSyscallAction::Return(ENOSYS));
    assert_eq!(exit, RuntimeSyscallAction::Exit { code: 0 });

    let exec_ret = dispatch_scaffold(SyscallFrame::new(SYS_EXECVE, [0; 6]));
    crate::println!("[syscall-dispatch-v59] execve scaffold ret = {}", exec_ret);

    if let Ok(info) = build_init_process_info() {
        let init = make_init_process(info);
        let zombie = make_zombie(init, 0);
        crate::println!("[syscall-dispatch-v59] zombie exit = {}", zombie.exit_code);
    }

    let mut fd_table = FdTable::with_stdio();
    if let Some(dev_dir_fd) = fd_table.alloc(FileKind::DevDir, true, false) {
        let closed = fd_table.close(dev_dir_fd);
        crate::println!("[syscall-dispatch-v59] devdir fd closed = {}", closed as usize);
    }

    crate::println!("[syscall-dispatch-v59] self-test passed");
}
