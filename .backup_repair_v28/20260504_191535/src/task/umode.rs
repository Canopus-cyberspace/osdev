use core::arch::asm;

pub const ENABLE_UMODE_TEST: bool = false;

const TEST_USER_STACK_SIZE: usize = crate::config::PAGE_SIZE * 2;

#[repr(align(16))]
struct UModeTestStack([u8; TEST_USER_STACK_SIZE]);

static mut TEST_USER_STACK: UModeTestStack = UModeTestStack([0; TEST_USER_STACK_SIZE]);

pub fn preflight() {
    crate::println!("[umode] scaffold preflight begin");

    let entry = dummy_user_entry as *const () as usize;
    let stack_bottom = core::ptr::addr_of_mut!(TEST_USER_STACK) as usize;
    let stack_top = stack_bottom + TEST_USER_STACK_SIZE;

    let cx = crate::trap::make_user_context_for_debug(entry, stack_top);
    let (sstatus, spp_is_user, spie_is_enabled) = crate::trap::debug_user_sstatus_bits();

    crate::println!("[umode] entry       = {:#x}", entry);
    crate::println!("[umode] stack       = {:#x}..{:#x}", stack_bottom, stack_top);
    crate::println!("[umode] ctx.sepc    = {:#x}", cx.sepc);
    crate::println!("[umode] ctx.sp      = {:#x}", cx.regs[2]);
    crate::println!("[umode] sstatus     = {:#x}", sstatus);

    if spp_is_user {
        crate::println!("[umode] SPP cleared");
    } else {
        crate::println!("[umode] SPP check failed");
    }

    if spie_is_enabled {
        crate::println!("[umode] SPIE set");
    } else {
        crate::println!("[umode] SPIE check failed");
    }

    crate::println!("[umode] scaffold preflight passed");
}

pub fn run() -> ! {
    let entry = dummy_user_entry as *const () as usize;
    let stack_bottom = core::ptr::addr_of_mut!(TEST_USER_STACK) as usize;
    let stack_top = stack_bottom + TEST_USER_STACK_SIZE;

    crate::println!("[umode] entering experimental U-mode");
    crate::trap::enter_user(entry, stack_top);
}

#[no_mangle]
extern "C" fn dummy_user_entry() -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
