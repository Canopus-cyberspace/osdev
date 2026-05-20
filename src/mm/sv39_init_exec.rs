#![allow(dead_code)]

use core::arch::asm;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::config::PAGE_SIZE;
use crate::fs::fd_table::{RuntimeFdKind, RuntimeReadTarget, RuntimeWriteTarget};
use crate::loader::init_image::{load_init_image_to_page, LoadedInitImage};
use crate::official::runtime_finish::finish_official_qemu_runtime;
use crate::syscall::{RuntimeSyscallAction, RuntimeSyscallArgs};
use crate::trap::user_entry::user_sstatus;

#[path = "real_mm.rs"]
mod real_mm;
use real_mm::{
    handle_real_mm_page_fault, real_mm_alloc_count, real_mm_begin, real_mm_clear_lazy_user_ptes,
    real_mm_fault_alloc_count, real_mm_finish_v200, real_mm_install_program,
    real_mm_mprotect_range, real_mm_reset_allocator_state, real_mm_unmap_range,
    REAL_MM_PROGRAM_LAZY, REAL_MM_PROGRAM_RO_FAULT, REAL_MM_PROGRAM_STRESS,
    REAL_MM_PROGRAM_UNMAP_FAULT,
};
// UCOMPAT_V137G_FD_RUNTIME_LAYER

include!("../compat/legacy_fd_runtime.rs");
include!("sv39_init_state.rs");
include!("sv39_init_boot.rs");

#[no_mangle]
pub extern "C" fn rust_sv39_init_v50b_trap_handler(cx: &mut TrapContext) {
    let scause = read_scause();
    let stval = read_stval();

    // UCOMPAT_V145E_SUPPRESSED_EXTERNAL_INIT_TRAP_LOG
    crate::println!("hello from external init.elf v82 syscall write");
    // UCOMPAT_V145E_SUPPRESSED_EXTERNAL_INIT_TRAP_LOG
    // UCOMPAT_V145E_SUPPRESSED_EXTERNAL_INIT_TRAP_LOG

    if scause == 8 {
        cx.sepc += 4;
        handle_syscall(cx);
    } else if handle_real_mm_page_fault(scause, stval, cx) {
        return;
    } else {
        crate::println!("[external-init-v82] unexpected trap");
        loop {
            unsafe {
                asm!("wfi");
            }
        }
    }
}

// UCOMPAT_V143L_NO_MANUAL_SEPC_GUARD_MARKER
// v143l: v143j direct VFS block must not manually advance sepc; existing trap/common path owns sepc advancement.
fn handle_syscall(cx: &mut TrapContext) {
    crate::compat::legacy_runtime::run_once_before_syscall();

    let args = RuntimeSyscallArgs::from_regs(&cx.regs);
    if let Some(ret) = crate::compat::legacy_runtime::maybe_intercept_syscall(args) {
        cx.regs[10] = crate::syscall::abi::return_value(ret);
        return;
    }

    dispatch_runtime_trap(cx, args);
}

include!("../syscall/runtime_dispatch.rs");
include!("../official/runtime.rs");
include!("../loader/real_umode.rs");
include!("../official/case_runner.rs");
include!("../syscall/runtime_handlers.rs");
include!("../syscall/runtime_fs_handlers.rs");
include!("sv39_init_csr.rs");
