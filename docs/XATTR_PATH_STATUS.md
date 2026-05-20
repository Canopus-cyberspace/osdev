# XATTR_PATH_STATUS

## v79 - xattr/path/permission scaffold

Implemented as smoke-level compatibility scaffolds:
- setxattr/lsetxattr/fsetxattr
- getxattr/lgetxattr/fgetxattr
- listxattr/llistxattr/flistxattr
- removexattr/lremovexattr/fremovexattr
- lookup_dcookie
- symlinkat/linkat
- pivot_root/nfsservctl
- fchdir/chroot
- fchmod/fchmodat/fchownat/fchown
- vhangup/quotactl

Still TODO:
- real xattr storage per inode
- real link count updates
- symlink inode creation and path resolution
- real permission and ownership model
- mount namespace root pivot/chroot semantics
- quota engine
