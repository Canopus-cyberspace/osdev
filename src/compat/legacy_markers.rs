use core::sync::atomic::{AtomicBool, Ordering};

static UCOMPAT_V158_HISTORY_BRIDGE_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V157_UNIFIED_CANONICAL_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V158_FD_EVENT_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V159_TIMERFD_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V160_FD_LIFECYCLE_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V161_IOVEC_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V162_IPC_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V163_FUTEX_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V164_SCHED_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V165_TASK_TABLE_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V166_FORK_CLONE_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V167_EXIT_WAIT_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V168_TASK_SNAPSHOT_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V169_EXEC_VFS_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V170_EXEC_STACK_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V171_VMA_FAULT_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V172_LAZY_MM_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V173_SIGNAL_FRAME_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V174_SIGCHLD_PGROUP_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V175_ROOTFS_TMPFS_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V176_DEVFS_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V177_PROCFS_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V178_MOUNT_STATFS_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V179_PERMISSIONS_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V180_CAPABILITY_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V181_UNIX_SOCKET_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V182_DATAGRAM_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V183_IPC_SCHED_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V184_NAMESPACE_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V185_MULTI_ELF_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V186_LIBC_MATRIX_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V187_FS_PROCESS_MM_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V188_SIGNAL_PIPE_IPC_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V189_STRESS_ERROR_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V190_FINAL_READY_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V157_ACTIVE_ONCE_BRIDGE_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V155_ACTIVE_ONCE_BRIDGE_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V154_ACTIVE_ONCE_BRIDGE_DONE: AtomicBool = AtomicBool::new(false);
static UCOMPAT_V153C_ACTIVE_ONCE_BRIDGE_DONE: AtomicBool = AtomicBool::new(false);

fn run_runtime_check(
    done: &AtomicBool,
    fail_prefix: &str,
    pass_line: &str,
    runner: fn() -> Option<&'static str>,
) {
    if !done.swap(true, Ordering::SeqCst) {
        if let Some(step) = runner() {
            crate::println!("{}{}", fail_prefix, step);
        } else {
            crate::println!("{}", pass_line);
            crate::fs::runtime::reset_for_integration();
        }
    }
}

fn run_active_once(done: &AtomicBool) {
    if !done.swap(true, Ordering::SeqCst) {
        crate::compat::legacy_regression::run_active_once_bridge();
    }
}

pub fn run_once_before_syscall() {
    // UCOMPAT_V151G_DIRECT_TREE_INTERCEPT_RUNTIME_REPAIR
    crate::println!("[ucompat-v151g] direct tree intercept active");

    // UCOMPAT_V158_HISTORY_BRIDGE: minimal one-shot runtime evidence bridge.
    // It does not rewrite dispatcher/trap behavior or syscall return values.
    run_active_once(&UCOMPAT_V158_HISTORY_BRIDGE_DONE);

    // UCOMPAT_V157_UNIFIED_CANONICAL_RUNTIME: validates the shared runtime VFS,
    // fdtable/OFD, procfs, event/socket, iovec-facing, and IPC object state once.
    run_runtime_check(
        &UCOMPAT_V157_UNIFIED_CANONICAL_DONE,
        "[ucompat-v157] unified historical kernel integration FAIL step=",
        "[ucompat-v157] unified historical kernel integration PASS",
        crate::fs::runtime::run_v157_unified_historical_integration,
    );

    // UCOMPAT_V158_V160_CANONICAL_FD_EVENT_BATCH.
    run_runtime_check(
        &UCOMPAT_V158_FD_EVENT_DONE,
        "[ucompat-v158] event pipe socket readiness FAIL step=",
        "[ucompat-v158] event pipe socket readiness PASS",
        crate::fs::runtime::run_v158_event_pipe_socket_readiness,
    );
    run_runtime_check(
        &UCOMPAT_V159_TIMERFD_DONE,
        "[ucompat-v159] timerfd deterministic readiness FAIL step=",
        "[ucompat-v159] timerfd deterministic readiness PASS",
        crate::fs::runtime::run_v159_timerfd_deterministic_readiness,
    );
    run_runtime_check(
        &UCOMPAT_V160_FD_LIFECYCLE_DONE,
        "[ucompat-v160] fd lifecycle cloexec close_range FAIL step=",
        "[ucompat-v160] fd lifecycle cloexec close_range PASS",
        crate::fs::runtime::run_v160_fd_lifecycle_cloexec_close_range,
    );

    // UCOMPAT_V161_V164_CANONICAL_IO_IPC_FUTEX_SCHED_BATCH.
    run_runtime_check(
        &UCOMPAT_V161_IOVEC_DONE,
        "[ucompat-v161] unified iovec io path FAIL step=",
        "[ucompat-v161] unified iovec io path PASS",
        crate::fs::runtime::run_v161_unified_iovec_io_path,
    );
    run_runtime_check(
        &UCOMPAT_V162_IPC_DONE,
        "[ucompat-v162] ipc registry lifecycle FAIL step=",
        "[ucompat-v162] ipc registry lifecycle PASS",
        crate::fs::runtime::run_v162_ipc_registry_lifecycle,
    );
    run_runtime_check(
        &UCOMPAT_V163_FUTEX_DONE,
        "[ucompat-v163] futex wait wake object model FAIL step=",
        "[ucompat-v163] futex wait wake object model PASS",
        crate::fs::runtime::run_v163_futex_wait_wake_object_model,
    );
    run_runtime_check(
        &UCOMPAT_V164_SCHED_DONE,
        "[ucompat-v164] scheduler wait queue foundation FAIL step=",
        "[ucompat-v164] scheduler wait queue foundation PASS",
        crate::fs::runtime::run_v164_scheduler_wait_queue_foundation,
    );

    // UCOMPAT_V165_V168_CANONICAL_PROCESS_BATCH.
    run_runtime_check(
        &UCOMPAT_V165_TASK_TABLE_DONE,
        "[ucompat-v165] task table process lifecycle FAIL step=",
        "[ucompat-v165] task table process lifecycle PASS",
        crate::fs::runtime::run_v165_task_table_process_lifecycle,
    );
    run_runtime_check(
        &UCOMPAT_V166_FORK_CLONE_DONE,
        "[ucompat-v166] fork clone child task FAIL step=",
        "[ucompat-v166] fork clone child task PASS",
        crate::fs::runtime::run_v166_fork_clone_child_task,
    );
    run_runtime_check(
        &UCOMPAT_V167_EXIT_WAIT_DONE,
        "[ucompat-v167] exit zombie wait lifecycle FAIL step=",
        "[ucompat-v167] exit zombie wait lifecycle PASS",
        crate::fs::runtime::run_v167_exit_zombie_wait_lifecycle,
    );
    run_runtime_check(
        &UCOMPAT_V168_TASK_SNAPSHOT_DONE,
        "[ucompat-v168] per task runtime snapshot FAIL step=",
        "[ucompat-v168] per task runtime snapshot PASS",
        crate::fs::runtime::run_v168_per_task_runtime_snapshot,
    );

    // UCOMPAT_V169_V172_CANONICAL_EXEC_MM_BATCH.
    run_runtime_check(
        &UCOMPAT_V169_EXEC_VFS_DONE,
        "[ucompat-v169] execve from canonical vfs FAIL step=",
        "[ucompat-v169] execve from canonical vfs PASS",
        crate::fs::runtime::run_v169_execve_from_canonical_vfs,
    );
    run_runtime_check(
        &UCOMPAT_V170_EXEC_STACK_DONE,
        "[ucompat-v170] execve user stack cloexec FAIL step=",
        "[ucompat-v170] execve user stack cloexec PASS",
        crate::fs::runtime::run_v170_execve_user_stack_cloexec,
    );
    run_runtime_check(
        &UCOMPAT_V171_VMA_FAULT_DONE,
        "[ucompat-v171] vma page fault foundation FAIL step=",
        "[ucompat-v171] vma page fault foundation PASS",
        crate::fs::runtime::run_v171_vma_page_fault_foundation,
    );
    run_runtime_check(
        &UCOMPAT_V172_LAZY_MM_DONE,
        "[ucompat-v172] lazy mmap brk munmap mprotect FAIL step=",
        "[ucompat-v172] lazy mmap brk munmap mprotect PASS",
        crate::fs::runtime::run_v172_lazy_mmap_brk_munmap_mprotect,
    );

    // UCOMPAT_V173_V174_CANONICAL_SIGNAL_BATCH.
    run_runtime_check(
        &UCOMPAT_V173_SIGNAL_FRAME_DONE,
        "[ucompat-v173] signal frame rt_sigreturn FAIL step=",
        "[ucompat-v173] signal frame rt_sigreturn PASS",
        crate::fs::runtime::run_v173_signal_frame_rt_sigreturn,
    );
    run_runtime_check(
        &UCOMPAT_V174_SIGCHLD_PGROUP_DONE,
        "[ucompat-v174] sigchld process group signal FAIL step=",
        "[ucompat-v174] sigchld process group signal PASS",
        crate::fs::runtime::run_v174_sigchld_process_group_signal,
    );

    // UCOMPAT_V175_V178_CANONICAL_FS_ENV_BATCH.
    run_runtime_check(
        &UCOMPAT_V175_ROOTFS_TMPFS_DONE,
        "[ucompat-v175] rootfs tmpfs backend FAIL step=",
        "[ucompat-v175] rootfs tmpfs backend PASS",
        crate::fs::runtime::run_v175_rootfs_tmpfs_backend,
    );
    run_runtime_check(
        &UCOMPAT_V176_DEVFS_DONE,
        "[ucompat-v176] devfs core devices FAIL step=",
        "[ucompat-v176] devfs core devices PASS",
        crate::fs::runtime::run_v176_devfs_core_devices,
    );
    run_runtime_check(
        &UCOMPAT_V177_PROCFS_DONE,
        "[ucompat-v177] procfs process status maps FAIL step=",
        "[ucompat-v177] procfs process status maps PASS",
        crate::fs::runtime::run_v177_procfs_process_status_maps,
    );
    run_runtime_check(
        &UCOMPAT_V178_MOUNT_STATFS_DONE,
        "[ucompat-v178] mount tree statfs FAIL step=",
        "[ucompat-v178] mount tree statfs PASS",
        crate::fs::runtime::run_v178_mount_tree_statfs,
    );

    // UCOMPAT_V179_V184_CANONICAL_SECURITY_SOCKET_NAMESPACE_BATCH.
    run_runtime_check(
        &UCOMPAT_V179_PERMISSIONS_DONE,
        "[ucompat-v179] permissions credentials FAIL step=",
        "[ucompat-v179] permissions credentials PASS",
        crate::fs::runtime::run_v179_permissions_credentials,
    );
    run_runtime_check(
        &UCOMPAT_V180_CAPABILITY_DONE,
        "[ucompat-v180] capability identity model FAIL step=",
        "[ucompat-v180] capability identity model PASS",
        crate::fs::runtime::run_v180_capability_identity_model,
    );
    run_runtime_check(
        &UCOMPAT_V181_UNIX_SOCKET_DONE,
        "[ucompat-v181] unix socket loopback FAIL step=",
        "[ucompat-v181] unix socket loopback PASS",
        crate::fs::runtime::run_v181_unix_socket_loopback,
    );
    run_runtime_check(
        &UCOMPAT_V182_DATAGRAM_DONE,
        "[ucompat-v182] local datagram socket FAIL step=",
        "[ucompat-v182] local datagram socket PASS",
        crate::fs::runtime::run_v182_local_datagram_socket,
    );
    run_runtime_check(
        &UCOMPAT_V183_IPC_SCHED_DONE,
        "[ucompat-v183] ipc blocking scheduler integration FAIL step=",
        "[ucompat-v183] ipc blocking scheduler integration PASS",
        crate::fs::runtime::run_v183_ipc_blocking_scheduler_integration,
    );
    run_runtime_check(
        &UCOMPAT_V184_NAMESPACE_DONE,
        "[ucompat-v184] namespace basics FAIL step=",
        "[ucompat-v184] namespace basics PASS",
        crate::fs::runtime::run_v184_namespace_basics,
    );

    // UCOMPAT_V185_V190_FINAL_USERLAND_READINESS_BATCH.
    run_runtime_check(
        &UCOMPAT_V185_MULTI_ELF_DONE,
        "[ucompat-v185] multi elf rootfs runner FAIL step=",
        "[ucompat-v185] multi elf rootfs runner PASS",
        crate::fs::runtime::run_v185_multi_elf_rootfs_runner,
    );
    run_runtime_check(
        &UCOMPAT_V186_LIBC_MATRIX_DONE,
        "[ucompat-v186] libc syscall matrix FAIL step=",
        "[ucompat-v186] libc syscall matrix PASS",
        crate::fs::runtime::run_v186_libc_syscall_matrix,
    );
    run_runtime_check(
        &UCOMPAT_V187_FS_PROCESS_MM_DONE,
        "[ucompat-v187] fs process memory suite FAIL step=",
        "[ucompat-v187] fs process memory suite PASS",
        crate::fs::runtime::run_v187_fs_process_memory_suite,
    );
    run_runtime_check(
        &UCOMPAT_V188_SIGNAL_PIPE_IPC_DONE,
        "[ucompat-v188] signal pipe poll ipc suite FAIL step=",
        "[ucompat-v188] signal pipe poll ipc suite PASS",
        crate::fs::runtime::run_v188_signal_pipe_poll_ipc_suite,
    );
    run_runtime_check(
        &UCOMPAT_V189_STRESS_ERROR_DONE,
        "[ucompat-v189] stress error path hardening FAIL step=",
        "[ucompat-v189] stress error path hardening PASS",
        crate::fs::runtime::run_v189_stress_error_path_hardening,
    );
    run_runtime_check(
        &UCOMPAT_V190_FINAL_READY_DONE,
        "[ucompat-v190] final competition kernel readiness FAIL step=",
        "[ucompat-v190] final competition kernel readiness PASS",
        crate::fs::runtime::run_v190_final_competition_kernel_readiness,
    );

    // UCOMPAT_V157_ACTIVE_ONCE_BRIDGE: one-shot broad-smoke evidence bridge.
    run_active_once(&UCOMPAT_V157_ACTIVE_ONCE_BRIDGE_DONE);
    // UCOMPAT_V155_ACTIVE_ONCE_BRIDGE: preserve v151k7/v154 regression and verify v155 once.
    run_active_once(&UCOMPAT_V155_ACTIVE_ONCE_BRIDGE_DONE);
    // UCOMPAT_V154_ACTIVE_ONCE_BRIDGE: preserve v151k7 regression and verify v154 core once.
    run_active_once(&UCOMPAT_V154_ACTIVE_ONCE_BRIDGE_DONE);
    // UCOMPAT_V153C_ACTIVE_ONCE_BRIDGE: minimal runtime evidence bridge.
    run_active_once(&UCOMPAT_V153C_ACTIVE_ONCE_BRIDGE_DONE);
}
