use core::sync::atomic::{AtomicUsize, Ordering};

use crate::core::fs::{
    seek_offset, FdError, FdTable, MountedRootfs, OpenOptions, SyscallVfs, VfsDirEntry, VfsError,
    VfsPath, VfsRuntimeError, VfsStat,
};
use crate::core::mm::{UserMemoryMapper, UserMemoryReader, UserMemoryWriter};
use crate::core::syscall::{
    dispatch_single_with_runtime, dispatch_with_runtime, SyscallFrame, SyscallOutcome,
};
use crate::core::task::Process;

static ACTIVE_ROOTFS: AtomicUsize = AtomicUsize::new(0);
static mut ACTIVE_FD_TABLE: FdTable = FdTable::new();

pub fn install_rootfs(rootfs: &mut MountedRootfs) {
    active_fd_table_mut().reset();
    ACTIVE_ROOTFS.store(rootfs as *mut MountedRootfs as usize, Ordering::Release);
}

pub fn clear_rootfs() {
    ACTIVE_ROOTFS.store(0, Ordering::Release);
    active_fd_table_mut().reset();
}

pub fn dispatch_with_memory<R: UserMemoryReader + UserMemoryWriter + UserMemoryMapper>(
    frame: SyscallFrame,
    process: &mut Process,
    memory: &R,
) -> SyscallOutcome {
    let vfs = ActiveSyscallVfs;
    dispatch_with_runtime(frame, process, memory, &vfs)
}

pub fn dispatch_single_with_memory<R: UserMemoryReader + UserMemoryWriter + UserMemoryMapper>(
    frame: SyscallFrame,
    memory: &R,
) -> SyscallOutcome {
    let vfs = ActiveSyscallVfs;
    dispatch_single_with_runtime(frame, memory, &vfs)
}

#[derive(Clone, Copy)]
struct ActiveSyscallVfs;

impl SyscallVfs for ActiveSyscallVfs {
    fn stat_path(&self, path: VfsPath<'_>) -> Result<VfsStat, VfsError> {
        active_rootfs_mut()
            .map_err(runtime_to_vfs_error)?
            .stat(path)
    }

    fn open_path(&self, path: VfsPath<'_>, options: OpenOptions) -> Result<usize, VfsRuntimeError> {
        let description = active_rootfs_mut()?
            .open_with_options(path, options)
            .map_err(VfsRuntimeError::Vfs)?;
        active_fd_table_mut()
            .insert(description)
            .map_err(VfsRuntimeError::Fd)
    }

    fn close_fd(&self, fd: usize) -> Result<(), FdError> {
        active_fd_table_mut().close(fd)
    }

    fn fstat_fd(&self, fd: usize) -> Result<VfsStat, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        Ok(active_rootfs_mut()?.stat_open(description))
    }

    fn read_fd(&self, fd: usize, out: &mut [u8]) -> Result<usize, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        if !description.is_regular() {
            return Err(VfsRuntimeError::Fd(FdError::NotRegularFile));
        }
        active_rootfs_mut()?
            .read_at(&description, description.offset(), out)
            .map(|read| read.bytes())
            .map_err(VfsRuntimeError::Vfs)
    }

    fn lseek_fd(&self, fd: usize, offset: i64, whence: usize) -> Result<u64, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        let next = seek_offset(
            description.offset(),
            description.identity().byte_len(),
            offset,
            whence,
        )
        .map_err(VfsRuntimeError::Fd)?;
        active_fd_table_mut()
            .set_offset(fd, next)
            .map_err(VfsRuntimeError::Fd)?;
        Ok(next)
    }

    fn dir_entry_at_fd(&self, fd: usize) -> Result<Option<VfsDirEntry>, VfsRuntimeError> {
        let description = active_fd_table_mut().get(fd).map_err(VfsRuntimeError::Fd)?;
        if !description.is_directory() {
            return Err(VfsRuntimeError::Fd(FdError::NotDirectory));
        }
        active_rootfs_mut()?
            .dir_entry_at(description, description.offset())
            .map_err(VfsRuntimeError::Vfs)
    }

    fn set_fd_offset(&self, fd: usize, offset: u64) -> Result<(), FdError> {
        active_fd_table_mut().set_offset(fd, offset)
    }

    fn set_fd_close_on_exec(&self, fd: usize, close_on_exec: bool) -> Result<(), FdError> {
        active_fd_table_mut().set_close_on_exec(fd, close_on_exec)
    }
}

fn active_rootfs_mut() -> Result<&'static mut MountedRootfs, VfsRuntimeError> {
    let raw = ACTIVE_ROOTFS.load(Ordering::Acquire);
    if raw == 0 {
        return Err(VfsRuntimeError::Vfs(VfsError::RootfsSourceMissing));
    }

    Ok(unsafe { &mut *(raw as *mut MountedRootfs) })
}

fn active_fd_table_mut() -> &'static mut FdTable {
    unsafe { &mut *core::ptr::addr_of_mut!(ACTIVE_FD_TABLE) }
}

fn runtime_to_vfs_error(error: VfsRuntimeError) -> VfsError {
    match error {
        VfsRuntimeError::Vfs(error) => error,
        VfsRuntimeError::Fd(_) => VfsError::RootfsSourceMissing,
    }
}
