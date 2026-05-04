use core::arch::asm;

const USER_STACK_SIZE: usize = crate::config::USER_STACK_SIZE;

#[repr(align(16))]
struct UserStack([u8; USER_STACK_SIZE]);

static mut USER_STACK: UserStack = UserStack([0; USER_STACK_SIZE]);

#[used]
pub static UMODE_V32_WRITE_MARKER: &[u8] = b"hello from umode v32 syscall write\n";

#[used]
pub static UMODE_V32_ZERO_WRITE_MARKER: &[u8] = b"zero length write returned 0\n";

#[used]
pub static UMODE_V32_PID_MARKER: &[u8] = b"umode getpid returned 1\n";

#[used]
pub static UMODE_V32_PPID_MARKER: &[u8] = b"umode getppid returned 0\n";

#[used]
pub static UMODE_V32_ENOSYS_MARKER: &[u8] = b"unsupported syscall returned -38\n";

pub fn run_umode_test() -> ! {
    let user_entry_addr = user_entry as *const () as usize;
    let user_stack_bottom = core::ptr::addr_of_mut!(USER_STACK) as usize;
    let user_stack_top = user_stack_bottom + USER_STACK_SIZE;

    crate::println!("[umode] enter v32 user-copy bounds test");
    crate::trap::enter_user(user_entry_addr, user_stack_top);
}

#[no_mangle]
extern "C" fn user_entry() -> ! {
    let msg = b"hello from umode v32 syscall write\n";
    user_syscall3(64, 1, msg.as_ptr() as usize, msg.len());

    let zero = b"this should not be printed by zero length write\n";
    let zero_ret = user_syscall3(64, 1, zero.as_ptr() as usize, 0);
    if zero_ret == 0 {
        let ok = b"zero length write returned 0\n";
        user_syscall3(64, 1, ok.as_ptr() as usize, ok.len());
    }

    let pid = user_syscall0(172);
    if pid == 1 {
        let ok = b"umode getpid returned 1\n";
        user_syscall3(64, 1, ok.as_ptr() as usize, ok.len());
    }

    let ppid = user_syscall0(173);
    if ppid == 0 {
        let ok = b"umode getppid returned 0\n";
        user_syscall3(64, 1, ok.as_ptr() as usize, ok.len());
    }

    let unsupported = user_syscall0(9999);
    if unsupported == -38 {
        let ok = b"unsupported syscall returned -38\n";
        user_syscall3(64, 1, ok.as_ptr() as usize, ok.len());
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
