use crate::console::{write_usize_dec, write_usize_hex};
use crate::early_console_write;
use crate::fd_table;
use crate::process;

static mut WRITE_SYSCALL_COUNT: usize = 0;
static mut BASIC_GROUP_ACTIVE: bool = false;
static mut USER_EXITED: bool = false;
static mut USER_EXIT_CODE: usize = 0;
static mut USER_FAULT_ACTIVE: bool = false;
static mut USER_FAULT_ECODE: usize = 0;
static mut USER_FAULT_ERA: usize = 0;
static mut USER_FAULT_BADV: usize = 0;
static mut MISSING_SYSCALL_ACTIVE: bool = false;
static mut MISSING_SYSCALL_ID: usize = 0;
static mut TIMEOUT_ACTIVE: bool = false;
static mut SYSCALL_BUDGET_LIMIT: usize = 0;
static mut SYSCALL_BUDGET_COUNT: usize = 0;
static mut SYSCALL_BUDGET_LAST_ID: usize = 0;

#[derive(Copy, Clone)]
pub(crate) struct UserRunSnapshot {
    pub(crate) exited: bool,
    pub(crate) exit_code: usize,
    pub(crate) fault_active: bool,
    pub(crate) fault_ecode: usize,
    pub(crate) fault_era: usize,
    pub(crate) fault_badv: usize,
    pub(crate) missing_syscall_active: bool,
    pub(crate) missing_syscall_id: usize,
    pub(crate) timeout_active: bool,
    pub(crate) timeout_syscalls: usize,
    pub(crate) timeout_last_syscall_id: usize,
}

pub(crate) fn reset_case_state() {
    reset_user_run_state();
    process::reset_case_process_state();
    fd_table::reset_case_fd_state();
}

fn reset_user_run_state() {
    unsafe {
        WRITE_SYSCALL_COUNT = 0;
        USER_EXITED = false;
        USER_EXIT_CODE = 0;
        USER_FAULT_ACTIVE = false;
        USER_FAULT_ECODE = 0;
        USER_FAULT_ERA = 0;
        USER_FAULT_BADV = 0;
        MISSING_SYSCALL_ACTIVE = false;
        MISSING_SYSCALL_ID = 0;
        TIMEOUT_ACTIVE = false;
        SYSCALL_BUDGET_LIMIT = 0;
        SYSCALL_BUDGET_COUNT = 0;
        SYSCALL_BUDGET_LAST_ID = 0;
    }
}

pub(crate) fn set_basic_group_active(active: bool) {
    unsafe {
        BASIC_GROUP_ACTIVE = active;
    }
}

pub(crate) fn is_basic_group_active() -> bool {
    unsafe { BASIC_GROUP_ACTIVE }
}

pub(crate) fn record_write_syscall() {
    unsafe {
        WRITE_SYSCALL_COUNT += 1;
    }
}

pub(crate) fn record_user_exit(code: usize) {
    unsafe {
        USER_EXITED = true;
        USER_EXIT_CODE = code;
    }
}

pub(crate) fn record_user_fault(ecode: usize, era: usize, badv: usize) {
    unsafe {
        USER_FAULT_ACTIVE = true;
        USER_FAULT_ECODE = ecode;
        USER_FAULT_ERA = era;
        USER_FAULT_BADV = badv;
    }
}

pub(crate) fn record_missing_syscall(id: usize) {
    unsafe {
        MISSING_SYSCALL_ACTIVE = true;
        MISSING_SYSCALL_ID = id;
    }
}

pub(crate) fn start_syscall_budget(limit: usize) {
    unsafe {
        TIMEOUT_ACTIVE = false;
        SYSCALL_BUDGET_LIMIT = limit;
        SYSCALL_BUDGET_COUNT = 0;
        SYSCALL_BUDGET_LAST_ID = 0;
    }
}

pub(crate) fn consume_syscall_budget(id: usize) -> bool {
    unsafe {
        if SYSCALL_BUDGET_LIMIT == 0 || TIMEOUT_ACTIVE {
            return false;
        }
        SYSCALL_BUDGET_COUNT = SYSCALL_BUDGET_COUNT.saturating_add(1);
        SYSCALL_BUDGET_LAST_ID = id;
        if SYSCALL_BUDGET_COUNT > SYSCALL_BUDGET_LIMIT {
            TIMEOUT_ACTIVE = true;
            true
        } else {
            false
        }
    }
}

pub(crate) fn report_missing_syscall() {
    unsafe {
        if MISSING_SYSCALL_ACTIVE {
            early_console_write("[loongarch64-basic] blocker: unsupported syscall id=");
            write_usize_dec(MISSING_SYSCALL_ID);
            early_console_write("\n");
        }
    }
}

pub(crate) fn run_snapshot() -> UserRunSnapshot {
    unsafe {
        UserRunSnapshot {
            exited: USER_EXITED,
            exit_code: USER_EXIT_CODE,
            fault_active: USER_FAULT_ACTIVE,
            fault_ecode: USER_FAULT_ECODE,
            fault_era: USER_FAULT_ERA,
            fault_badv: USER_FAULT_BADV,
            missing_syscall_active: MISSING_SYSCALL_ACTIVE,
            missing_syscall_id: MISSING_SYSCALL_ID,
            timeout_active: TIMEOUT_ACTIVE,
            timeout_syscalls: SYSCALL_BUDGET_COUNT,
            timeout_last_syscall_id: SYSCALL_BUDGET_LAST_ID,
        }
    }
}

pub(crate) fn report_user_run_status(case_name: &str, emit: bool) -> bool {
    unsafe {
        if USER_FAULT_ACTIVE && !USER_EXITED && WRITE_SYSCALL_COUNT == 0 {
            if emit {
                early_console_write(
                    "[loongarch64-basic] blocker: failed to enter PLV3 user ELF: case=",
                );
                early_console_write(case_name);
                early_console_write(" ecode=");
                write_usize_dec(USER_FAULT_ECODE);
                early_console_write(" era=");
                write_usize_hex(USER_FAULT_ERA);
                early_console_write(" badv=");
                write_usize_hex(USER_FAULT_BADV);
                early_console_write("\n");
            }
            false
        } else if USER_FAULT_ACTIVE && !USER_EXITED {
            if emit {
                early_console_write("[loongarch64-basic] blocker: user ELF trapped: case=");
                early_console_write(case_name);
                early_console_write(" ecode=");
                write_usize_dec(USER_FAULT_ECODE);
                early_console_write(" era=");
                write_usize_hex(USER_FAULT_ERA);
                early_console_write(" badv=");
                write_usize_hex(USER_FAULT_BADV);
                early_console_write("\n");
            }
            false
        } else if !USER_EXITED {
            if emit {
                early_console_write(
                    "[loongarch64-basic] blocker: user ELF returned without exit: case=",
                );
                early_console_write(case_name);
                early_console_write("\n");
            }
            false
        } else if USER_EXIT_CODE != 0 {
            if emit {
                early_console_write("[loongarch64-basic] case ");
                early_console_write(case_name);
                early_console_write(" exited with code ");
                write_usize_dec(USER_EXIT_CODE);
                early_console_write("\n");
            }
            false
        } else if MISSING_SYSCALL_ACTIVE {
            false
        } else {
            true
        }
    }
}
