//! Shared read-only rootfs and VFS gateway.

pub mod ext4;
pub mod fd;
pub mod stat;
pub mod vfs;

pub use fd::{seek_offset, FdError, FdTable, OpenFileDescription, OpenOptions, OpenOptionsError};
pub use stat::{encode_linux_stat, StatEncodingError, LINUX_STAT_SIZE};
pub use vfs::{
    FileIdentity, MountedRootfs, NoSyscallVfs, SyscallVfs, VfsDirEntry, VfsDirEntryKind, VfsError,
    VfsNodeKind, VfsPath, VfsReadResult, VfsRuntimeError, VfsStat,
};
