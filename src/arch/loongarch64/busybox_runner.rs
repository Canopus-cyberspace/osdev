use crate::console::{write_usize_dec, write_usize_hex};
use crate::early_console_write;
use crate::fd_table::{self, PathBuf};
use crate::real_elf::{self, ExecString};
use crate::trap;
use crate::user;

pub(crate) fn run_loongarch_busybox_loader_probe() {
    let mut argv = [ExecString::empty(); 2];
    if argv[0].set_from_slice(b"busybox").is_err() || argv[1].set_from_slice(b"true").is_err() {
        early_console_write("[loongarch64-busybox] blocker: argv setup failed\n");
        return;
    }

    match real_elf::load_user_elf_with_args("/musl/busybox", &argv, &[]) {
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
            early_console_write("\n");
            run_true_probe(load.entry, load.stack_pointer);
        }
        Err(err) => {
            early_console_write("[loongarch64-busybox] blocker: failed to load /musl/busybox: ");
            early_console_write(err);
            early_console_write("\n");
        }
    }
}

fn run_true_probe(entry: usize, stack_pointer: usize) {
    user::reset_case_state();
    let mut cwd = PathBuf::empty();
    let _ = cwd.set_from_slice(b"/musl");
    fd_table::set_cwd(&cwd);

    early_console_write("[loongarch64-busybox] entering command=true\n");
    trap::enter_user_entry(entry, stack_pointer);
    let report = user::run_snapshot();
    if report.exited {
        early_console_write("[loongarch64-busybox] command=true exit_code=");
        write_usize_dec(report.exit_code);
        early_console_write("\n");
    } else if report.fault_active {
        early_console_write("[loongarch64-busybox] blocker: user fault ecode=");
        write_usize_dec(report.fault_ecode);
        early_console_write(" era=");
        write_usize_hex(report.fault_era);
        early_console_write(" badv=");
        write_usize_hex(report.fault_badv);
        early_console_write("\n");
    } else if report.missing_syscall_active {
        early_console_write("[loongarch64-busybox] blocker: missing syscall id=");
        write_usize_dec(report.missing_syscall_id);
        early_console_write("\n");
    } else {
        early_console_write("[loongarch64-busybox] blocker: returned without exit status\n");
    }
}
