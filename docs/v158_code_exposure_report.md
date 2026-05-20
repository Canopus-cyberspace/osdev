# v158 当前代码启用/禁用/历史残留清单

## 已接入稳定模块
- `ucompat_vfs_tree_v151k7`: `src/fs/ucompat_vfs_tree_v151k7.rs` -> `run_v151k7_tree_scenario` -> `[ucompat-v151k7] vfs_tree_dirfd_multiinode PASS`
- `ucompat_vfs_core_v154`: `src/fs/ucompat_vfs_core_v154.rs` -> `run_v154_core_scenario` -> `[ucompat-v154] fs_core_multi_feature PASS`
- `ucompat_vfs_core_v155`: `src/fs/ucompat_vfs_core_v155.rs` -> `run_v155_namespace_procfd_scenario` -> `[ucompat-v155] namespace_procfd_multi_feature PASS`
- `ucompat_procfs_v156`: `src/fs/ucompat_procfs_v156.rs` -> `run_v156_procfs_fd_observability_scenario` -> `[ucompat-v156] procfs_fd_observability PASS`
- `ucompat_proc_task_v157`: `src/fs/ucompat_proc_task_v157.rs` -> `run_v157_process_signal_task_scenario` -> `[ucompat-v157] process_signal_task PASS`
- `ucompat_vfs_tree_v151k`: `src/fs/ucompat_vfs_tree_v151k.rs` -> `run_v151k_tree_scenario` -> `[ucompat-v158] ucompat_vfs_tree_v151k PASS`

## 当前 src/fs/mod.rs 暴露模块
- `ucompat_vfs_tree`
- `ucompat_vfs`
- `ucompat_fdtable`
- `ucompat_memfs`
- `devfs`
- `ext4`
- `fat32`
- `fd_table`
- `file`
- `pipe`
- `procfs`
- `tmpfs`
- `vfs`
- `ucompat_vfs_tree_v151k7`
- `ucompat_vfs_core_v154`
- `ucompat_vfs_core_v155`
- `ucompat_procfs_v156`
- `ucompat_proc_task_v157`
- `ucompat_vfs_tree_v151k`
- `ucompat_history_v158`

## ucompat 源文件清单
- `src/fs/ucompat_fdtable.rs`
- `src/fs/ucompat_history_v158.rs`
- `src/fs/ucompat_memfs.rs`
- `src/fs/ucompat_proc_task_v157.rs`
- `src/fs/ucompat_procfs_v156.rs`
- `src/fs/ucompat_vfs.rs`
- `src/fs/ucompat_vfs_core_v154.rs`
- `src/fs/ucompat_vfs_core_v155.rs`
- `src/fs/ucompat_vfs_namespace_v152.rs`
- `src/fs/ucompat_vfs_namespace_v152b.rs`
- `src/fs/ucompat_vfs_tree.rs`
- `src/fs/ucompat_vfs_tree_v151k.rs`
- `src/fs/ucompat_vfs_tree_v151k3.rs`
- `src/fs/ucompat_vfs_tree_v151k4.rs`
- `src/fs/ucompat_vfs_tree_v151k5.rs`
- `src/fs/ucompat_vfs_tree_v151k6.rs`
- `src/fs/ucompat_vfs_tree_v151k7.rs`

## 失败/归档残留，v158 不盲目启用
- `src/fs/ucompat_vfs_namespace_v152.rs`
- `src/fs/ucompat_vfs_namespace_v152b.rs`
- `src/fs/ucompat_vfs_tree_v151k3.rs`
- `src/fs/ucompat_vfs_tree_v151k4.rs`
- `src/fs/ucompat_vfs_tree_v151k5.rs`
- `src/fs/ucompat_vfs_tree_v151k6.rs`

结论：当前包会接入稳定历史实现；失败实验代码如存在会保留为源码档案或后续修复对象，而不会默认启用破坏 v156 基线。
