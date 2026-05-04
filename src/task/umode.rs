use core::arch::asm;

const USER_STACK_SIZE: usize = 4096 * 4;

#[repr(align(16))]
struct UserStack([u8; USER_STACK_SIZE]);

static mut USER_STACK: UserStack = UserStack([0; USER_STACK_SIZE]);

#[no_mangle]
#[used]
pub static UMODE_V30B_WRITE_MARKER: &[u8] = b"hello from umode v30b syscall write
";

#[no_mangle]
#[used]
pub static UMODE_V30B_ENOSYS_MARKER: &[u8] = b"unsupported syscall returned -38
";

#[no_mangle]
#[used]
pub static UMODE_V30B_PID_MARKER: &[u8] = b"umode getpid returned 1
";

pub fn run_umode_test() -> ! {
    let entry = user_entry as *const () as usize;
    let stack_bottom = core::ptr::addr_of_mut!(USER_STACK) as usize;
    let stack_top = stack_bottom + USER_STACK_SIZE;

    crate::println!("[umode] v30b preflight begin");
    crate::println!("[umode] entry = {:#x}", entry);
    crate::println!("[umode] stack = {:#x}..{:#x}", stack_bottom, stack_top);
    crate::println!("[umode] enter user mode now");

    crate::trap::enter_user(entry, stack_top);
}

#[no_mangle]
extern "C" fn user_entry() -> ! {
    user_write(&UMODE_V30B_WRITE_MARKER);

    let pid = user_syscall0(172);
    if pid == 1 {
        user_write(&UMODE_V30B_PID_MARKER);
    } else {
        user_write(b"umode getpid unexpected value\n");
    }

    let ret = user_syscall0(9999);
    if ret == -38 {
        user_write(&UMODE_V30B_ENOSYS_MARKER);
    } else {
        user_write(b"unsupported syscall returned unexpected value\n");
    }

    user_syscall1(93, 0);

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

fn user_write(buf: &[u8]) {
    let _ = user_syscall3(64, 1, buf.as_ptr() as usize, buf.len());
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
