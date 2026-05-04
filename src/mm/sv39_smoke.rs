pub const ENABLE_KERNEL_SV39_SMOKE: bool = false;

pub fn init() {
    crate::println!("[mm::sv39_smoke] scaffold init v41d");
}

pub fn test() {
    test_scaffold();
}

pub fn test_scaffold() {
    crate::println!("[sv39-smoke-v41d] scaffold begin");

    if ENABLE_KERNEL_SV39_SMOKE {
        crate::println!("[sv39-smoke-v41d] kernel Sv39 smoke flag enabled");
    } else {
        crate::println!("[sv39-smoke-v41d] activation disabled");
    }

    crate::println!("[sv39-smoke-v41d] scaffold passed");
}

pub fn run_kernel_sv39_smoke() -> ! {
    crate::println!("[sv39-smoke-v41d] run_kernel_sv39_smoke entered");

    if ENABLE_KERNEL_SV39_SMOKE {
        run_kernel_sv39_activation_smoke();
    }

    crate::println!("[sv39-smoke-v41d] activation disabled; entering kernel idle loop");
    idle_loop();
}

pub fn run_kernel_sv39_activation_smoke() -> ! {
    crate::println!("[sv39-smoke-v41d] run_kernel_sv39_activation_smoke entered");
    crate::println!("[sv39-smoke-v41d] real satp activation intentionally disabled in v41d");
    idle_loop();
}

fn idle_loop() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
