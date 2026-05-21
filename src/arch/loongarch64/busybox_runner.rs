use crate::console::{write_usize_dec, write_usize_hex};
use crate::early_console_write;
use crate::fd_table::{self, PathBuf};
use crate::real_elf::{self, ExecString};
use crate::trap;
use crate::user;

struct BusyboxCommand {
    name: &'static str,
    argv: &'static [&'static [u8]],
}

const COMMANDS: &[BusyboxCommand] = &[
    BusyboxCommand {
        name: "true",
        argv: &[b"busybox", b"true"],
    },
];

pub(crate) fn run_loongarch_busybox_loader_probe() {
    let mut completed = 0usize;
    let mut i = 0usize;
    while i < COMMANDS.len() {
        if run_command(&COMMANDS[i]) {
            completed += 1;
            i += 1;
        } else {
            break;
        }
    }
    early_console_write("[loongarch64-busybox] smoke completed=");
    write_usize_dec(completed);
    early_console_write(" attempted=");
    write_usize_dec(i + if i < COMMANDS.len() { 1 } else { 0 });
    early_console_write("\n");
}

fn run_command(command: &BusyboxCommand) -> bool {
    let mut argv = [ExecString::empty(); 4];
    let mut i = 0usize;
    while i < command.argv.len() {
        if argv[i].set_from_slice(command.argv[i]).is_err() {
            early_console_write("[loongarch64-busybox] blocker: argv setup failed command=");
            early_console_write(command.name);
            early_console_write("\n");
            return false;
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
            run_probe(command.name, load.entry, load.stack_pointer)
        }
        Err(err) => {
            early_console_write("[loongarch64-busybox] blocker: failed to load /musl/busybox: ");
            early_console_write(err);
            early_console_write("\n");
            false
        }
    }
}

fn run_probe(command_name: &str, entry: usize, stack_pointer: usize) -> bool {
    user::reset_case_state();
    let mut cwd = PathBuf::empty();
    let _ = cwd.set_from_slice(b"/musl");
    fd_table::set_cwd(&cwd);

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
            return false;
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
        true
    } else if report.fault_active {
        early_console_write("[loongarch64-busybox] blocker: user fault ecode=");
        write_usize_dec(report.fault_ecode);
        early_console_write(" era=");
        write_usize_hex(report.fault_era);
        early_console_write(" badv=");
        write_usize_hex(report.fault_badv);
        early_console_write("\n");
        real_elf::dump_user_regions("[loongarch64-busybox] ");
        false
    } else if report.missing_syscall_active {
        early_console_write("[loongarch64-busybox] blocker: missing syscall id=");
        write_usize_dec(report.missing_syscall_id);
        early_console_write("\n");
        false
    } else {
        early_console_write("[loongarch64-busybox] blocker: returned without exit status\n");
        false
    }
}
