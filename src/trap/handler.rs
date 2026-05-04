use super::TrapContext;

pub fn handle(cx: &mut TrapContext) {
    let scause = read_scause();
    let stval = read_stval();

    crate::println!("[trap] scause = {:#x}", scause);
    crate::println!("[trap] sepc   = {:#x}", cx.sepc);
    crate::println!("[trap] stval  = {:#x}", stval);

    match scause {
        8 => handle_user_ecall(cx),
        _ => {
            crate::println!("[trap] unsupported trap");
            loop {
                unsafe {
                    core::arch::asm!("wfi");
                }
            }
        }
    }
}

fn handle_user_ecall(cx: &mut TrapContext) {
    let syscall_id = cx.regs[17];
    let args = [
        cx.regs[10],
        cx.regs[11],
        cx.regs[12],
        cx.regs[13],
        cx.regs[14],
        cx.regs[15],
    ];

    crate::println!("[trap] user syscall id = {}", syscall_id);

    let ret = crate::syscall::syscall(syscall_id, args);
    cx.regs[10] = ret as usize;
    cx.sepc += 4;
}

fn read_scause() -> usize {
    let value: usize;
    unsafe {
        core::arch::asm!("csrr {}, scause", out(reg) value);
    }
    value
}

fn read_stval() -> usize {
    let value: usize;
    unsafe {
        core::arch::asm!("csrr {}, stval", out(reg) value);
    }
    value
}
