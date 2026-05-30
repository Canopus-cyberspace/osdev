//! Kernel orchestration for file-backed exec.

use core::sync::atomic::{AtomicUsize, Ordering};

use crate::arch::contract::{
    Architecture, BootInitPath, BoundaryMode, BspServices, UserEntryState, UserMmuBlocker,
    UserMmuState, UserMmuUnsupported, BOOT_INIT_ARG_COUNT,
};
use crate::core::block::BlockIoError;
use crate::core::fs::{FileIdentity, MountedRootfs, VfsError, VfsPath};
use crate::core::loader::{prepare_executable_image, AuxEntry, ExecutableAbi, LoaderBlocker};
use crate::core::mm::{MemoryFoundation, PAGE_SIZE};
use crate::core::task::{
    single_pid, single_set_user_memory_state, ExecCommitBlocker, PendingUserEntry, Process,
};

#[no_mangle]
static EXEC_STAGE: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
static BOOT_INIT_EXEC_BLOCKER: AtomicUsize = AtomicUsize::new(0);

const EM_RISCV: u16 = 243;
const EM_LOONGARCH: u16 = 258;
const BOOT_EXEC_BUFFER_SIZE: usize = 2 * 1024 * 1024;
const USER_MMAP_BASE: usize = 0x1_0000_0000;
const AT_PAGESZ: usize = 6;

#[repr(align(4096))]
struct BootExecBuffer {
    bytes: [u8; BOOT_EXEC_BUFFER_SIZE],
}

static mut BOOT_EXEC_BUFFER: BootExecBuffer = BootExecBuffer {
    bytes: [0; BOOT_EXEC_BUFFER_SIZE],
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileBackedExec {
    file: FileIdentity,
    bytes_read: usize,
    pending: PendingUserEntry,
}

impl FileBackedExec {
    pub const fn file(self) -> FileIdentity {
        self.file
    }

    pub const fn bytes_read(self) -> usize {
        self.bytes_read
    }

    pub const fn pending(self) -> PendingUserEntry {
        self.pending
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileExecBlocker {
    ExecCommit(ExecCommitBlocker),
    Loader(LoaderBlocker),
    UserMmu(UserMmuBlocker),
    UserMmuNotApplied,
    UserMmuUnsupported(UserMmuUnsupported),
    Vfs(VfsError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootInitExecBlocker {
    FileExec(FileExecBlocker),
    PendingEntryMissing,
    Rootfs(VfsError),
    UserEntryReturned {
        file_exec: FileBackedExec,
        state: UserEntryState,
    },
    VfsPath(VfsError),
}

pub fn commit_file_backed_exec(
    bsp: BspServices,
    memory: &mut MemoryFoundation,
    process: &mut Process,
    rootfs: &mut MountedRootfs,
    path: VfsPath<'_>,
    abi: ExecutableAbi,
    file_buffer: &mut [u8],
    argv: &[&[u8]],
) -> Result<FileBackedExec, FileExecBlocker> {
    EXEC_STAGE.store(1, Ordering::Relaxed);
    let file = rootfs.open(path).map_err(FileExecBlocker::Vfs)?;
    EXEC_STAGE.store(2, Ordering::Relaxed);
    if !file.identity().is_regular() {
        return Err(FileExecBlocker::Vfs(VfsError::NotRegularFile));
    }
    if !file.identity().executable() {
        return Err(FileExecBlocker::Vfs(VfsError::NotExecutable));
    }
    let read = rootfs
        .read_all(&file, file_buffer)
        .map_err(FileExecBlocker::Vfs)?;
    let bytes = read.bytes();
    EXEC_STAGE.store(3, Ordering::Relaxed);
    let auxv = [AuxEntry::new(AT_PAGESZ, PAGE_SIZE)];
    let load = prepare_executable_image(
        Some(&file_buffer[..bytes]),
        memory.kernel_globals(),
        abi,
        argv,
        &[],
        &auxv,
    )
    .map_err(FileExecBlocker::Loader)?;
    EXEC_STAGE.store(4, Ordering::Relaxed);

    let user_mmu = bsp.prepare_user_mmu(
        memory.frames_mut(),
        load.address_space_load(),
        BoundaryMode::ApplyUnsafe,
    );
    let address_space = match user_mmu {
        UserMmuState::Applied(address_space) => address_space,
        UserMmuState::Planned(_) | UserMmuState::Prepared(_) => {
            return Err(FileExecBlocker::UserMmuNotApplied);
        }
        UserMmuState::NotReady(blocker) => return Err(FileExecBlocker::UserMmu(blocker)),
        UserMmuState::Unsupported(blocker) => {
            return Err(FileExecBlocker::UserMmuUnsupported(blocker));
        }
    };

    EXEC_STAGE.store(5, Ordering::Relaxed);
    let image = load
        .complete(address_space)
        .map_err(FileExecBlocker::Loader)?;
    let pending = process
        .commit_exec(image)
        .map_err(FileExecBlocker::ExecCommit)?;
    single_set_user_memory_state(process.heap_base(), process.program_break(), USER_MMAP_BASE);
    EXEC_STAGE.store(6, Ordering::Relaxed);
    Ok(FileBackedExec {
        file: file.identity(),
        bytes_read: bytes,
        pending,
    })
}

pub fn drive_boot_init_exec(
    bsp: BspServices,
    memory: &mut MemoryFoundation,
    init_path: BootInitPath,
) -> Result<(), BootInitExecBlocker> {
    let mut rootfs = match MountedRootfs::mount_ext4(bsp.block_provider()) {
        Ok(rootfs) => rootfs,
        Err(error) => {
            record_boot_init_exec_blocker(boot_init_rootfs_blocker_code(error));
            return Err(BootInitExecBlocker::Rootfs(error));
        }
    };
    let path = match VfsPath::new(init_path.bytes()) {
        Ok(path) => path,
        Err(error) => {
            record_boot_init_exec_blocker(boot_init_vfs_path_blocker_code(error));
            return Err(BootInitExecBlocker::VfsPath(error));
        }
    };
    let mut process = Process::new(single_pid());
    let abi = executable_abi(bsp.snapshot().boot().architecture());
    let file_buffer = unsafe { boot_exec_buffer() };
    let (argv_storage, argv_len) = boot_argv(&init_path);
    let file_exec = commit_file_backed_exec(
        bsp,
        memory,
        &mut process,
        &mut rootfs,
        path,
        abi,
        file_buffer,
        &argv_storage[..argv_len],
    )
    .map_err(|blocker| {
        record_boot_init_exec_blocker(file_exec_blocker_code(blocker));
        BootInitExecBlocker::FileExec(blocker)
    })?;
    let pending = process
        .take_pending_entry()
        .map_err(|_| BootInitExecBlocker::PendingEntryMissing)?;
    EXEC_STAGE.store(7, Ordering::Relaxed);
    crate::kernel::syscall_runtime::install_rootfs(&mut rootfs);
    let state = bsp.enter_user(pending, BoundaryMode::ApplyUnsafe);
    crate::kernel::syscall_runtime::clear_rootfs();
    Err(BootInitExecBlocker::UserEntryReturned { file_exec, state })
}

fn record_boot_init_exec_blocker(code: usize) {
    BOOT_INIT_EXEC_BLOCKER.store(code, Ordering::Relaxed);
}

const fn boot_init_rootfs_blocker_code(error: VfsError) -> usize {
    1000 + vfs_error_code(error)
}

const fn boot_init_vfs_path_blocker_code(error: VfsError) -> usize {
    2000 + vfs_error_code(error)
}

const fn file_exec_blocker_code(blocker: FileExecBlocker) -> usize {
    match blocker {
        FileExecBlocker::Vfs(error) => 3000 + vfs_error_code(error),
        FileExecBlocker::Loader(_) => 4000,
        FileExecBlocker::UserMmu(_) => 5000,
        FileExecBlocker::UserMmuNotApplied => 5001,
        FileExecBlocker::UserMmuUnsupported(_) => 5002,
        FileExecBlocker::ExecCommit(_) => 6000,
    }
}

const fn vfs_error_code(error: VfsError) -> usize {
    match error {
        VfsError::Block(error) => 100 + block_error_code(error),
        VfsError::DirectoryExpected => 1,
        VfsError::EmptyFile => 2,
        VfsError::FileTooLarge => 3,
        VfsError::InvalidPath => 4,
        VfsError::MetadataCorrupt => 5,
        VfsError::NotExecutable => 6,
        VfsError::NotRegularFile => 7,
        VfsError::PathNotFound => 8,
        VfsError::RootfsSourceMissing => 9,
        VfsError::UnsupportedPath => 10,
        VfsError::UnsupportedRootfs => 11,
    }
}

const fn block_error_code(error: BlockIoError) -> usize {
    match error {
        BlockIoError::CompletionTimeout => 1,
        BlockIoError::DeviceKindMismatch => 2,
        BlockIoError::DeviceStatusNonZero => 3,
        BlockIoError::FeatureNegotiationFailed => 4,
        BlockIoError::ProviderMissing => 5,
        BlockIoError::QueueUnavailable => 6,
        BlockIoError::RequestOverflow => 7,
        BlockIoError::SectorOutOfRange => 8,
        BlockIoError::UnsupportedSectorSize => 9,
        BlockIoError::UnsupportedTransport => 10,
    }
}

fn boot_argv(init_path: &BootInitPath) -> ([&[u8]; BOOT_INIT_ARG_COUNT + 1], usize) {
    let empty: &[u8] = &[];
    let mut argv = [empty; BOOT_INIT_ARG_COUNT + 1];
    argv[0] = init_path.bytes();
    let mut len = 1usize;
    let mut index = 0usize;
    while index < init_path.arg_count() {
        if let Some(arg) = init_path.arg(index) {
            argv[len] = arg;
            len += 1;
        }
        index += 1;
    }

    (argv, len)
}

fn executable_abi(architecture: Architecture) -> ExecutableAbi {
    match architecture {
        Architecture::Riscv64 => ExecutableAbi::new(EM_RISCV),
        Architecture::LoongArch64 => ExecutableAbi::new(EM_LOONGARCH),
    }
}

unsafe fn boot_exec_buffer() -> &'static mut [u8] {
    let ptr = core::ptr::addr_of_mut!(BOOT_EXEC_BUFFER.bytes) as *mut u8;
    core::slice::from_raw_parts_mut(ptr, BOOT_EXEC_BUFFER_SIZE)
}
