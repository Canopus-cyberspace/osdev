use crate::console::{write_usize_dec, write_usize_hex};
use crate::early_console_write;
use crate::fd_table::{self, PathBuf};
use crate::real_elf::{self, ExecString};
use crate::trap;
use crate::user;

struct BusyboxCommand {
    name: &'static str,
    argv: &'static [&'static [u8]],
    expected_exit: usize,
    official_name: Option<&'static str>,
    class: BusyboxCommandClass,
}

#[derive(Copy, Clone)]
enum BusyboxCommandClass {
    Scoring,
    Smoke,
    Disabled,
}

const RUN_COMMANDS: &[BusyboxCommand] = &[
    BusyboxCommand {
        name: "true",
        argv: &[b"busybox", b"true"],
        expected_exit: 0,
        official_name: Some("true"),
        class: BusyboxCommandClass::Scoring,
    },
    BusyboxCommand {
        name: "false",
        argv: &[b"busybox", b"false"],
        expected_exit: 1,
        official_name: Some("false"),
        class: BusyboxCommandClass::Scoring,
    },
    BusyboxCommand {
        name: "echo",
        argv: &[b"busybox", b"echo", b"hello"],
        expected_exit: 0,
        official_name: None,
        class: BusyboxCommandClass::Smoke,
    },
    BusyboxCommand {
        name: "pwd",
        argv: &[b"busybox", b"pwd"],
        expected_exit: 0,
        official_name: Some("pwd"),
        class: BusyboxCommandClass::Scoring,
    },
    BusyboxCommand {
        name: "sh-exit",
        argv: &[b"busybox", b"sh", b"-c", b"exit"],
        expected_exit: 0,
        official_name: Some("sh -c exit"),
        class: BusyboxCommandClass::Scoring,
    },
    BusyboxCommand {
        name: "ls",
        argv: &[b"busybox", b"ls"],
        expected_exit: 0,
        official_name: Some("ls"),
        class: BusyboxCommandClass::Scoring,
    },
    BusyboxCommand {
        name: "cat",
        argv: &[b"busybox", b"cat", b"/musl/busybox_cmd.txt"],
        expected_exit: 0,
        official_name: None,
        class: BusyboxCommandClass::Smoke,
    },
];

const DISABLED_COMMANDS: &[BusyboxCommand] = &[
    BusyboxCommand {
        name: "basename",
        argv: &[b"busybox", b"basename", b"/aaa/bbb"],
        expected_exit: 0,
        official_name: Some("basename /aaa/bbb"),
        class: BusyboxCommandClass::Disabled,
    },
    BusyboxCommand {
        name: "uname",
        argv: &[b"busybox", b"uname"],
        expected_exit: 0,
        official_name: Some("uname"),
        class: BusyboxCommandClass::Disabled,
    },
    BusyboxCommand {
        name: "ash-exit",
        argv: &[b"busybox", b"ash", b"-c", b"exit"],
        expected_exit: 0,
        official_name: Some("ash -c exit"),
        class: BusyboxCommandClass::Disabled,
    },
];

pub(crate) fn run_loongarch_busybox_loader_probe() {
    early_console_write("[loongarch64-busybox] scoring-capable group begin\n");
    early_console_write("#### OS COMP TEST GROUP START busybox-musl ####\n");
    user::set_busybox_group_active(true);
    let mut completed = 0usize;
    let mut matched = 0usize;
    let mut failed = 0usize;
    let mut attempted = 0usize;
    run_command_set(
        RUN_COMMANDS,
        &mut attempted,
        &mut completed,
        &mut matched,
        &mut failed,
    );
    user::set_busybox_group_active(false);
    early_console_write("#### OS COMP TEST GROUP END busybox-musl ####\n");
    early_console_write("[loongarch64-busybox] smoke completed=");
    write_usize_dec(completed);
    early_console_write(" attempted=");
    write_usize_dec(attempted);
    early_console_write(" matched=");
    write_usize_dec(matched);
    early_console_write(" failed=");
    write_usize_dec(failed);
    early_console_write(" disabled=");
    write_usize_dec(DISABLED_COMMANDS.len());
    early_console_write("\n");
}

fn run_command_set(
    commands: &[BusyboxCommand],
    attempted: &mut usize,
    completed: &mut usize,
    matched: &mut usize,
    failed: &mut usize,
) {
    let mut i = 0usize;
    while i < commands.len() {
        *attempted += 1;
        let result = run_command(&commands[i]);
        if result.completed {
            *completed += 1;
        }
        if result.matched_expected {
            *matched += 1;
        } else {
            *failed += 1;
        }
        if let BusyboxCommandClass::Scoring = commands[i].class {
            emit_official_result(&commands[i], result.matched_expected);
        }
        i += 1;
    }
}

fn emit_official_result(command: &BusyboxCommand, success: bool) {
    if let Some(name) = command.official_name {
        early_console_write("testcase busybox ");
        early_console_write(name);
        if success {
            early_console_write(" success\n");
        } else {
            early_console_write(" fail\n");
        }
    }
}

struct BusyboxRunResult {
    completed: bool,
    matched_expected: bool,
}

fn run_command(command: &BusyboxCommand) -> BusyboxRunResult {
    let mut argv = [ExecString::empty(); 8];
    if command.argv.len() > argv.len() {
        early_console_write("[loongarch64-busybox] blocker: too many argv entries command=");
        early_console_write(command.name);
        early_console_write("\n");
        return BusyboxRunResult {
            completed: false,
            matched_expected: false,
        };
    }
    let mut i = 0usize;
    while i < command.argv.len() {
        if argv[i].set_from_slice(command.argv[i]).is_err() {
            early_console_write("[loongarch64-busybox] blocker: argv setup failed command=");
            early_console_write(command.name);
            early_console_write("\n");
            return BusyboxRunResult {
                completed: false,
                matched_expected: false,
            };
        }
        i += 1;
    }

    match real_elf::load_user_elf_with_args("/musl/busybox", &argv[..command.argv.len()], &[]) {
        Ok(load) => {
            early_console_write("[loongarch64-busybox] loaded /musl/busybox file_size=");
            write_usize_dec(load.file_size);
            early_console_write(" entry=");
            write_usize_hex(load.entry);
            early_console_write(" first_load=");
            write_usize_hex(load.load_base);
            early_console_write(" load_size=");
            write_usize_dec(load.load_size);
            early_console_write(" segments=");
            write_usize_dec(load.load_segments);
            early_console_write(" command=");
            early_console_write(command.name);
            early_console_write("\n");
            run_probe(
                command.name,
                command.expected_exit,
                load.entry,
                load.stack_pointer,
            )
        }
        Err(err) => {
            early_console_write("[loongarch64-busybox] blocker: failed to load /musl/busybox: ");
            early_console_write(err);
            early_console_write("\n");
            BusyboxRunResult {
                completed: false,
                matched_expected: false,
            }
        }
    }
}

fn run_probe(
    command_name: &str,
    expected_exit: usize,
    entry: usize,
    stack_pointer: usize,
) -> BusyboxRunResult {
    user::reset_case_state();
    user::start_syscall_budget(4096);
    let mut cwd = PathBuf::empty();
    let _ = cwd.set_from_slice(b"/musl");
    fd_table::set_cwd(&cwd);

    early_console_write("[loongarch64-busybox] command-start=");
    early_console_write(command_name);
    early_console_write("\n");
    early_console_write("[loongarch64-busybox] entering command=");
    early_console_write(command_name);
    early_console_write("\n");
    match real_elf::activate_current_user_mmu() {
        Ok(()) => {
            early_console_write("[loongarch64-busybox] mapped entry=");
            write_usize_hex(real_elf::current_entry());
            early_console_write("\n");
        }
        Err(err) => {
            early_console_write("[loongarch64-busybox] blocker: failed to activate user mmu: ");
            early_console_write(err);
            early_console_write("\n");
            return BusyboxRunResult {
                completed: false,
                matched_expected: false,
            };
        }
    }
    trap::enter_user_entry(entry, stack_pointer);
    real_elf::deactivate_current_user_mmu();
    let report = user::run_snapshot();
    if report.exited {
        early_console_write("[loongarch64-busybox] command=");
        early_console_write(command_name);
        early_console_write(" exit_code=");
        write_usize_dec(report.exit_code);
        early_console_write("\n");
        BusyboxRunResult {
            completed: true,
            matched_expected: report.exit_code == expected_exit,
        }
    } else if report.timeout_active {
        early_console_write("[loongarch64-busybox] blocker: command timeout command=");
        early_console_write(command_name);
        early_console_write(" syscalls=");
        write_usize_dec(report.timeout_syscalls);
        early_console_write(" last_syscall=");
        write_usize_dec(report.timeout_last_syscall_id);
        early_console_write("\n");
        BusyboxRunResult {
            completed: false,
            matched_expected: false,
        }
    } else if report.fault_active {
        early_console_write("[loongarch64-busybox] blocker: user fault ecode=");
        write_usize_dec(report.fault_ecode);
        early_console_write(" era=");
        write_usize_hex(report.fault_era);
        early_console_write(" badv=");
        write_usize_hex(report.fault_badv);
        early_console_write("\n");
        real_elf::dump_user_regions("[loongarch64-busybox] ");
        BusyboxRunResult {
            completed: false,
            matched_expected: false,
        }
    } else if report.missing_syscall_active {
        early_console_write("[loongarch64-busybox] blocker: missing syscall id=");
        write_usize_dec(report.missing_syscall_id);
        early_console_write("\n");
        BusyboxRunResult {
            completed: false,
            matched_expected: false,
        }
    } else {
        early_console_write("[loongarch64-busybox] blocker: returned without exit status\n");
        BusyboxRunResult {
            completed: false,
            matched_expected: false,
        }
    }
}
