use crate::config::{MEMORY_END, PAGE_SIZE};
use crate::mm::page_table::{AddressSpace, PTE_R, PTE_W, PTE_X};

const RAM_BASE: usize = 0x8000_0000;
const UART0: usize = 0x1000_0000;
const UART0_END: usize = UART0 + PAGE_SIZE;

static mut SV39_V42_DATA_PROBE: usize = 0x1122_3344_5566_7788;

pub fn init() {
    crate::println!("[mm::sv39_smoke] init v42");
}

pub fn test() {
    test_scaffold();
}

pub fn test_scaffold() {
    crate::mm::sv39::test_scaffold();
    crate::println!("[mm::sv39_smoke] scaffold test passed v42");
}

pub fn run_kernel_sv39_smoke() -> ! {
    run_kernel_sv39_activation_smoke();
}

pub fn run_kernel_sv39_activation_smoke() -> ! {
    crate::println!("[sv39-v42] isolated kernel Sv39 activation begin");

    if !crate::mm::sv39::ENABLE_SV39_ACTIVATION_TEST {
        crate::println!("[sv39-v42] activation disabled");
        idle_loop();
    }

    let mut kernel_space = AddressSpace::new();

    crate::println!("[sv39-v42] map UART identity {:#x}..{:#x}", UART0, UART0_END);
    map_identity_range(&mut kernel_space, UART0, UART0_END, PTE_R | PTE_W);

    crate::println!(
        "[sv39-v42] map RAM identity {:#x}..{:#x}",
        RAM_BASE,
        MEMORY_END
    );
    map_identity_range(&mut kernel_space, RAM_BASE, MEMORY_END, PTE_R | PTE_W | PTE_X);

    let root_ppn = kernel_space.root_ppn();
    let satp = crate::mm::sv39::make_satp(root_ppn);

    crate::println!("[sv39-v42] root ppn = {:#x}", root_ppn);
    crate::println!("[sv39-v42] satp before = {:#x}", crate::mm::sv39::read_satp());
    crate::println!("[sv39-v42] satp target = {:#x}", satp);

    unsafe {
        crate::mm::sv39::activate_satp_unchecked(satp);
    }

    crate::println!("[sv39-v42] after satp write");
    crate::println!("[sv39-v42] satp after = {:#x}", crate::mm::sv39::read_satp());

    unsafe {
        core::ptr::write_volatile(core::ptr::addr_of_mut!(SV39_V42_DATA_PROBE), 0x8877_6655_4433_2211);
        let value = core::ptr::read_volatile(core::ptr::addr_of!(SV39_V42_DATA_PROBE));
        assert_eq!(value, 0x8877_6655_4433_2211);
    }

    crate::println!("[sv39-v42] data probe passed");
    crate::println!("[sv39-v42] kernel Sv39 activation passed");
    crate::println!("[sv39-v42] isolated kernel idle loop");

    idle_loop();
}

fn map_identity_range(space: &mut AddressSpace, start: usize, end: usize, flags: usize) {
    let mut va = align_down(start, PAGE_SIZE);
    let end = align_up(end, PAGE_SIZE);

    while va < end {
        space.map(va, va, flags);
        va += PAGE_SIZE;
    }
}

fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

fn idle_loop() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
