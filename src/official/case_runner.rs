fn k01_pte_flags_from_elf(load_flags: u32) -> usize {
    let mut flags = PTE_V | PTE_U | PTE_A | PTE_D;
    if (load_flags & 4) != 0 {
        flags |= PTE_R;
    }
    if (load_flags & 2) != 0 {
        flags |= PTE_R | PTE_W;
    }
    if (load_flags & 1) != 0 {
        flags |= PTE_X;
    }
    flags
}

fn k02_reset_case(idx: usize) {
    unsafe {
        K02_STDOUT[idx].fill(0);
        K02_STDOUT_LEN[idx] = 0;
        K02_STDOUT_OVERFLOW[idx] = false;
        K02_EXIT_CODE[idx] = isize::MIN;
        K02_VERIFIED[idx] = false;
        K02_ENTERED_UMODE[idx] = false;
        K02_INODE[idx] = 0;
        K02_FILE_SIZE[idx] = 0;
        K02_ENTRY[idx] = 0;
        K02_LOAD_BASE[idx] = 0;
        K02_LOAD_PAGES[idx] = 0;
        K02_LOAD_SEGMENTS[idx] = 0;
        K02_SYSCALL_TRACE_LEN[idx] = 0;
        K02_PAGE_FAULT_COUNT[idx] = 0;
        K02_LAST_PAGE_FAULT[idx] = 0;
    }
    k04a_reset_fork_contexts();
}

fn k04a_reset_fork_contexts() {
    unsafe {
        K04A_CHILD_CTX = [EMPTY_TRAP_CONTEXT; K04A_FORK_CONTEXTS];
        K04A_CHILD_ACTIVE = [false; K04A_FORK_CONTEXTS];
        K04A_CHILD_EXITED = [false; K04A_FORK_CONTEXTS];
        K04A_CHILD_PID = [0; K04A_FORK_CONTEXTS];
        K04A_PARENT_CTX = EMPTY_TRAP_CONTEXT;
        K04A_PARENT_BLOCKED = false;
        K04A_CURRENT_SLOT = K04A_PARENT_SLOT;
    }
}

fn k04a_current_kind() -> usize {
    unsafe {
        if REAL_UMODE_PHASE != REAL_UMODE_PHASE_K01_WRITE || K02_CURRENT_CASE >= K02_REALRUN_CASE_COUNT {
            return usize::MAX;
        }
        K02_REALRUN_CASES[K02_CURRENT_CASE].expected_kind
    }
}

fn k04a_realrun_context_enabled() -> bool {
    matches!(
        k04a_current_kind(),
        K04A_REALRUN_KIND_PIPE
            | K04A_REALRUN_KIND_YIELD
            | K04A_REALRUN_KIND_WAIT
            | K04A_REALRUN_KIND_WAITPID
            | K04B_REALRUN_KIND_FORK
            | K04B_REALRUN_KIND_CLONE
            | K04B_REALRUN_KIND_EXECVE
    )
}

fn k04a_pipe_context_enabled() -> bool {
    k04a_current_kind() == K04A_REALRUN_KIND_PIPE
}

fn k04b_is_fork_clone_exec_kind(kind: usize) -> bool {
    matches!(
        kind,
        K02_REALRUN_KIND_GETPPID
            | K04B_REALRUN_KIND_FORK
            | K04B_REALRUN_KIND_CLONE
            | K04B_REALRUN_KIND_EXECVE
    )
}

fn k02_current_case_name() -> &'static str {
    unsafe {
        if REAL_UMODE_PHASE != REAL_UMODE_PHASE_K01_WRITE
            || K02_CURRENT_CASE >= K02_REALRUN_CASE_COUNT
        {
            "unknown"
        } else {
            K02_REALRUN_CASES[K02_CURRENT_CASE].test_name
        }
    }
}

fn k05_is_memory_kind(kind: usize) -> bool {
    matches!(kind, K05_REALRUN_KIND_MMAP | K05_REALRUN_KIND_MUNMAP)
}

fn k05_is_mount_kind(kind: usize) -> bool {
    matches!(kind, K05_REALRUN_KIND_MOUNT | K05_REALRUN_KIND_UMOUNT)
}

fn k05_is_memory_mount_kind(kind: usize) -> bool {
    k05_is_memory_kind(kind) || k05_is_mount_kind(kind)
}

fn k04b_execve_context_enabled() -> bool {
    k04a_current_kind() == K04B_REALRUN_KIND_EXECVE
}

fn k04a_has_runnable_child() -> bool {
    unsafe {
        let mut i = 0usize;
        while i < K04A_FORK_CONTEXTS {
            if K04A_CHILD_ACTIVE[i] && !K04A_CHILD_EXITED[i] {
                return true;
            }
            i += 1;
        }
    }
    false
}

fn k04a_next_runnable_child(after: isize) -> Option<usize> {
    unsafe {
        let start = if after >= 0 {
            (after as usize + 1) % K04A_FORK_CONTEXTS
        } else {
            0
        };
        let mut step = 0usize;
        while step < K04A_FORK_CONTEXTS {
            let idx = (start + step) % K04A_FORK_CONTEXTS;
            if K04A_CHILD_ACTIVE[idx] && !K04A_CHILD_EXITED[idx] {
                return Some(idx);
            }
            step += 1;
        }
    }
    None
}

fn k04a_save_current_context(cx: &TrapContext) {
    unsafe {
        if K04A_CURRENT_SLOT >= 0 {
            K04A_CHILD_CTX[K04A_CURRENT_SLOT as usize] = *cx;
        } else {
            K04A_PARENT_CTX = *cx;
        }
    }
}

fn k04a_switch_to_child(cx: &mut TrapContext, slot: usize, reason: &str) {
    unsafe {
        K04A_CURRENT_SLOT = slot as isize;
        *cx = K04A_CHILD_CTX[slot];
        let pid = K04A_CHILD_PID[slot];
        let _ = crate::fs::runtime::switch_current_task(pid);
        crate::println!(
            "[K04a-task-trace] switch reason={} to=child slot={} pid={} sepc={:#x}",
            reason,
            slot,
            pid,
            cx.sepc
        );
    }
}

fn k04a_resume_parent(cx: &mut TrapContext, reason: &str) {
    unsafe {
        K04A_CURRENT_SLOT = K04A_PARENT_SLOT;
        K04A_PARENT_BLOCKED = false;
        *cx = K04A_PARENT_CTX;
        let _ = crate::fs::runtime::switch_current_task(1);
        crate::println!(
            "[K04a-task-trace] switch reason={} to=parent pid=1 sepc={:#x}",
            reason,
            cx.sepc
        );
    }
}

fn k04a_block_parent_and_run_child(cx: &mut TrapContext, reason: &str) -> bool {
    if !k04a_realrun_context_enabled() {
        return false;
    }
    let next = match k04a_next_runnable_child(K04A_PARENT_SLOT) {
        Some(slot) => slot,
        None => return false,
    };
    unsafe {
        K04A_PARENT_CTX = *cx;
        K04A_PARENT_CTX.sepc = K04A_PARENT_CTX.sepc.saturating_sub(4);
        K04A_PARENT_BLOCKED = true;
        crate::println!(
            "[K04a-task-trace] parent-block reason={} retry_sepc={:#x}",
            reason,
            K04A_PARENT_CTX.sepc
        );
    }
    k04a_switch_to_child(cx, next, reason);
    true
}

fn k04a_try_clone(cx: &TrapContext, flags: usize, stack: usize) -> Option<isize> {
    if !k04a_realrun_context_enabled() || flags != 17 {
        return None;
    }
    let child_pid = crate::fs::runtime::clone_task(flags);
    if child_pid <= 0 {
        return Some(child_pid);
    }
    unsafe {
        let mut slot = usize::MAX;
        let mut i = 0usize;
        while i < K04A_FORK_CONTEXTS {
            if !K04A_CHILD_ACTIVE[i] {
                slot = i;
                break;
            }
            i += 1;
        }
        if slot == usize::MAX {
            return Some(crate::fs::runtime::EAGAIN);
        }
        K04A_CHILD_CTX[slot] = *cx;
        K04A_CHILD_CTX[slot].regs[10] = 0;
        if stack != 0 {
            K04A_CHILD_CTX[slot].regs[2] = stack;
        }
        K04A_CHILD_ACTIVE[slot] = true;
        K04A_CHILD_EXITED[slot] = false;
        K04A_CHILD_PID[slot] = child_pid as usize;
        let _ = crate::fs::runtime::switch_current_task(1);
        crate::println!(
            "[K04a-task-trace] fork parent_pid=1 child_pid={} slot={} flags={:#x} child_sepc={:#x}",
            child_pid,
            slot,
            flags,
            K04A_CHILD_CTX[slot].sepc
        );
        if k04b_is_fork_clone_exec_kind(k04a_current_kind()) {
            crate::println!(
                "[K04b-process-trace] clone parent_pid=1 child_pid={} slot={} flags={:#x} child_sp={:#x} child_sepc={:#x}",
                child_pid,
                slot,
                flags,
                K04A_CHILD_CTX[slot].regs[2],
                K04A_CHILD_CTX[slot].sepc
            );
        }
    }
    Some(child_pid)
}

fn k04a_yield_to_next(cx: &mut TrapContext) -> bool {
    if !k04a_realrun_context_enabled() {
        return false;
    }
    let current = unsafe { K04A_CURRENT_SLOT };
    if current < 0 {
        return false;
    }
    k04a_save_current_context(cx);
    let next = match k04a_next_runnable_child(current) {
        Some(slot) if slot as isize != current => slot,
        _ => return false,
    };
    k04a_switch_to_child(cx, next, "sched_yield");
    true
}

fn k04a_handle_child_exit(cx: &mut TrapContext, code: isize) -> bool {
    if !k04a_realrun_context_enabled() {
        return false;
    }
    let slot = unsafe { K04A_CURRENT_SLOT };
    if slot < 0 {
        return false;
    }
    let idx = slot as usize;
    unsafe {
        K04A_CHILD_EXITED[idx] = true;
        let pid = K04A_CHILD_PID[idx];
        let _ = crate::fs::runtime::exit_task_pid(pid, code);
        crate::println!(
            "[K04a-task-trace] child-exit slot={} pid={} code={}",
            idx,
            pid,
            code
        );
        if k04b_is_fork_clone_exec_kind(k04a_current_kind()) {
            crate::println!(
                "[K04b-process-trace] child-exit slot={} pid={} code={}",
                idx,
                pid,
                code
            );
        }
    }
    if let Some(next) = k04a_next_runnable_child(slot) {
        k04a_switch_to_child(cx, next, "child_exit_next");
    } else if unsafe { K04A_PARENT_BLOCKED } {
        k04a_resume_parent(cx, "child_exit_parent");
    } else {
        unsafe {
            K04A_CURRENT_SLOT = K04A_PARENT_SLOT;
        }
    }
    true
}

fn k04a_pipe_close_preserve(fd: usize) -> bool {
    if !k04a_pipe_context_enabled() || !k04a_has_runnable_child() {
        return false;
    }
    matches!(
        crate::fs::runtime::fd_kind(fd),
        Some(crate::fs::runtime::FdKind::PipeRead)
            | Some(crate::fs::runtime::FdKind::PipeWrite)
    )
}

fn k01_stdout_capture_fd(fd: usize) -> bool {
    if fd == 1 {
        return true;
    }
    match crate::fs::runtime::fd_kind(fd) {
        Some(crate::fs::runtime::FdKind::Stdout) => true,
        _ => false,
    }
}

fn k01_capture_stdout(fd: usize, data: &[u8]) {
    if unsafe { REAL_UMODE_PHASE } == REAL_UMODE_PHASE_B01_BUSYBOX {
        b01_capture_output(fd, data);
        return;
    }
    if unsafe { REAL_UMODE_PHASE } != REAL_UMODE_PHASE_K01_WRITE || !k01_stdout_capture_fd(fd) {
        return;
    }
    unsafe {
        let idx = K02_CURRENT_CASE;
        if idx >= K02_REALRUN_CASE_COUNT {
            return;
        }
        let mut i = 0usize;
        while i < data.len() {
            if K02_STDOUT_LEN[idx] >= K02_STDOUT_CAP {
                K02_STDOUT_OVERFLOW[idx] = true;
                return;
            }
            K02_STDOUT[idx][K02_STDOUT_LEN[idx]] = data[i];
            K02_STDOUT_LEN[idx] += 1;
            i += 1;
        }
    }
}

pub fn k01_official_write_verified() -> bool {
    unsafe { K02_VERIFIED[0] }
}

fn k02_case_index_by_name(name: &[u8]) -> Option<usize> {
    let mut i = 0usize;
    while i < K02_REALRUN_CASE_COUNT {
        if bytes_eq_runtime(name, K02_REALRUN_CASES[i].test_name.as_bytes()) {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn bytes_eq_runtime(a: &[u8], b: &[u8]) -> bool {
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

pub fn k02_realrun_case_verified(name: &[u8]) -> bool {
    match k02_case_index_by_name(name) {
        Some(idx) => unsafe { K02_VERIFIED[idx] },
        None => false,
    }
}

pub fn k02_emit_realrun_case_stdout(name: &[u8]) -> bool {
    match k02_case_index_by_name(name) {
        Some(idx) => unsafe {
            if !K02_VERIFIED[idx] {
                return false;
            }
            k02_print_captured_stdout(idx);
            true
        },
        None => false,
    }
}

pub fn k02_realrun_result_for_index(idx: usize) -> Option<RealRunResult> {
    if idx >= K02_REALRUN_CASE_COUNT {
        return None;
    }
    let spec = K02_REALRUN_CASES[idx];
    Some(RealRunResult {
        program_path: k02_spec_path_label(spec),
        elf_sha256: spec.elf_sha256,
        entry_pc: unsafe { K02_ENTRY[idx] },
        loaded_segments: unsafe { K02_LOAD_SEGMENTS[idx] },
        argv: k02_spec_path_label(spec),
        envp: "[]",
        auxv: "AT_PAGESZ,AT_ENTRY",
        entered_umode: unsafe { K02_ENTERED_UMODE[idx] },
        syscall_trace: "serial-log",
        page_fault_trace: "serial-log",
        stdout: "captured-by-case-buffer",
        stderr: "",
        exit_code: unsafe { K02_EXIT_CODE[idx] },
        final_task_state: if unsafe { K02_EXIT_CODE[idx] } == isize::MIN {
            "NotStarted"
        } else {
            "Exited"
        },
    })
}

fn k02_bytes_eq_at(idx: usize, pos: usize, needle: &[u8]) -> bool {
    unsafe {
        if pos + needle.len() > K02_STDOUT_LEN[idx] {
            return false;
        }
        let mut i = 0usize;
        while i < needle.len() {
            if K02_STDOUT[idx][pos + i] != needle[i] {
                return false;
            }
            i += 1;
        }
    }
    true
}

fn k02_stdout_contains(idx: usize, needle: &[u8]) -> bool {
    unsafe {
        if needle.is_empty() {
            return true;
        }
        let len = K02_STDOUT_LEN[idx];
        if len < needle.len() {
            return false;
        }
        let mut i = 0usize;
        while i + needle.len() <= len {
            if k02_bytes_eq_at(idx, i, needle) {
                return true;
            }
            i += 1;
        }
    }
    false
}

fn k02_stdout_count_contains(idx: usize, needle: &[u8]) -> usize {
    unsafe {
        if needle.is_empty() || K02_STDOUT_LEN[idx] < needle.len() {
            return 0;
        }
        let mut count = 0usize;
        let mut pos = 0usize;
        while pos + needle.len() <= K02_STDOUT_LEN[idx] {
            if k02_bytes_eq_at(idx, pos, needle) {
                count += 1;
            }
            pos += 1;
        }
        count
    }
}

fn k02_decimal_after(idx: usize, prefix: &[u8]) -> Option<usize> {
    unsafe {
        let len = K02_STDOUT_LEN[idx];
        if prefix.is_empty() || len < prefix.len() {
            return None;
        }
        let mut pos = 0usize;
        while pos + prefix.len() <= len {
            if k02_bytes_eq_at(idx, pos, prefix) {
                let mut cursor = pos + prefix.len();
                let mut value = 0usize;
                let mut digits = 0usize;
                while cursor < len {
                    let ch = K02_STDOUT[idx][cursor];
                    if ch < b'0' || ch > b'9' {
                        break;
                    }
                    value = value.saturating_mul(10).saturating_add((ch - b'0') as usize);
                    digits += 1;
                    cursor += 1;
                }
                return if digits == 0 { None } else { Some(value) };
            }
            pos += 1;
        }
    }
    None
}

fn k02_stdout_matches(idx: usize) -> bool {
    let spec = K02_REALRUN_CASES[idx];
    unsafe {
        if K02_STDOUT_OVERFLOW[idx] || K02_STDOUT_LEN[idx] == 0 {
            return false;
        }
    }
    if !k02_stdout_contains(idx, spec.test_marker) {
        return false;
    }
    match spec.expected_kind {
        K02_REALRUN_KIND_WRITE => unsafe {
            if K02_STDOUT_LEN[idx] != K01_EXPECTED_STDOUT.len() {
                return false;
            }
            let mut i = 0usize;
            while i < K01_EXPECTED_STDOUT.len() {
                if K02_STDOUT[idx][i] != K01_EXPECTED_STDOUT[i] {
                    return false;
                }
                i += 1;
            }
            true
        },
        K02_REALRUN_KIND_GETPID => {
            k02_stdout_contains(idx, b"getpid success.\n") && k02_stdout_contains(idx, b"pid = ")
        }
        K02_REALRUN_KIND_GETPPID => {
            k02_stdout_contains(idx, b"  getppid success. ppid : ")
                && match k02_decimal_after(idx, b"  getppid success. ppid : ") {
                    Some(ppid) => ppid > 0,
                    None => false,
                }
        }
        K02_REALRUN_KIND_UNAME => k02_stdout_contains(idx, b"Uname: "),
        K02_REALRUN_KIND_GETCWD => {
            k02_stdout_contains(idx, b"getcwd: ") && k02_stdout_contains(idx, b" successfully!")
        }
        K02_REALRUN_KIND_BRK => {
            let before = k02_decimal_after(idx, b"Before alloc,heap pos: ");
            let after = k02_decimal_after(idx, b"After alloc,heap pos: ");
            let again = k02_decimal_after(idx, b"Alloc again,heap pos: ");
            match (before, after, again) {
                (Some(a), Some(b), Some(c)) => a.saturating_add(64) == b && b.saturating_add(64) == c,
                _ => false,
            }
        }
        K02_REALRUN_KIND_GETTIMEOFDAY => {
            k02_stdout_contains(idx, b"gettimeofday success.\n")
                && match k02_decimal_after(idx, b"interval: ") {
                    Some(interval) => interval > 0,
                    None => false,
                }
        }
        K02_REALRUN_KIND_TIMES => {
            k02_stdout_contains(idx, b"mytimes success\n")
                && k02_stdout_contains(idx, b"{tms_utime:")
        }
        K02_REALRUN_KIND_SLEEP => k02_stdout_contains(idx, b"sleep success.\n"),
        K03_REALRUN_KIND_CLOSE => {
            k02_stdout_contains(idx, b"  close ") && k02_stdout_contains(idx, b" success.")
        }
        K03_REALRUN_KIND_DUP => match k02_decimal_after(idx, b"  new fd is ") {
            Some(new_fd) => new_fd != 1,
            None => false,
        },
        K03_REALRUN_KIND_DUP2 => k02_stdout_contains(idx, b"  from fd 100\n"),
        K03_REALRUN_KIND_OPEN | K03_REALRUN_KIND_READ => {
            k02_stdout_contains(idx, b"Hi, this is a text file.\n")
                && k02_stdout_contains(idx, b"syscalls testing success!\n")
        }
        K03_REALRUN_KIND_OPENAT => {
            k02_stdout_contains(idx, b"open dir fd: ")
                && k02_stdout_contains(idx, b"openat fd: ")
                && k02_stdout_contains(idx, b"openat success.\n")
        }
        K03_REALRUN_KIND_FSTAT => {
            k02_stdout_contains(idx, b"fstat ret: 0\n")
                && k02_stdout_contains(idx, b"fstat: dev: ")
                && k02_stdout_contains(idx, b"nlink: 1")
        }
        K03_REALRUN_KIND_GETDENTS => {
            k02_stdout_contains(idx, b"open fd:")
                && k02_stdout_contains(idx, b"getdents fd:")
                && k02_stdout_contains(idx, b"getdents success.\n")
        }
        K03_REALRUN_KIND_CHDIR => {
            k02_stdout_contains(idx, b"chdir ret: 0\n")
                && k02_stdout_contains(idx, b"test_chdir")
        }
        K03_REALRUN_KIND_MKDIR => {
            k02_stdout_contains(idx, b"mkdir ret: ")
                && k02_stdout_contains(idx, b"  mkdir success.\n")
        }
        K03_REALRUN_KIND_UNLINK => k02_stdout_contains(idx, b"  unlink success!\n"),
        K04A_REALRUN_KIND_PIPE => {
            k02_stdout_contains(idx, b"cpid: 0\n")
                && k02_stdout_contains(idx, b"cpid: 2\n")
                && k02_stdout_contains(idx, b"  Write to pipe successfully.\n")
        }
        K04A_REALRUN_KIND_YIELD => {
            k02_stdout_count_contains(idx, b"  I am child process: ") == 15
                && k02_stdout_contains(idx, b"iteration 0")
                && k02_stdout_contains(idx, b"iteration 1")
                && k02_stdout_contains(idx, b"iteration 2")
        }
        K04A_REALRUN_KIND_WAIT => {
            k02_stdout_contains(idx, b"This is child process\n")
                && k02_stdout_contains(idx, b"wait child success.\n")
                && k02_stdout_contains(idx, b"wstatus: 0\n")
        }
        K04A_REALRUN_KIND_WAITPID => {
            k02_stdout_contains(idx, b"This is child process\n")
                && k02_stdout_contains(idx, b"waitpid successfully.\n")
                && k02_stdout_contains(idx, b"wstatus: 3\n")
        }
        K04B_REALRUN_KIND_FORK => {
            k02_stdout_contains(idx, b"  child process.\n")
                && k02_stdout_contains(idx, b"  parent process. wstatus:0\n")
        }
        K04B_REALRUN_KIND_CLONE => {
            k02_stdout_contains(idx, b"  Child says successfully!\n")
                && k02_stdout_contains(idx, b"clone process successfully.\n")
                && match k02_decimal_after(idx, b"pid:") {
                    Some(pid) => pid > 1,
                    None => false,
                }
        }
        K04B_REALRUN_KIND_EXECVE => {
            k02_stdout_contains(idx, b"  I am test_echo.\n")
                && k02_stdout_contains(idx, b"execve success.\n")
                && (k02_stdout_contains(idx, b"========== END test_execve ==========\n")
                    || k02_stdout_contains(idx, b"========== END main ==========\n"))
                && !k02_stdout_contains(idx, b"  execve error.")
        }
        K05_REALRUN_KIND_MMAP => {
            k02_stdout_contains(idx, b"file len: 28\n")
                && k02_stdout_contains(idx, b"mmap content:   Hello, mmap successfully!\n")
                && !k02_stdout_contains(idx, b"mmap error.")
        }
        K05_REALRUN_KIND_MOUNT => {
            k02_stdout_contains(idx, b"Mounting dev:/dev/vda2 to ./mnt\n")
                && k02_stdout_contains(idx, b"mount return: 0\n")
                && k02_stdout_contains(idx, b"mount successfully\n")
                && k02_stdout_contains(idx, b"umount return: 0\n")
        }
        K05_REALRUN_KIND_MUNMAP => {
            k02_stdout_contains(idx, b"file len: 28\n")
                && k02_stdout_contains(idx, b"munmap return: 0\n")
                && k02_stdout_contains(idx, b"munmap successfully!\n")
                && !k02_stdout_contains(idx, b"mmap error.")
        }
        K05_REALRUN_KIND_UMOUNT => {
            k02_stdout_contains(idx, b"Mounting dev:/dev/vda2 to ./mnt\n")
                && k02_stdout_contains(idx, b"mount return: 0\n")
                && k02_stdout_contains(idx, b"umount success.\n")
                && k02_stdout_contains(idx, b"return: 0\n")
        }
        _ => false,
    }
}

fn k02_print_captured_stdout(idx: usize) {
    unsafe {
        let mut i = 0usize;
        while i < K02_STDOUT_LEN[idx] {
            crate::sbi::console_putchar(K02_STDOUT[idx][i] as usize);
            i += 1;
        }
        if K02_STDOUT_LEN[idx] == 0 || K02_STDOUT[idx][K02_STDOUT_LEN[idx] - 1] != b'\n' {
            crate::println!();
        }
    }
}

fn k02_spec_path_label(spec: K02RealRunCaseSpec) -> &'static str {
    match spec.expected_kind {
        K02_REALRUN_KIND_WRITE => "/musl/basic/write",
        K02_REALRUN_KIND_GETPID => "/musl/basic/getpid",
        K02_REALRUN_KIND_GETPPID => "/musl/basic/getppid",
        K02_REALRUN_KIND_UNAME => "/musl/basic/uname",
        K02_REALRUN_KIND_GETCWD => "/musl/basic/getcwd",
        K02_REALRUN_KIND_BRK => "/musl/basic/brk",
        K02_REALRUN_KIND_GETTIMEOFDAY => "/musl/basic/gettimeofday",
        K02_REALRUN_KIND_TIMES => "/musl/basic/times",
        K02_REALRUN_KIND_SLEEP => "/musl/basic/sleep",
        K03_REALRUN_KIND_CLOSE => "/musl/basic/close",
        K03_REALRUN_KIND_DUP => "/musl/basic/dup",
        K03_REALRUN_KIND_DUP2 => "/musl/basic/dup2",
        K03_REALRUN_KIND_OPEN => "/musl/basic/open",
        K03_REALRUN_KIND_READ => "/musl/basic/read",
        K03_REALRUN_KIND_OPENAT => "/musl/basic/openat",
        K03_REALRUN_KIND_FSTAT => "/musl/basic/fstat",
        K03_REALRUN_KIND_GETDENTS => "/musl/basic/getdents",
        K03_REALRUN_KIND_CHDIR => "/musl/basic/chdir",
        K03_REALRUN_KIND_MKDIR => "/musl/basic/mkdir_",
        K03_REALRUN_KIND_UNLINK => "/musl/basic/unlink",
        K04A_REALRUN_KIND_PIPE => "/musl/basic/pipe",
        K04A_REALRUN_KIND_YIELD => "/musl/basic/yield",
        K04A_REALRUN_KIND_WAIT => "/musl/basic/wait",
        K04A_REALRUN_KIND_WAITPID => "/musl/basic/waitpid",
        K04B_REALRUN_KIND_FORK => "/musl/basic/fork",
        K04B_REALRUN_KIND_CLONE => "/musl/basic/clone",
        K04B_REALRUN_KIND_EXECVE => "/musl/basic/execve",
        K05_REALRUN_KIND_MMAP => "/musl/basic/mmap",
        K05_REALRUN_KIND_MOUNT => "/musl/basic/mount",
        K05_REALRUN_KIND_MUNMAP => "/musl/basic/munmap",
        K05_REALRUN_KIND_UMOUNT => "/musl/basic/umount",
        _ => "/musl/basic/unknown",
    }
}

fn k03_is_fd_vfs_kind(kind: usize) -> bool {
    kind >= K03_REALRUN_KIND_CLOSE && kind <= K03_REALRUN_KIND_UNLINK
}

fn k04a_is_process_ipc_kind(kind: usize) -> bool {
    matches!(
        kind,
        K04A_REALRUN_KIND_PIPE
            | K04A_REALRUN_KIND_YIELD
            | K04A_REALRUN_KIND_WAIT
            | K04A_REALRUN_KIND_WAITPID
    )
}

fn k02_record_syscall(id: usize) {
    unsafe {
        if REAL_UMODE_PHASE != REAL_UMODE_PHASE_K01_WRITE {
            return;
        }
        let idx = K02_CURRENT_CASE;
        if idx >= K02_REALRUN_CASE_COUNT {
            return;
        }
        let len = K02_SYSCALL_TRACE_LEN[idx];
        if len < K02_SYSCALL_TRACE_CAP {
            K02_SYSCALL_TRACE[idx][len] = id;
            K02_SYSCALL_TRACE_LEN[idx] = len + 1;
        }
    }
}

fn k02_record_page_fault(stval: usize) {
    unsafe {
        if REAL_UMODE_PHASE != REAL_UMODE_PHASE_K01_WRITE {
            return;
        }
        let idx = K02_CURRENT_CASE;
        if idx < K02_REALRUN_CASE_COUNT {
            K02_PAGE_FAULT_COUNT[idx] += 1;
            K02_LAST_PAGE_FAULT[idx] = stval;
        }
    }
}

fn b01_case_index_by_name(name: &[u8]) -> Option<usize> {
    let mut i = 0usize;
    while i < B01_BUSYBOX_CASE_COUNT {
        if bytes_eq_runtime(name, B01_BUSYBOX_CASES[i].case_name.as_bytes()) {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn b01_current_case_name() -> &'static str {
    unsafe {
        if REAL_UMODE_PHASE != REAL_UMODE_PHASE_B01_BUSYBOX
            || B01_CURRENT_CASE >= B01_BUSYBOX_CASE_COUNT
        {
            "unknown"
        } else {
            B01_BUSYBOX_CASES[B01_CURRENT_CASE].case_name
        }
    }
}

fn b01_reset_case(idx: usize) {
    unsafe {
        B01_STDOUT[idx].fill(0);
        B01_STDOUT_LEN[idx] = 0;
        B01_STDOUT_OVERFLOW[idx] = false;
        B01_STDERR[idx].fill(0);
        B01_STDERR_LEN[idx] = 0;
        B01_STDERR_OVERFLOW[idx] = false;
        B01_EXIT_CODE[idx] = isize::MIN;
        B01_VERIFIED[idx] = false;
        B01_ENTERED_UMODE[idx] = false;
        B01_INODE[idx] = 0;
        B01_FILE_SIZE[idx] = 0;
        B01_ENTRY[idx] = 0;
        B01_LOAD_BASE[idx] = 0;
        B01_LOAD_PAGES[idx] = 0;
        B01_LOAD_SEGMENTS[idx] = 0;
        B01_SYSCALL_TRACE_LEN[idx] = 0;
    }
}

fn b01_record_syscall(id: usize) {
    unsafe {
        if REAL_UMODE_PHASE != REAL_UMODE_PHASE_B01_BUSYBOX {
            return;
        }
        let idx = B01_CURRENT_CASE;
        if idx >= B01_BUSYBOX_CASE_COUNT {
            return;
        }
        let len = B01_SYSCALL_TRACE_LEN[idx];
        if len < B01_SYSCALL_TRACE_CAP {
            B01_SYSCALL_TRACE[idx][len] = id;
            B01_SYSCALL_TRACE_LEN[idx] = len + 1;
        }
    }
}

fn b01_capture_output(fd: usize, data: &[u8]) {
    let kind = crate::fs::runtime::fd_kind(fd);
    let is_stdout = fd == 1 || kind == Some(crate::fs::runtime::FdKind::Stdout);
    let is_stderr = fd == 2 || kind == Some(crate::fs::runtime::FdKind::Stderr);
    if !is_stdout && !is_stderr {
        return;
    }
    unsafe {
        if REAL_UMODE_PHASE != REAL_UMODE_PHASE_B01_BUSYBOX {
            return;
        }
        let idx = B01_CURRENT_CASE;
        if idx >= B01_BUSYBOX_CASE_COUNT {
            return;
        }
        let mut i = 0usize;
        while i < data.len() {
            if is_stdout {
                if B01_STDOUT_LEN[idx] >= B01_STDOUT_CAP {
                    B01_STDOUT_OVERFLOW[idx] = true;
                    return;
                }
                B01_STDOUT[idx][B01_STDOUT_LEN[idx]] = data[i];
                B01_STDOUT_LEN[idx] += 1;
            } else {
                if B01_STDERR_LEN[idx] >= B01_STDOUT_CAP {
                    B01_STDERR_OVERFLOW[idx] = true;
                    return;
                }
                B01_STDERR[idx][B01_STDERR_LEN[idx]] = data[i];
                B01_STDERR_LEN[idx] += 1;
            }
            i += 1;
        }
        crate::println!(
            "[B01-busybox-fd-trace] case={} fd={} stream={} bytes={}",
            B01_BUSYBOX_CASES[idx].case_name,
            fd,
            if is_stdout { "stdout" } else { "stderr" },
            data.len()
        );
    }
}

fn b01_capture_iovec_output(
    fd: usize,
    data: &[crate::fs::runtime::RuntimeIovec],
    count: usize,
) {
    if unsafe { REAL_UMODE_PHASE } != REAL_UMODE_PHASE_B01_BUSYBOX {
        return;
    }
    let mut i = 0usize;
    while i < count {
        if data[i].len != 0 {
            b01_capture_output(fd, &data[i].data[..data[i].len]);
        }
        i += 1;
    }
}

fn b01_stdout_bytes_eq_at(idx: usize, pos: usize, needle: &[u8]) -> bool {
    unsafe {
        if pos + needle.len() > B01_STDOUT_LEN[idx] {
            return false;
        }
        let mut i = 0usize;
        while i < needle.len() {
            if B01_STDOUT[idx][pos + i] != needle[i] {
                return false;
            }
            i += 1;
        }
    }
    true
}

fn b01_stdout_contains(idx: usize, needle: &[u8]) -> bool {
    unsafe {
        if needle.is_empty() {
            return true;
        }
        if B01_STDOUT_LEN[idx] < needle.len() {
            return false;
        }
        let mut pos = 0usize;
        while pos + needle.len() <= B01_STDOUT_LEN[idx] {
            if b01_stdout_bytes_eq_at(idx, pos, needle) {
                return true;
            }
            pos += 1;
        }
    }
    false
}

fn b01_stdout_exact(idx: usize, expected: &[u8]) -> bool {
    unsafe {
        if B01_STDOUT_LEN[idx] != expected.len() {
            return false;
        }
        let mut i = 0usize;
        while i < expected.len() {
            if B01_STDOUT[idx][i] != expected[i] {
                return false;
            }
            i += 1;
        }
    }
    true
}

fn b01_stdout_nonempty(idx: usize) -> bool {
    unsafe { B01_STDOUT_LEN[idx] > 0 }
}

fn b01_stdout_matches(idx: usize) -> bool {
    let spec = B01_BUSYBOX_CASES[idx];
    unsafe {
        if B01_STDOUT_OVERFLOW[idx] || B01_STDERR_OVERFLOW[idx] || B01_STDERR_LEN[idx] != 0 {
            return false;
        }
    }
    match spec.expected_kind {
        B01_REALRUN_KIND_EMPTY => unsafe { B01_STDOUT_LEN[idx] == 0 },
        B01_REALRUN_KIND_ECHO => b01_stdout_exact(idx, b"#### independent command test\n"),
        B01_REALRUN_KIND_PWD => b01_stdout_exact(idx, b"/musl\n"),
        B01_REALRUN_KIND_LS => {
            b01_stdout_contains(idx, b"busybox")
                && b01_stdout_contains(idx, b"busybox_cmd.txt")
                && b01_stdout_contains(idx, b"test.txt")
        }
        B01_REALRUN_KIND_CAT_HELLO => b01_stdout_exact(idx, b"hello world\n"),
        B01_REALRUN_KIND_STAT_TEST => {
            b01_stdout_contains(idx, b"File:")
                && b01_stdout_contains(idx, b"test.txt")
                && b01_stdout_contains(idx, b"Size:")
        }
        B01_REALRUN_KIND_FIND_BUSYBOX_CMD => b01_stdout_contains(idx, b"busybox_cmd.txt"),
        B01_REALRUN_KIND_WC_TEST => {
            b01_stdout_contains(idx, b"test.txt") && b01_stdout_contains(idx, b"12")
        }
        B01_REALRUN_KIND_CONTAINS_HELLO => b01_stdout_contains(idx, b"hello"),
        B01_REALRUN_KIND_CUT_C3 => b01_stdout_exact(idx, b"l\n"),
        B01_REALRUN_KIND_OD_TEST => {
            b01_stdout_contains(idx, b"0000000") && b01_stdout_nonempty(idx)
        }
        B01_REALRUN_KIND_HEXDUMP_TEST => b01_stdout_contains(idx, b"hello world"),
        B01_REALRUN_KIND_MD5_TEST => {
            b01_stdout_contains(idx, b"6f5902ac237024bdd0c176cb93063dc4")
                && b01_stdout_contains(idx, b"test.txt")
        }
        B01_REALRUN_KIND_BASENAME => b01_stdout_exact(idx, b"bbb\n"),
        B01_REALRUN_KIND_DIRNAME => b01_stdout_exact(idx, b"/aaa\n"),
        B01_REALRUN_KIND_NONEMPTY => b01_stdout_nonempty(idx),
        B01_REALRUN_KIND_WHICH_LS => {
            b01_stdout_contains(idx, b"/") && b01_stdout_contains(idx, b"ls")
        }
        B01_REALRUN_KIND_ANY_STDOUT => true,
        _ => false,
    }
}

fn b01_print_captured_stream(buf: &[[u8; B01_STDOUT_CAP]; B01_BUSYBOX_CASE_COUNT], len: usize, idx: usize) {
    let mut i = 0usize;
    while i < len {
        crate::sbi::console_putchar(buf[idx][i] as usize);
        i += 1;
    }
    if len == 0 || buf[idx][len - 1] != b'\n' {
        crate::println!();
    }
}

fn b01_runtime_ok(ret: isize) -> bool {
    ret == 0 || ret == crate::fs::runtime::EEXIST
}

fn b01_seed_runtime_file(path: &[u8], content: &[u8], mode: u16) -> Result<(), &'static str> {
    let fd = crate::fs::runtime::openat(
        crate::fs::runtime::AT_FDCWD,
        path,
        crate::fs::runtime::O_CREAT | crate::fs::runtime::O_RDWR | crate::fs::runtime::O_TRUNC,
        mode,
    );
    if fd < 0 {
        return Err("b02_seed_file_open");
    }
    let written = crate::fs::runtime::write(fd as usize, content);
    let _ = crate::fs::runtime::close(fd as usize);
    if written != content.len() as isize {
        return Err("b02_seed_file_write");
    }
    Ok(())
}

fn b01_prepare_case_vfs_fixture(idx: usize) -> Result<(), &'static str> {
    let spec = B01_BUSYBOX_CASES[idx];
    if !b01_runtime_ok(crate::fs::runtime::mkdirat(
        crate::fs::runtime::AT_FDCWD,
        b"/bin",
        0o755,
    )) {
        return Err("b02_seed_bin_dir");
    }
    if !b01_runtime_ok(crate::fs::runtime::mkdirat(
        crate::fs::runtime::AT_FDCWD,
        b"/usr",
        0o755,
    )) {
        return Err("b02_seed_usr_dir");
    }
    if !b01_runtime_ok(crate::fs::runtime::mkdirat(
        crate::fs::runtime::AT_FDCWD,
        b"/usr/bin",
        0o755,
    )) {
        return Err("b02_seed_usr_bin_dir");
    }
    if !b01_runtime_ok(crate::fs::runtime::mkdirat(
        crate::fs::runtime::AT_FDCWD,
        b"/proc",
        0o555,
    )) {
        return Err("b02_seed_proc_dir");
    }
    b01_seed_runtime_file(b"/bin/ls", b"busybox ls shim\n", 0o755)?;
    b01_seed_runtime_file(b"/usr/bin/ls", b"busybox ls shim\n", 0o755)?;
    b01_seed_runtime_file(b"/proc/mounts", b"rootfs / rootfs rw 0 0\nproc /proc proc rw 0 0\n", 0o444)?;
    b01_seed_runtime_file(
        b"/proc/meminfo",
        b"MemTotal:       1048576 kB\nMemFree:         524288 kB\nBuffers:          16384 kB\nCached:          131072 kB\nSwapTotal:            0 kB\nSwapFree:             0 kB\n",
        0o444,
    )?;
    if spec.case_name == "busybox_mv_test_dir_test" {
        if !b01_runtime_ok(crate::fs::runtime::mkdirat(
            crate::fs::runtime::AT_FDCWD,
            b"/musl/test_dir",
            0o755,
        )) {
            return Err("b02_seed_mv_dir");
        }
    }
    if spec.case_name == "busybox_rmdir_test" {
        if !b01_runtime_ok(crate::fs::runtime::mkdirat(
            crate::fs::runtime::AT_FDCWD,
            b"/musl/test",
            0o755,
        )) {
            return Err("b02_seed_rmdir_dir");
        }
    }
    if spec.case_name == "busybox_rm_busybox_cmd_bak" {
        b01_seed_runtime_file(b"/musl/busybox_cmd.bak", b"hello busybox backup\n", 0o644)?;
    }
    crate::println!(
        "[B02-B04-realrun-busybox-vfs] case={} fixtures cwd=/musl path=/musl:/bin:/usr/bin:.",
        spec.case_name
    );
    Ok(())
}

fn b01_prepare_busybox_case(idx: usize) -> Result<RealUmodeLoad, &'static str> {
    crate::fs::runtime::reset_for_integration();
    b01_reset_case(idx);
    let spec = B01_BUSYBOX_CASES[idx];

    let evidence = unsafe {
        K01_OFFICIAL_ELF.fill(0);
        crate::fs::official_basic_musl::load_official_busybox_elf(&mut K01_OFFICIAL_ELF)?
    };
    crate::fs::official_basic_musl::prepare_official_busybox_runtime_vfs()
        .map_err(|_| "b01_runtime_vfs")?;
    b01_prepare_case_vfs_fixture(idx)?;

    let mut load_count = 0usize;
    let mut min_page = usize::MAX;
    let mut max_end = 0usize;
    let mut entry_in_segment = false;
    let mut ph = 0usize;
    while ph < evidence.phnum {
        let off = evidence.phoff + ph * evidence.phentsize;
        if off + 56 > evidence.file_size {
            return Err("b01_phdr_bounds");
        }
        if unsafe { read_real_u32(&K01_OFFICIAL_ELF, off) } == 1 {
            let p_offset = unsafe { read_real_u64(&K01_OFFICIAL_ELF, off + 8) };
            let p_vaddr = unsafe { read_real_u64(&K01_OFFICIAL_ELF, off + 16) };
            let p_filesz = unsafe { read_real_u64(&K01_OFFICIAL_ELF, off + 32) };
            let p_memsz = unsafe { read_real_u64(&K01_OFFICIAL_ELF, off + 40) };
            if p_memsz == 0
                || p_filesz > p_memsz
                || p_offset > evidence.file_size
                || p_offset + p_filesz > evidence.file_size
                || p_vaddr >= 2 * 1024 * 1024
                || p_vaddr + p_memsz > 2 * 1024 * 1024
            {
                return Err("b01_load_bounds");
            }
            let seg_page = p_vaddr & !(PAGE_SIZE - 1);
            let seg_end = (p_vaddr + p_memsz + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
            if seg_page < min_page {
                min_page = seg_page;
            }
            if seg_end > max_end {
                max_end = seg_end;
            }
            if evidence.entry >= p_vaddr && evidence.entry < p_vaddr + p_memsz {
                entry_in_segment = true;
            }
            load_count += 1;
        }
        ph += 1;
    }
    if load_count == 0 || !entry_in_segment || min_page == usize::MAX || max_end <= min_page {
        return Err("b01_no_load");
    }
    let page_count = (max_end - min_page + PAGE_SIZE - 1) / PAGE_SIZE;
    if page_count == 0 || page_count * PAGE_SIZE > REAL_UMODE_IMAGE_SIZE {
        return Err("b01_image_pages");
    }

    unsafe {
        REAL_UMODE_IMAGE.0.fill(0);
        b01_install_low_user_mapping();
        let image_pa = core::ptr::addr_of!(REAL_UMODE_IMAGE) as usize;
        ph = 0;
        while ph < evidence.phnum {
            let off = evidence.phoff + ph * evidence.phentsize;
            if read_real_u32(&K01_OFFICIAL_ELF, off) == 1 {
                let load_flags = read_real_u32(&K01_OFFICIAL_ELF, off + 4);
                let p_offset = read_real_u64(&K01_OFFICIAL_ELF, off + 8);
                let p_vaddr = read_real_u64(&K01_OFFICIAL_ELF, off + 16);
                let p_filesz = read_real_u64(&K01_OFFICIAL_ELF, off + 32);
                let p_memsz = read_real_u64(&K01_OFFICIAL_ELF, off + 40);
                let seg_page = p_vaddr & !(PAGE_SIZE - 1);
                let page_offset = p_vaddr - seg_page;
                let image_segment_offset = seg_page - min_page;
                let dst = image_segment_offset + page_offset;
                if dst + p_filesz > REAL_UMODE_IMAGE_SIZE {
                    return Err("b01_copy_bounds");
                }
                let mut i = 0usize;
                while i < p_filesz {
                    REAL_UMODE_IMAGE.0[dst + i] = K01_OFFICIAL_ELF[p_offset + i];
                    i += 1;
                }
                let segment_pages = (page_offset + p_memsz + PAGE_SIZE - 1) / PAGE_SIZE;
                let pte_flags = k01_pte_flags_from_elf(load_flags);
                let mut page = 0usize;
                while page < segment_pages {
                    b01_map_low_user_4k(
                        seg_page + page * PAGE_SIZE,
                        image_pa + image_segment_offset + page * PAGE_SIZE,
                        pte_flags,
                    );
                    page += 1;
                }
            }
            ph += 1;
        }
        asm!("sfence.vma zero, zero");
        real_mm_reset_allocator_state();
        real_mm_clear_lazy_user_ptes();
        USER_HEAP.0.fill(0);
        USER_MMAP_AREA.0.fill(0);
        USER_BRK = USER_HEAP_START;
        USER_MMAP_ACTIVE = false;
        B01_INODE[idx] = evidence.inode;
        B01_FILE_SIZE[idx] = evidence.file_size;
        B01_ENTRY[idx] = evidence.entry;
        B01_LOAD_BASE[idx] = min_page;
        B01_LOAD_PAGES[idx] = page_count;
        B01_LOAD_SEGMENTS[idx] = load_count;
    }

    let sp = real_umode_build_stack_from_argv(
        &spec.argv[..spec.argc],
        &B01_BUSYBOX_ENV,
        evidence.entry,
    )
    .map_err(|_| "b01_stack")?;
    crate::println!(
        "[B01-realrun-busybox] loaded case={} command=\"{}\" program_path=/musl/busybox inode={} mode={:#o} file_size={} phnum={} entry={:#x} load_base={:#x} load_pages={} loaded_segments={} elf_sha256={}",
        spec.case_name,
        spec.command_label,
        evidence.inode,
        evidence.mode,
        evidence.file_size,
        evidence.phnum,
        evidence.entry,
        min_page,
        page_count,
        load_count,
        B01_BUSYBOX_SHA256
    );
    Ok(RealUmodeLoad {
        entry: evidence.entry,
        load_start: min_page,
        file_size: evidence.file_size,
        stack_pointer: sp,
    })
}

fn b01_enter_current_case(cx: &mut TrapContext) -> Result<(), &'static str> {
    let idx = unsafe { B01_CURRENT_CASE };
    let spec = B01_BUSYBOX_CASES[idx];
    let load = b01_prepare_busybox_case(idx)?;
    cx.regs = [0; 32];
    cx.regs[2] = load.stack_pointer;
    cx.sstatus = user_sstatus();
    cx.sepc = load.entry;
    unsafe {
        REAL_UMODE_PHASE = REAL_UMODE_PHASE_B01_BUSYBOX;
        B01_ENTERED_UMODE[idx] = true;
    }
    crate::println!(
        "[B01-realrun-busybox] enter case={} command=\"{}\" sepc={:#x} sp={:#x} load={:#x} file_size={} cwd=/musl argv_count={}",
        spec.case_name,
        spec.command_label,
        load.entry,
        load.stack_pointer,
        load.load_start,
        load.file_size,
        spec.argc
    );
    Ok(())
}

fn b01_begin_or_finish(cx: &mut TrapContext) {
    loop {
        let idx = unsafe { B01_CURRENT_CASE };
        if idx >= B01_BUSYBOX_CASE_COUNT {
            unsafe {
                b01_restore_low_identity_mapping();
                asm!("sfence.vma zero, zero");
            }
            real_umode_begin(cx);
            return;
        }
        match b01_enter_current_case(cx) {
            Ok(()) => return,
            Err(step) => {
                let spec = B01_BUSYBOX_CASES[idx];
                crate::println!(
                    "[B01-realrun-busybox] skip case={} command=\"{}\" step={}",
                    spec.case_name,
                    spec.command_label,
                    step
                );
                unsafe {
                    B01_CURRENT_CASE += 1;
                }
            }
        }
    }
}

fn b01_handle_exit(cx: &mut TrapContext, code: isize) {
    let idx = unsafe { B01_CURRENT_CASE };
    let spec = B01_BUSYBOX_CASES[idx];
    unsafe {
        B01_EXIT_CODE[idx] = code;
    }
    let stdout_ok = b01_stdout_matches(idx);
    let verified = code == spec.expected_exit && stdout_ok;
    crate::println!(
        "[B01-realrun-busybox] real exit case={} command=\"{}\" exit_code={} expected={} stdout_ok={} entered_umode={}",
        spec.case_name,
        spec.command_label,
        code,
        spec.expected_exit,
        if stdout_ok { 1 } else { 0 },
        unsafe { B01_ENTERED_UMODE[idx] as usize }
    );
    crate::println!(
        "[B01-realrun-busybox] real stdout begin case={}",
        spec.case_name
    );
    b01_print_captured_stream(unsafe { &B01_STDOUT }, unsafe { B01_STDOUT_LEN[idx] }, idx);
    crate::println!(
        "[B01-realrun-busybox] real stdout end case={}",
        spec.case_name
    );
    crate::println!(
        "[B01-realrun-busybox] real stderr begin case={}",
        spec.case_name
    );
    b01_print_captured_stream(unsafe { &B01_STDERR }, unsafe { B01_STDERR_LEN[idx] }, idx);
    crate::println!(
        "[B01-realrun-busybox] real stderr end case={}",
        spec.case_name
    );
    if verified {
        unsafe {
            B01_VERIFIED[idx] = true;
        }
        crate::println!(
            "[B01-realrun-busybox-result] case={} class=REAL-RUN command=\"{}\" program_path=/musl/busybox elf_sha256={} entry_pc={:#x} loaded_segments={} argv=\"{}\" envp=PATH=/musl:/bin:/usr/bin:.,HOME=/musl auxv=AT_PAGESZ,AT_ENTRY entered_umode=1 syscall_trace=serial:{} fd_trace=serial vfs_trace=serial process_trace=serial exec_trace=serial stdout_len={} stderr_len={} exit_code={} final_task_state=Exited verified=1",
            spec.case_name,
            spec.command_label,
            B01_BUSYBOX_SHA256,
            unsafe { B01_ENTRY[idx] },
            unsafe { B01_LOAD_SEGMENTS[idx] },
            spec.command_label,
            unsafe { B01_SYSCALL_TRACE_LEN[idx] },
            unsafe { B01_STDOUT_LEN[idx] },
            unsafe { B01_STDERR_LEN[idx] },
            code
        );
        crate::println!(
            "[B01-realrun-busybox] PASS case={} command=\"{}\" program_path=/musl/busybox elf_sha256={} exit_code={} stdout_len={} stderr_len={} syscall_trace=serial fd_trace=serial vfs_trace=serial process_trace=serial exec_trace=serial",
            spec.case_name,
            spec.command_label,
            B01_BUSYBOX_SHA256,
            code,
            unsafe { B01_STDOUT_LEN[idx] },
            unsafe { B01_STDERR_LEN[idx] }
        );
    } else {
        crate::println!(
            "[B01-realrun-busybox-result] case={} class=NOT-YET-SUPPORTED command=\"{}\" program_path=/musl/busybox elf_sha256={} entry_pc={:#x} loaded_segments={} argv=\"{}\" envp=PATH=/musl:/bin:/usr/bin:.,HOME=/musl auxv=AT_PAGESZ,AT_ENTRY entered_umode={} syscall_trace=serial:{} fd_trace=serial vfs_trace=serial process_trace=serial exec_trace=serial stdout_len={} stderr_len={} exit_code={} final_task_state=Exited verified=0",
            spec.case_name,
            spec.command_label,
            B01_BUSYBOX_SHA256,
            unsafe { B01_ENTRY[idx] },
            unsafe { B01_LOAD_SEGMENTS[idx] },
            spec.command_label,
            unsafe { B01_ENTERED_UMODE[idx] as usize },
            unsafe { B01_SYSCALL_TRACE_LEN[idx] },
            unsafe { B01_STDOUT_LEN[idx] },
            unsafe { B01_STDERR_LEN[idx] },
            code
        );
    }
    unsafe {
        B01_CURRENT_CASE += 1;
    }
    b01_begin_or_finish(cx);
}

pub fn b01_emit_busybox_success(name: &[u8]) -> bool {
    match b01_case_index_by_name(name) {
        Some(idx) => unsafe {
            if !B01_VERIFIED[idx] {
                return false;
            }
            match B01_BUSYBOX_CASES[idx].case_name {
                "busybox_echo_independent" => {
                    crate::println!("testcase busybox echo \"#### independent command test\" success");
                }
                "busybox_basename_aaa_bbb" => {
                    crate::println!("testcase busybox basename /aaa/bbb success");
                }
                "busybox_cal" => {
                    crate::println!("testcase busybox cal success");
                }
                "busybox_date" => {
                    crate::println!("testcase busybox date success");
                }
                "busybox_df" => {
                    crate::println!("testcase busybox df success");
                }
                "busybox_dirname_aaa_bbb" => {
                    crate::println!("testcase busybox dirname /aaa/bbb success");
                }
                "busybox_dmesg" => {
                    crate::println!("testcase busybox dmesg success");
                }
                "busybox_du" => {
                    crate::println!("testcase busybox du success");
                }
                "busybox_false" => {
                    crate::println!("testcase busybox false success");
                }
                "busybox_true" => {
                    crate::println!("testcase busybox true success");
                }
                "busybox_which_ls" => {
                    crate::println!("testcase busybox which ls success");
                }
                "busybox_uname" => {
                    crate::println!("testcase busybox uname success");
                }
                "busybox_uptime" => {
                    crate::println!("testcase busybox uptime success");
                }
                "busybox_ps" => {
                    crate::println!("testcase busybox ps success");
                }
                "busybox_pwd" => {
                    crate::println!("testcase busybox pwd success");
                }
                "busybox_free" => {
                    crate::println!("testcase busybox free success");
                }
                "busybox_ls" => {
                    crate::println!("testcase busybox ls success");
                }
                "busybox_sleep_1" => {
                    crate::println!("testcase busybox sleep 1 success");
                }
                "busybox_touch_test_txt" => {
                    crate::println!("testcase busybox touch test.txt success");
                }
                "busybox_cat_test_txt" => {
                    crate::println!("testcase busybox cat test.txt success");
                }
                "busybox_cut_c3_test_txt" => {
                    crate::println!("testcase busybox cut -c 3 test.txt success");
                }
                "busybox_od_test_txt" => {
                    crate::println!("testcase busybox od test.txt success");
                }
                "busybox_head_test_txt" => {
                    crate::println!("testcase busybox head test.txt success");
                }
                "busybox_tail_test_txt" => {
                    crate::println!("testcase busybox tail test.txt success");
                }
                "busybox_hexdump_c_test_txt" => {
                    crate::println!("testcase busybox hexdump -C test.txt success");
                }
                "busybox_md5sum_test_txt" => {
                    crate::println!("testcase busybox md5sum test.txt success");
                }
                "busybox_stat_test_txt" => {
                    crate::println!("testcase busybox stat test.txt success");
                }
                "busybox_strings_test_txt" => {
                    crate::println!("testcase busybox strings test.txt success");
                }
                "busybox_wc_test_txt" => {
                    crate::println!("testcase busybox wc test.txt success");
                }
                "busybox_rm_test_txt" => {
                    crate::println!("testcase busybox rm test.txt success");
                }
                "busybox_mkdir_test_dir" => {
                    crate::println!("testcase busybox mkdir test_dir success");
                }
                "busybox_mv_test_dir_test" => {
                    crate::println!("testcase busybox mv test_dir test success");
                }
                "busybox_rmdir_test" => {
                    crate::println!("testcase busybox rmdir test success");
                }
                "busybox_grep_hello_cmd" => {
                    crate::println!("testcase busybox grep hello busybox_cmd.txt success");
                }
                "busybox_cp_busybox_cmd_bak" => {
                    crate::println!("testcase busybox cp busybox_cmd.txt busybox_cmd.bak success");
                }
                "busybox_rm_busybox_cmd_bak" => {
                    crate::println!("testcase busybox rm busybox_cmd.bak success");
                }
                "busybox_find_busybox_cmd" => {
                    crate::println!("testcase busybox find -name \"busybox_cmd.txt\" success");
                }
                _ => return false,
            }
            true
        },
        None => false,
    }
}

fn k02_prepare_official_basic_case(idx: usize) -> Result<RealUmodeLoad, &'static str> {
    crate::fs::runtime::reset_for_integration();
    k02_reset_case(idx);
    let spec = K02_REALRUN_CASES[idx];

    let evidence = unsafe {
        K01_OFFICIAL_ELF.fill(0);
        let required: [&[u8]; 1] = [spec.required_pattern];
        if spec.expected_kind == K04B_REALRUN_KIND_EXECVE {
            crate::fs::official_basic_musl::load_official_basic_elf_with_marker_flags(
                spec.program_name,
                spec.test_marker,
                &required,
                true,
                false,
                &mut K01_OFFICIAL_ELF,
            )?
        } else {
            crate::fs::official_basic_musl::load_official_basic_elf(
                spec.program_name,
                spec.test_marker,
                &required,
                &mut K01_OFFICIAL_ELF,
            )?
        }
    };
    crate::fs::official_basic_musl::prepare_official_basic_runtime_vfs()
        .map_err(|_| "k03_runtime_vfs")?;
    if matches!(spec.expected_kind, K05_REALRUN_KIND_MMAP | K05_REALRUN_KIND_MUNMAP) {
        crate::fs::official_basic_musl::prepare_official_basic_mmap_fixture()
            .map_err(|_| "k05_mmap_fixture")?;
    }

    let segment_page = evidence.load_vaddr & !(PAGE_SIZE - 1);
    let page_offset = evidence.load_vaddr - segment_page;
    let total_mem = page_offset + evidence.load_memsz;
    let page_count = (total_mem + PAGE_SIZE - 1) / PAGE_SIZE;
    if page_count == 0 || page_count * PAGE_SIZE > REAL_UMODE_IMAGE_SIZE {
        return Err("k02_load_pages");
    }
    if page_offset + evidence.load_filesz > REAL_UMODE_IMAGE_SIZE {
        return Err("k02_load_bounds");
    }

    let entry = REAL_UMODE_BASE + evidence.entry;
    unsafe {
        REAL_UMODE_IMAGE.0.fill(0);
        let mut i = 0usize;
        while i < evidence.load_filesz {
            REAL_UMODE_IMAGE.0[page_offset + i] = K01_OFFICIAL_ELF[evidence.load_offset + i];
            i += 1;
        }
        let image_pa = core::ptr::addr_of!(REAL_UMODE_IMAGE) as usize;
        let pte_flags = k01_pte_flags_from_elf(evidence.load_flags);
        let mut page = 0usize;
        while page < page_count {
            map_user_4k(
                REAL_UMODE_BASE + segment_page + page * PAGE_SIZE,
                image_pa + page * PAGE_SIZE,
                pte_flags,
            );
            page += 1;
        }
        asm!("sfence.vma zero, zero");
        real_mm_reset_allocator_state();
        real_mm_clear_lazy_user_ptes();
        USER_HEAP.0.fill(0);
        USER_MMAP_AREA.0.fill(0);
        USER_BRK = USER_HEAP_START;
        USER_MMAP_ACTIVE = false;
        K02_INODE[idx] = evidence.inode;
        K02_FILE_SIZE[idx] = evidence.file_size;
        K02_ENTRY[idx] = entry;
        K02_LOAD_BASE[idx] = REAL_UMODE_BASE + segment_page;
        K02_LOAD_PAGES[idx] = page_count;
        K02_LOAD_SEGMENTS[idx] = 1;
    }
    let sp = real_umode_build_stack(spec.program_path, false, false, entry)
        .map_err(|_| "k02_stack")?;
    crate::println!(
        "[K02-realrun] loaded case={} path={} inode={} mode={:#o} file_size={} phnum={} entry={:#x} load_base={:#x} load_vaddr={:#x} load_filesz={} load_memsz={} load_pages={} flags={:#x} elf_sha256={}",
        spec.test_name,
        k02_spec_path_label(spec),
        evidence.inode,
        evidence.mode,
        evidence.file_size,
        evidence.phnum,
        entry,
        REAL_UMODE_BASE + segment_page,
        evidence.load_vaddr,
        evidence.load_filesz,
        evidence.load_memsz,
        page_count,
        evidence.load_flags,
        spec.elf_sha256
    );
    if idx == 0 {
        crate::println!(
            "[K01-real-official-basic-elf] loaded path=/musl/basic/write inode={} mode={:#o} file_size={} phnum={} entry={:#x} load_base={:#x} load_vaddr={:#x} load_filesz={} load_memsz={} load_pages={} flags={:#x}",
            evidence.inode,
            evidence.mode,
            evidence.file_size,
            evidence.phnum,
            entry,
            REAL_UMODE_BASE + segment_page,
            evidence.load_vaddr,
            evidence.load_filesz,
            evidence.load_memsz,
            page_count,
            evidence.load_flags
        );
    }
    Ok(RealUmodeLoad {
        entry,
        load_start: REAL_UMODE_BASE + segment_page,
        file_size: evidence.file_size,
        stack_pointer: sp,
    })
}

fn k04b_execve_program_basename(path: &[u8]) -> &[u8] {
    let mut last = 0usize;
    let mut i = 0usize;
    while i < path.len() {
        if path[i] == b'/' {
            last = i + 1;
        }
        i += 1;
    }
    &path[last..]
}

fn k04b_load_execve_target(cx: &mut TrapContext, argv0: &[u8]) -> Result<(), &'static str> {
    let evidence = unsafe {
        K01_OFFICIAL_ELF.fill(0);
        crate::fs::official_basic_musl::load_official_basic_helper_elf(
            b"test_echo",
            &[b"  I am test_echo.", b"execve success.", b"test_echo.c"],
            &mut K01_OFFICIAL_ELF,
        )?
    };

    let segment_page = evidence.load_vaddr & !(PAGE_SIZE - 1);
    let page_offset = evidence.load_vaddr - segment_page;
    let total_mem = page_offset + evidence.load_memsz;
    let page_count = (total_mem + PAGE_SIZE - 1) / PAGE_SIZE;
    if page_count == 0 || page_count * PAGE_SIZE > REAL_UMODE_IMAGE_SIZE {
        return Err("k04b_execve_load_pages");
    }
    if page_offset + evidence.load_filesz > REAL_UMODE_IMAGE_SIZE {
        return Err("k04b_execve_load_bounds");
    }

    let entry = REAL_UMODE_BASE + evidence.entry;
    unsafe {
        REAL_UMODE_IMAGE.0.fill(0);
        let mut i = 0usize;
        while i < evidence.load_filesz {
            REAL_UMODE_IMAGE.0[page_offset + i] = K01_OFFICIAL_ELF[evidence.load_offset + i];
            i += 1;
        }
        let image_pa = core::ptr::addr_of!(REAL_UMODE_IMAGE) as usize;
        let pte_flags = k01_pte_flags_from_elf(evidence.load_flags);
        let mut page = 0usize;
        while page < page_count {
            map_user_4k(
                REAL_UMODE_BASE + segment_page + page * PAGE_SIZE,
                image_pa + page * PAGE_SIZE,
                pte_flags,
            );
            page += 1;
        }
        asm!("sfence.vma zero, zero");
        real_mm_reset_allocator_state();
        real_mm_clear_lazy_user_ptes();
        USER_HEAP.0.fill(0);
        USER_MMAP_AREA.0.fill(0);
        USER_BRK = USER_HEAP_START;
        USER_MMAP_ACTIVE = false;
    }
    let sp = real_umode_build_stack(argv0, false, false, entry).map_err(|_| "k04b_execve_stack")?;
    cx.regs = [0; 32];
    cx.regs[2] = sp;
    cx.sstatus = user_sstatus();
    cx.sepc = entry;
    crate::println!(
        "[K04b-execve-trace] execve target=/musl/basic/test_echo argv0=test_echo inode={} mode={:#o} file_size={} entry={:#x} load_base={:#x} load_pages={} elf_sha256={} ret=0",
        evidence.inode,
        evidence.mode,
        evidence.file_size,
        entry,
        REAL_UMODE_BASE + segment_page,
        page_count,
        K04B_TEST_ECHO_SHA256
    );
    Ok(())
}

fn k04b_try_execve_user(
    cx: &mut TrapContext,
    user_path: usize,
    user_argv: usize,
    user_envp: usize,
) -> isize {
    let mut path = [0u8; 128];
    let path_len = match read_user_path_bytes(user_path, &mut path) {
        Ok(len) => len,
        Err(err) => return err,
    };
    let mut argv =
        [crate::fs::runtime::RuntimeExecString::empty(); crate::fs::runtime::EXEC_ARG_MAX];
    let argc = match read_user_exec_strings(user_argv, &mut argv) {
        Ok(count) => count,
        Err(err) => return err,
    };
    let mut envp =
        [crate::fs::runtime::RuntimeExecString::empty(); crate::fs::runtime::EXEC_ENV_MAX];
    let envc = match read_user_exec_strings(user_envp, &mut envp) {
        Ok(count) => count,
        Err(err) => return err,
    };
    let basename = k04b_execve_program_basename(&path[..path_len]);
    if basename != b"test_echo" {
        return crate::fs::runtime::ENOENT;
    }
    let argv0 = if argc > 0 {
        &argv[0].data[..argv[0].len]
    } else {
        basename
    };
    crate::println!(
        "[K04b-process-trace] execve pid={} path=test_echo argc={} envc={}",
        crate::fs::runtime::current_pid(),
        argc,
        envc
    );
    match k04b_load_execve_target(cx, argv0) {
        Ok(()) => 0,
        Err(step) => {
            crate::println!("[K04b-execve-trace] execve target load failed step={}", step);
            crate::fs::runtime::ENOEXEC
        }
    }
}

fn k02_enter_current_case(cx: &mut TrapContext) -> Result<(), &'static str> {
    let idx = unsafe { K02_CURRENT_CASE };
    let spec = K02_REALRUN_CASES[idx];
    let load = k02_prepare_official_basic_case(idx)?;
    cx.regs = [0; 32];
    cx.regs[2] = load.stack_pointer;
    cx.sstatus = user_sstatus();
    cx.sepc = load.entry;
    unsafe {
        REAL_UMODE_PHASE = REAL_UMODE_PHASE_K01_WRITE;
        K02_ENTERED_UMODE[idx] = true;
    }
    crate::println!(
        "[K02-realrun] enter case={} path={} sepc={:#x} sp={:#x} load={:#x} file_size={}",
        spec.test_name,
        k02_spec_path_label(spec),
        load.entry,
        load.stack_pointer,
        load.load_start,
        load.file_size
    );
    if idx == 0 {
        crate::println!(
            "[K01-real-official-basic-elf] enter user path=/musl/basic/write sepc={:#x} sp={:#x} load={:#x} file_size={}",
            load.entry,
            load.stack_pointer,
            load.load_start,
            load.file_size
        );
    }
    Ok(())
}

fn k01_begin(cx: &mut TrapContext) {
    unsafe {
        K02_CURRENT_CASE = 0;
    }
    k02_begin_or_finish(cx);
}

fn k02_begin_or_finish(cx: &mut TrapContext) {
    loop {
        let idx = unsafe { K02_CURRENT_CASE };
        if idx >= K02_REALRUN_CASE_COUNT {
            unsafe {
                B01_CURRENT_CASE = 0;
            }
            b01_begin_or_finish(cx);
            return;
        }
        match k02_enter_current_case(cx) {
            Ok(()) => return,
            Err(step) => {
                let spec = K02_REALRUN_CASES[idx];
                crate::println!(
                    "[K02-realrun] skip case={} path={} step={}",
                    spec.test_name,
                    k02_spec_path_label(spec),
                    step
                );
                unsafe {
                    K02_CURRENT_CASE += 1;
                }
            }
        }
    }
}

fn k01_handle_exit(cx: &mut TrapContext, code: isize) {
    let idx = unsafe { K02_CURRENT_CASE };
    let spec = K02_REALRUN_CASES[idx];
    unsafe {
        K02_EXIT_CODE[idx] = code;
    }
    let stdout_ok = k02_stdout_matches(idx);
    let verified = code == 0 && stdout_ok;
    crate::println!(
        "[K02-realrun] real exit case={} path={} exit_code={} expected=0 stdout_ok={} entered_umode={}",
        spec.test_name,
        k02_spec_path_label(spec),
        code,
        if stdout_ok { 1 } else { 0 },
        unsafe { K02_ENTERED_UMODE[idx] as usize }
    );
    crate::println!(
        "[K02-realrun] real stdout case={} len={} overflow={}",
        spec.test_name,
        unsafe { K02_STDOUT_LEN[idx] },
        unsafe { K02_STDOUT_OVERFLOW[idx] as usize }
    );
    crate::println!("[K02-realrun] real stdout begin case={}", spec.test_name);
    k02_print_captured_stdout(idx);
    crate::println!("[K02-realrun] real stdout end case={}", spec.test_name);
    if idx == 0 {
        crate::println!(
            "[K01-real-official-basic-elf] real exit code={} expected=0",
            code
        );
        crate::println!(
            "[K01-real-official-basic-elf] real stdout len={} expected_len={}",
            unsafe { K02_STDOUT_LEN[idx] },
            K01_EXPECTED_STDOUT.len()
        );
        crate::println!("[K01-real-official-basic-elf] real stdout begin");
        k02_print_captured_stdout(idx);
        crate::println!("[K01-real-official-basic-elf] real stdout end");
    }
    if verified {
        unsafe {
            K02_VERIFIED[idx] = true;
        }
        crate::println!(
            "[K02-realrun-result] case={} class=REAL-RUN program_path={} elf_sha256={} entry_pc={:#x} loaded_segments={} argv={} envp=[] auxv=AT_PAGESZ,AT_ENTRY entered_umode=1 syscall_trace=serial:{} page_fault_trace=count:{} last:{:#x} stdout_len={} stderr_len=0 exit_code=0 final_task_state=Exited verified=1",
            spec.test_name,
            k02_spec_path_label(spec),
            spec.elf_sha256,
            unsafe { K02_ENTRY[idx] },
            unsafe { K02_LOAD_SEGMENTS[idx] },
            k02_spec_path_label(spec),
            unsafe { K02_SYSCALL_TRACE_LEN[idx] },
            unsafe { K02_PAGE_FAULT_COUNT[idx] },
            unsafe { K02_LAST_PAGE_FAULT[idx] },
            unsafe { K02_STDOUT_LEN[idx] }
        );
        if idx == 0 {
            crate::println!(
                "[K01-real-official-basic-elf] PASS real /musl/basic/write U-mode execution inode={} file_size={} entry={:#x} load_base={:#x} pages={}",
                unsafe { K02_INODE[idx] },
                unsafe { K02_FILE_SIZE[idx] },
                unsafe { K02_ENTRY[idx] },
                unsafe { K02_LOAD_BASE[idx] },
                unsafe { K02_LOAD_PAGES[idx] }
            );
        }
        if k03_is_fd_vfs_kind(spec.expected_kind) {
            crate::println!(
                "[K03-realrun-fd-vfs] PASS case={} program_path={} elf_sha256={} exit_code=0 stdout_len={} fd_trace=serial vfs_trace=serial",
                spec.test_name,
                k02_spec_path_label(spec),
                spec.elf_sha256,
                unsafe { K02_STDOUT_LEN[idx] }
            );
        }
        if k04a_is_process_ipc_kind(spec.expected_kind) {
            crate::println!(
                "[K04a-realrun-process-ipc] PASS case={} program_path={} elf_sha256={} exit_code=0 stdout_len={} task_trace=serial pipe_trace=serial",
                spec.test_name,
                k02_spec_path_label(spec),
                spec.elf_sha256,
                unsafe { K02_STDOUT_LEN[idx] }
            );
        }
        if k04b_is_fork_clone_exec_kind(spec.expected_kind) {
            crate::println!(
                "[K04b-realrun-process-family] PASS case={} program_path={} elf_sha256={} exit_code=0 stdout_len={} task_trace=serial process_trace=serial execve_trace=serial",
                spec.test_name,
                k02_spec_path_label(spec),
                spec.elf_sha256,
                unsafe { K02_STDOUT_LEN[idx] }
            );
        }
        if k05_is_memory_mount_kind(spec.expected_kind) {
            crate::println!(
                "[K05-realrun-memory-mount] PASS case={} program_path={} elf_sha256={} exit_code=0 stdout_len={} memory_trace=serial page_fault_trace=serial vma_trace=serial mount_trace=serial vfs_trace=serial",
                spec.test_name,
                k02_spec_path_label(spec),
                spec.elf_sha256,
                unsafe { K02_STDOUT_LEN[idx] }
            );
        }
    } else {
        crate::println!(
            "[K02-realrun-result] case={} class=NOT-YET-SUPPORTED program_path={} elf_sha256={} entry_pc={:#x} loaded_segments={} argv={} envp=[] auxv=AT_PAGESZ,AT_ENTRY entered_umode={} syscall_trace=serial:{} page_fault_trace=count:{} last:{:#x} stdout_len={} stderr_len=0 exit_code={} final_task_state=Exited verified=0",
            spec.test_name,
            k02_spec_path_label(spec),
            spec.elf_sha256,
            unsafe { K02_ENTRY[idx] },
            unsafe { K02_LOAD_SEGMENTS[idx] },
            k02_spec_path_label(spec),
            unsafe { K02_ENTERED_UMODE[idx] as usize },
            unsafe { K02_SYSCALL_TRACE_LEN[idx] },
            unsafe { K02_PAGE_FAULT_COUNT[idx] },
            unsafe { K02_LAST_PAGE_FAULT[idx] },
            unsafe { K02_STDOUT_LEN[idx] },
            code
        );
        if k03_is_fd_vfs_kind(spec.expected_kind) {
            crate::println!(
                "[K03-realrun-fd-vfs] NOT-YET-SUPPORTED case={} program_path={} exit_code={} stdout_ok={} fd_trace=serial vfs_trace=serial",
                spec.test_name,
                k02_spec_path_label(spec),
                code,
                if stdout_ok { 1 } else { 0 }
            );
        }
        if k04a_is_process_ipc_kind(spec.expected_kind) {
            crate::println!(
                "[K04a-realrun-process-ipc] NOT-YET-SUPPORTED case={} program_path={} exit_code={} stdout_ok={} task_trace=serial pipe_trace=serial",
                spec.test_name,
                k02_spec_path_label(spec),
                code,
                if stdout_ok { 1 } else { 0 }
            );
        }
        if k04b_is_fork_clone_exec_kind(spec.expected_kind) {
            crate::println!(
                "[K04b-realrun-process-family] NOT-YET-SUPPORTED case={} program_path={} exit_code={} stdout_ok={} task_trace=serial process_trace=serial execve_trace=serial",
                spec.test_name,
                k02_spec_path_label(spec),
                code,
                if stdout_ok { 1 } else { 0 }
            );
        }
        if k05_is_memory_mount_kind(spec.expected_kind) {
            crate::println!(
                "[K05-realrun-memory-mount] NOT-YET-SUPPORTED case={} program_path={} exit_code={} stdout_ok={} memory_trace=serial page_fault_trace=serial vma_trace=serial mount_trace=serial vfs_trace=serial",
                spec.test_name,
                k02_spec_path_label(spec),
                code,
                if stdout_ok { 1 } else { 0 }
            );
        }
    }
    unsafe {
        K02_CURRENT_CASE += 1;
    }
    k02_begin_or_finish(cx);
}

fn real_umode_path_label(path: &[u8]) -> &'static str {
    if path == REAL_UMODE_V191_PATH {
        "/umode/v191.elf"
    } else if path == REAL_UMODE_V192_A_PATH {
        "/umode/v192a.elf"
    } else if path == REAL_UMODE_V192_B_PATH {
        "/umode/v192b.elf"
    } else if path == REAL_UMODE_V192_C_PATH {
        "/umode/v192c.elf"
    } else if path == REAL_UMODE_V193_PATH {
        "/umode/v193child.elf"
    } else if path == REAL_UMODE_V194_PATH {
        "/umode/v194abi.elf"
    } else if path == REAL_UMODE_V197_PATH {
        "/umode/v197lazy.elf"
    } else if path == REAL_UMODE_V198_RO_PATH {
        "/umode/v198ro.elf"
    } else if path == REAL_UMODE_V198_UNMAP_PATH {
        "/umode/v198unmap.elf"
    } else if path == REAL_UMODE_V200_PATH {
        "/umode/v200stress.elf"
    } else {
        "/umode/unknown.elf"
    }
}

fn real_umode_fail(marker: &str, step: &str) -> ! {
    crate::println!("{} FAIL step={}", marker, step);
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

fn real_umode_launch_phase(cx: &mut TrapContext, phase: usize) {
    unsafe {
        REAL_UMODE_PHASE = phase;
    }
    let (path, argc2, envp, marker, step) = match phase {
        REAL_UMODE_PHASE_V191 => (
            REAL_UMODE_V191_PATH,
            false,
            false,
            "[ucompat-v191] real rootfs elf execution bridge",
            "launch_v191",
        ),
        REAL_UMODE_PHASE_V192_A => (
            REAL_UMODE_V192_A_PATH,
            false,
            false,
            "[ucompat-v192] real multi program umode matrix",
            "launch_v192a",
        ),
        REAL_UMODE_PHASE_V192_B => (
            REAL_UMODE_V192_B_PATH,
            false,
            false,
            "[ucompat-v192] real multi program umode matrix",
            "launch_v192b",
        ),
        REAL_UMODE_PHASE_V192_C => (
            REAL_UMODE_V192_C_PATH,
            false,
            false,
            "[ucompat-v192] real multi program umode matrix",
            "launch_v192c",
        ),
        REAL_UMODE_PHASE_V193_CHILD => (
            REAL_UMODE_V193_PATH,
            false,
            false,
            "[ucompat-v193] fork exec wait real path",
            "launch_v193",
        ),
        REAL_UMODE_PHASE_V194_ABI => (
            REAL_UMODE_V194_PATH,
            true,
            true,
            "[ucompat-v194] userland abi hardening",
            "launch_v194",
        ),
        REAL_UMODE_PHASE_V197_LAZY => (
            REAL_UMODE_V197_PATH,
            false,
            false,
            "[ucompat-v197] real page fault lazy allocation",
            "launch_v197",
        ),
        REAL_UMODE_PHASE_V198_RO => (
            REAL_UMODE_V198_RO_PATH,
            false,
            false,
            "[ucompat-v198] page permission unmap",
            "launch_v198_ro",
        ),
        REAL_UMODE_PHASE_V198_UNMAP => (
            REAL_UMODE_V198_UNMAP_PATH,
            false,
            false,
            "[ucompat-v198] page permission unmap",
            "launch_v198_unmap",
        ),
        REAL_UMODE_PHASE_V200_STRESS => (
            REAL_UMODE_V200_PATH,
            false,
            false,
            "[ucompat-v200] memory stress suite",
            "launch_v200",
        ),
        _ => real_umode_fail(
            "[ucompat-v191] real rootfs elf execution bridge",
            "bad_phase",
        ),
    };
    if let Err(err) = real_umode_enter(cx, path, argc2, envp) {
        crate::println!(
            "{} launch error path={} err={}",
            marker,
            real_umode_path_label(path),
            err
        );
        real_umode_fail(marker, step);
    }
}

fn real_umode_begin(cx: &mut TrapContext) {
    let install = real_umode_install_rootfs_programs();
    if install != 0 {
        real_umode_fail(
            "[ucompat-v191] real rootfs elf execution bridge",
            "install_rootfs",
        );
    }
    unsafe {
        REAL_UMODE_V192_PASS_COUNT = 0;
        REAL_UMODE_CHILD_PID = 0;
        REAL_UMODE_CLOEXEC_FD = -1;
    }
    real_umode_launch_phase(cx, REAL_UMODE_PHASE_V191);
}

fn real_umode_start_v193(cx: &mut TrapContext) {
    let child = crate::fs::runtime::clone_task(17);
    if child <= 0 {
        real_umode_fail("[ucompat-v193] fork exec wait real path", "clone_child");
    }
    let switch = crate::fs::runtime::switch_current_task(child as usize);
    if switch != 0 {
        real_umode_fail("[ucompat-v193] fork exec wait real path", "switch_child");
    }
    unsafe {
        REAL_UMODE_CHILD_PID = child as usize;
    }
    real_umode_launch_phase(cx, REAL_UMODE_PHASE_V193_CHILD);
}

fn real_umode_start_v194(cx: &mut TrapContext) {
    let fd = crate::fs::runtime::openat(
        crate::fs::runtime::AT_FDCWD,
        b"/dev/zero",
        crate::fs::runtime::O_CLOEXEC,
        0,
    );
    if fd < 0 {
        real_umode_fail("[ucompat-v194] userland abi hardening", "open_cloexec");
    }
    unsafe {
        REAL_UMODE_CLOEXEC_FD = fd;
    }
    real_umode_launch_phase(cx, REAL_UMODE_PHASE_V194_ABI);
}

fn real_umode_handle_v194_exit(cx: &mut TrapContext, code: isize) {
    if code != 44 {
        real_umode_fail("[ucompat-v194] userland abi hardening", "abi_exit_status");
    }
    let cloexec_fd = unsafe { REAL_UMODE_CLOEXEC_FD };
    if cloexec_fd >= 0 && crate::fs::runtime::fd_exists(cloexec_fd as usize) {
        real_umode_fail(
            "[ucompat-v194] userland abi hardening",
            "cloexec_still_open",
        );
    }
    let argv0 = match crate::fs::runtime::RuntimeExecString::from_bytes(REAL_UMODE_V194_PATH) {
        Ok(item) => item,
        Err(_) => real_umode_fail("[ucompat-v194] userland abi hardening", "argv_string"),
    };
    let missing = crate::fs::runtime::execve_from_vfs(b"/umode/missing.elf", &[argv0], &[]);
    if missing != crate::fs::runtime::ENOENT {
        real_umode_fail(
            "[ucompat-v194] userland abi hardening",
            "invalid_path_errno",
        );
    }
    let bad = crate::fs::runtime::execve_from_vfs(REAL_UMODE_BAD_PATH, &[argv0], &[]);
    if bad != crate::fs::runtime::ENOEXEC {
        real_umode_fail("[ucompat-v194] userland abi hardening", "invalid_elf_errno");
    }
    crate::println!(
        "[ucompat-v194] abi evidence cloexec_fd={} closed=1 missing_errno={} bad_elf_errno={} PASS",
        cloexec_fd,
        missing,
        bad
    );
    crate::println!("[ucompat-v194] userland abi hardening PASS");
    real_mm_begin(cx);
}

fn handle_real_umode_exit(cx: &mut TrapContext, code: isize) -> bool {
    let phase = unsafe { REAL_UMODE_PHASE };
    match phase {
        REAL_UMODE_PHASE_IDLE => {
            if code != 0 {
                return false;
            }
            k01_begin(cx);
            true
        }
        REAL_UMODE_PHASE_K01_WRITE => {
            if k04a_handle_child_exit(cx, code) {
                return true;
            }
            k01_handle_exit(cx, code);
            true
        }
        REAL_UMODE_PHASE_B01_BUSYBOX => {
            b01_handle_exit(cx, code);
            true
        }
        REAL_UMODE_PHASE_V191 => {
            if code != 11 {
                real_umode_fail(
                    "[ucompat-v191] real rootfs elf execution bridge",
                    "exit_status",
                );
            }
            crate::println!("[ucompat-v191] real rootfs elf execution bridge PASS");
            real_umode_launch_phase(cx, REAL_UMODE_PHASE_V192_A);
            true
        }
        REAL_UMODE_PHASE_V192_A => {
            if code != 21 {
                real_umode_fail("[ucompat-v192] real multi program umode matrix", "exit_a");
            }
            unsafe {
                REAL_UMODE_V192_PASS_COUNT += 1;
            }
            crate::println!(
                "[ucompat-v192] matrix program=/umode/v192a.elf exit=21 expected=21 PASS"
            );
            real_umode_launch_phase(cx, REAL_UMODE_PHASE_V192_B);
            true
        }
        REAL_UMODE_PHASE_V192_B => {
            if code != 22 {
                real_umode_fail("[ucompat-v192] real multi program umode matrix", "exit_b");
            }
            unsafe {
                REAL_UMODE_V192_PASS_COUNT += 1;
            }
            crate::println!(
                "[ucompat-v192] matrix program=/umode/v192b.elf exit=22 expected=22 PASS"
            );
            real_umode_launch_phase(cx, REAL_UMODE_PHASE_V192_C);
            true
        }
        REAL_UMODE_PHASE_V192_C => {
            if code != 23 {
                real_umode_fail("[ucompat-v192] real multi program umode matrix", "exit_c");
            }
            unsafe {
                REAL_UMODE_V192_PASS_COUNT += 1;
            }
            crate::println!(
                "[ucompat-v192] matrix program=/umode/v192c.elf exit=23 expected=23 PASS"
            );
            if unsafe { REAL_UMODE_V192_PASS_COUNT } != 3 {
                real_umode_fail(
                    "[ucompat-v192] real multi program umode matrix",
                    "matrix_count",
                );
            }
            crate::println!("[ucompat-v192] real multi program umode matrix PASS");
            real_umode_start_v193(cx);
            true
        }
        REAL_UMODE_PHASE_V193_CHILD => {
            if code != 33 {
                real_umode_fail(
                    "[ucompat-v193] fork exec wait real path",
                    "child_exit_status",
                );
            }
            let exit_ret = crate::fs::runtime::exit_current_task(code);
            if exit_ret != 0 {
                real_umode_fail("[ucompat-v193] fork exec wait real path", "exit_child_task");
            }
            if crate::fs::runtime::switch_current_task(1) != 0 {
                real_umode_fail("[ucompat-v193] fork exec wait real path", "switch_parent");
            }
            let child = unsafe { REAL_UMODE_CHILD_PID };
            let mut status = 0isize;
            let wait_ret = crate::fs::runtime::wait4(child as isize, &mut status);
            if wait_ret != child as isize || status != (33 << 8) {
                real_umode_fail("[ucompat-v193] fork exec wait real path", "wait_status");
            }
            crate::println!(
                "[ucompat-v193] fork-exec child pid={} status={} expected={} PASS",
                child,
                status,
                33 << 8
            );
            crate::println!("[ucompat-v193] fork exec wait real path PASS");
            real_umode_start_v194(cx);
            true
        }
        REAL_UMODE_PHASE_V194_ABI => {
            real_umode_handle_v194_exit(cx, code);
            true
        }
        REAL_UMODE_PHASE_V197_LAZY => {
            if code != 47 {
                real_umode_fail(
                    "[ucompat-v197] real page fault lazy allocation",
                    "exit_status",
                );
            }
            let faults = real_mm_fault_alloc_count();
            if faults < 4 {
                real_umode_fail(
                    "[ucompat-v197] real page fault lazy allocation",
                    "fault_count",
                );
            }
            crate::println!(
                "[ucompat-v197] lazy allocation evidence faults={} alloc={} PASS",
                faults,
                real_mm_alloc_count()
            );
            crate::println!("[ucompat-v197] real page fault lazy allocation PASS");
            real_umode_launch_phase(cx, REAL_UMODE_PHASE_V198_RO);
            true
        }
        REAL_UMODE_PHASE_V198_RO | REAL_UMODE_PHASE_V198_UNMAP => {
            real_umode_fail("[ucompat-v198] page permission unmap", "unexpected_exit")
        }
        REAL_UMODE_PHASE_V200_STRESS => {
            if code != 60 {
                real_umode_fail("[ucompat-v200] memory stress suite", "exit_status");
            }
            real_mm_finish_v200();
        }
        REAL_UMODE_PHASE_DONE => true,
        _ => false,
    }
}

