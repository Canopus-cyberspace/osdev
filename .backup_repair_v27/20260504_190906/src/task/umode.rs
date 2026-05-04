use core::arch::asm;

/// U-mode 恢复开关。
/// 当前阶段先保持 false，确保完整机制骨架稳定。
/// 下一阶段再单独把它改成 true，并修 trap / sret / 用户栈路径。
pub const ENABLE_UMODE_TEST: bool = false;

#[repr(align(16))]
struct UserStack([u8; crate::config::USER_STACK_SIZE]);

static mut USER_STACK: UserStack = UserStack([0; crate::config::USER_STACK_SIZE]);

pub fn run_raw_user_task() -> ! {
    let user_entry_addr = user_entry as *const () as usize;
    let user_stack_bottom = core::ptr::addr_of_mut!(USER_STACK) as usize;
    let user_stack_top = user_stack_bottom + crate::config::USER_STACK_SIZE;

    crate::println!("[umode] enter raw U-mode test");
    crate::trap::enter_user(user_entry_addr, user_stack_top);
}

#[no_mangle]
extern "C" fn user_entry() -> ! {
    let msg = b"hello from raw U-mode syscall write\n";
    user_syscall3(64, 1, msg.as_ptr() as usize, msg.len());

    let pid = user_syscall0(172);

    if pid == 1 {
        let ok = b"raw U-mode getpid returned 1\n";
        user_syscall3(64, 1, ok.as_ptr() as usize, ok.len());
    } else {
        let bad = b"raw U-mode getpid returned unexpected value\n";
        user_syscall3(64, 1, bad.as_ptr() as usize, bad.len());
    }

    user_syscall1(93, 0);

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

fn user_syscall0(id: usize) -> isize {
    let ret: isize;

    unsafe {
        asm!(
            "ecall",
            lateout("a0") ret,
            in("a7") id,
        );
    }

    ret
}

fn user_syscall1(id: usize, a0: usize) -> isize {
    let ret: isize;

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") a0 => ret,
            in("a7") id,
        );
    }

    ret
}

fn user_syscall3(id: usize, a0: usize, a1: usize, a2: usize) -> isize {
    let ret: isize;

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") a0 => ret,
            in("a1") a1,
            in("a2") a2,
            in("a7") id,
        );
    }

    ret
}
