use core::arch::asm;

const USER_STACK_SIZE: usize = 4096 * 4;

#[repr(align(16))]
struct UserStack([u8; USER_STACK_SIZE]);

static mut USER_STACK: UserStack = UserStack([0; USER_STACK_SIZE]);

pub fn run_umode_smoke_test() -> ! {
    let entry = user_entry as *const () as usize;
    let stack_bottom = core::ptr::addr_of_mut!(USER_STACK) as usize;
    let stack_top = stack_bottom + USER_STACK_SIZE;

    crate::println!("[umode] enter user mode smoke v29");
    crate::println!("[umode] entry = {:#x}", entry);
    crate::println!("[umode] stack = {:#x}..{:#x}", stack_bottom, stack_top);

    crate::trap::enter_user(entry, stack_top);
}

#[no_mangle]
extern "C" fn user_entry() -> ! {
    let msg = b"hello from user mode v29\n";
    user_syscall3(64, 1, msg.as_ptr() as usize, msg.len());

    let pid = user_syscall0(172);

    if pid == 1 {
        let ok = b"getpid returned 1 from user mode v29\n";
        user_syscall3(64, 1, ok.as_ptr() as usize, ok.len());
    } else {
        let bad = b"getpid returned unexpected value from user mode v29\n";
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
