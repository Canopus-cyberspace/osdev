//! Shared read-only rootfs and VFS gateway.

pub mod ext4;
pub mod fd;
pub mod stat;
pub mod vfs;

pub use fd::{
    seek_offset, FdError, FdTable, OpenFileDescription, OpenOptions, OpenOptionsError, PipeTable,
};
pub use stat::{encode_linux_stat, StatEncodingError, LINUX_STAT_SIZE};
pub use vfs::{
    builtin_dir_entry_at, builtin_ioctl, builtin_open, builtin_read_at, builtin_stat,
    builtin_stat_open, builtin_write, resolve_path, FileIdentity, MountedRootfs, NoSyscallVfs,
    SyscallVfs, VfsDirEntry, VfsDirEntryKind, VfsError, VfsIoctl, VfsMountRecord, VfsMountTable,
    VfsNodeKind, VfsPath, VfsPathBuffer, VfsReadResult, VfsRuntimeError, VfsStat, WritableOverlay,
    VFS_MOUNT_CAPACITY, VFS_NAME_MAX, VFS_OVERLAY_DIR_CAPACITY, VFS_PATH_MAX,
};
