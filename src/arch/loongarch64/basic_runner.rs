use crate::early_console_write;
use crate::real_elf;
use crate::trap;
use crate::user;
use crate::console::write_usize_dec;

#[derive(Copy, Clone)]
struct LaBasicCase {
    name: &'static str,
    path: &'static str,
    enabled: bool,
}

const CASES: &[LaBasicCase] = &[
    LaBasicCase {
        name: "write",
        path: "/musl/basic/write",
        enabled: true,
    },
    LaBasicCase {
        name: "getpid",
        path: "/musl/basic/getpid",
        enabled: true,
    },
    LaBasicCase {
        name: "getppid",
        path: "/musl/basic/getppid",
        enabled: true,
    },
    LaBasicCase {
        name: "uname",
        path: "/musl/basic/uname",
        enabled: true,
    },
    LaBasicCase {
        name: "gettimeofday",
        path: "/musl/basic/gettimeofday",
        enabled: true,
    },
    LaBasicCase {
        name: "times",
        path: "/musl/basic/times",
        enabled: true,
    },
    LaBasicCase {
        name: "sleep",
        path: "/musl/basic/sleep",
        enabled: true,
    },
    LaBasicCase {
        name: "getcwd",
        path: "/musl/basic/getcwd",
        enabled: true,
    },
    LaBasicCase {
        name: "brk",
        path: "/musl/basic/brk",
        enabled: true,
    },
    LaBasicCase {
        name: "exit",
        path: "/musl/basic/exit",
        enabled: false,
    },
    LaBasicCase {
        name: "fork",
        path: "/musl/basic/fork",
        enabled: false,
    },
    LaBasicCase {
        name: "clone",
        path: "/musl/basic/clone",
        enabled: false,
    },
    LaBasicCase {
        name: "execve",
        path: "/musl/basic/execve",
        enabled: false,
    },
    LaBasicCase {
        name: "wait",
        path: "/musl/basic/wait",
        enabled: false,
    },
    LaBasicCase {
        name: "waitpid",
        path: "/musl/basic/waitpid",
        enabled: false,
    },
    LaBasicCase {
        name: "yield",
        path: "/musl/basic/yield",
        enabled: false,
    },
    LaBasicCase {
        name: "mmap",
        path: "/musl/basic/mmap",
        enabled: true,
    },
    LaBasicCase {
        name: "mount",
        path: "/musl/basic/mount",
        enabled: true,
    },
    LaBasicCase {
        name: "munmap",
        path: "/musl/basic/munmap",
        enabled: true,
    },
    LaBasicCase {
        name: "umount",
        path: "/musl/basic/umount",
        enabled: true,
    },
    LaBasicCase {
        name: "close",
        path: "/musl/basic/close",
        enabled: true,
    },
    LaBasicCase {
        name: "dup",
        path: "/musl/basic/dup",
        enabled: true,
    },
    LaBasicCase {
        name: "dup2",
        path: "/musl/basic/dup2",
        enabled: true,
    },
    LaBasicCase {
        name: "pipe",
        path: "/musl/basic/pipe",
        enabled: false,
    },
    LaBasicCase {
        name: "open",
        path: "/musl/basic/open",
        enabled: true,
    },
    LaBasicCase {
        name: "read",
        path: "/musl/basic/read",
        enabled: true,
    },
    LaBasicCase {
        name: "openat",
        path: "/musl/basic/openat",
        enabled: true,
    },
    LaBasicCase {
        name: "fstat",
        path: "/musl/basic/fstat",
        enabled: true,
    },
    LaBasicCase {
        name: "getdents",
        path: "/musl/basic/getdents",
        enabled: true,
    },
    LaBasicCase {
        name: "chdir",
        path: "/musl/basic/chdir",
        enabled: true,
    },
    LaBasicCase {
        name: "mkdir",
        path: "/musl/basic/mkdir_",
        enabled: true,
    },
    LaBasicCase {
        name: "unlink",
        path: "/musl/basic/unlink",
        enabled: true,
    },
];

pub(crate) fn run_loongarch_basic_musl_group() {
    let first_index = match first_enabled_case() {
        Some(index) => index,
        None => {
            early_console_write("[loongarch64-basic] blocker: no enabled basic-musl cases\n");
            return;
        }
    };

    early_console_write("[loongarch64-basic] preparing first case ");
    early_console_write(CASES[first_index].name);
    early_console_write("\n");
    let mut first_load = match real_elf::load_basic_case(CASES[first_index].path) {
        Ok(load) => Some(load),
        Err(err) => {
            early_console_write("[loongarch64-basic] blocker: failed to load ");
            early_console_write(CASES[first_index].path);
            early_console_write(": ");
            early_console_write(err);
            early_console_write("\n");
            return;
        }
    };

    let mut attempted = 0usize;
    let mut completed = 0usize;
    let mut failed: Option<&'static str> = None;
    early_console_write("#### OS COMP TEST GROUP START basic-musl ####\n");
    user::set_basic_group_active(true);

    let mut i = first_index;
    while i < CASES.len() {
        let case = CASES[i];
        if case.enabled {
            attempted += 1;
            let load = if i == first_index {
                match first_load.take() {
                    Some(load) => load,
                    None => match real_elf::load_basic_case(case.path) {
                        Ok(load) => load,
                        Err(_) => {
                            failed = Some(case.name);
                            break;
                        }
                    },
                }
            } else {
                match real_elf::load_basic_case(case.path) {
                    Ok(load) => load,
                    Err(_) => {
                        failed = Some(case.name);
                        break;
                    }
                }
            };
            user::reset_case_state();
            trap::enter_user_entry(load.entry, load.stack_pointer);
            if user::report_user_run_status(case.name, false) {
                completed += 1;
            } else {
                failed = Some(case.name);
                break;
            }
        }
        i += 1;
    }

    user::set_basic_group_active(false);
    early_console_write("#### OS COMP TEST GROUP END basic-musl ####\n");
    early_console_write("[loongarch64-basic] attempted=");
    write_usize_dec(attempted);
    early_console_write(" completed=");
    write_usize_dec(completed);
    early_console_write(" failed=");
    match failed {
        Some(name) => early_console_write(name),
        None => early_console_write("none"),
    }
    early_console_write("\n");
    user::report_missing_syscall();
    if let Some(name) = failed {
        user::report_user_run_status(name, true);
    }
}

fn first_enabled_case() -> Option<usize> {
    let mut i = 0usize;
    while i < CASES.len() {
        if CASES[i].enabled {
            return Some(i);
        }
        i += 1;
    }
    None
}
