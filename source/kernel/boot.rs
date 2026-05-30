use core::sync::atomic::{AtomicUsize, Ordering};

use crate::arch::contract::{
    BootInitPath, BoundaryMode, BspServices, HaltReason, KernelMmuRequest, BOOT_INIT_ARG_COUNT,
};
use crate::core::mm::{KernelGlobalMappings, MemoryFoundation};
use crate::kernel::exec::drive_boot_init_exec;

#[no_mangle]
static BOOT_STAGE: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
static INIT_PATH_LEN: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
static INIT_PATH_BYTES: [AtomicUsize; 32] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
];

#[no_mangle]
static INIT_ARG_COUNT: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
static INIT_ARG_LEN: [AtomicUsize; BOOT_INIT_ARG_COUNT] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
];

#[no_mangle]
static INIT_ARG_BYTES: [AtomicUsize; 32] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
];

pub fn kernel_start(bsp: BspServices) -> ! {
    BOOT_STAGE.store(1, Ordering::Relaxed);
    crate::official::user_output::install_user_output_writer(bsp.console_writer());
    let kernel_layout = match crate::kernel::linker::kernel_layout() {
        Ok(layout) => layout,
        Err(_) => bsp.fatal(crate::arch::contract::FatalReason::InvalidKernelImageRange),
    };

    let trap = bsp.install_trap_vector(BoundaryMode::ApplyUnsafe);
    BOOT_STAGE.store(2, Ordering::Relaxed);
    let kernel_globals = KernelGlobalMappings::required(kernel_layout.image());
    let boot_memory = bsp.discover_boot_memory(kernel_layout);
    let mmu_request = KernelMmuRequest::new(kernel_layout, kernel_globals, trap.vector());
    let mmu = bsp.activate_kernel_mmu(mmu_request, BoundaryMode::ApplyUnsafe);
    BOOT_STAGE.store(3, Ordering::Relaxed);
    let mut memory = MemoryFoundation::from_kernel_mmu(kernel_globals, mmu.root(), boot_memory);
    let boot_init_path = bsp.discover_boot_init_path();

    match &boot_init_path {
        Ok(path) => {
            record_init_trace(path);
            BOOT_STAGE.store(5, Ordering::Relaxed);
        }
        Err(_) => {
            BOOT_STAGE.store(14, Ordering::Relaxed);
        }
    }

    let _boot_init_exec = match boot_init_path {
        Ok(init_path) => Some(drive_boot_init_exec(bsp, &mut memory, init_path)),
        Err(_) => None,
    };
    let _closure_summary = (
        bsp.snapshot(),
        boot_memory,
        trap.readiness(),
        mmu.readiness(),
        memory.summary(),
        _boot_init_exec,
    );

    bsp.halt(HaltReason::NoRunnableWork)
}

fn record_init_trace(path: &BootInitPath) {
    let bytes = path.bytes();
    INIT_PATH_LEN.store(bytes.len(), Ordering::Relaxed);
    let mut i = 0usize;
    while i < bytes.len() && i < INIT_PATH_BYTES.len() * core::mem::size_of::<usize>() {
        store_packed_byte(&INIT_PATH_BYTES, i, bytes[i]);
        i += 1;
    }

    INIT_ARG_COUNT.store(path.arg_count(), Ordering::Relaxed);
    let mut arg_index = 0usize;
    let mut byte_offset = 0usize;
    while arg_index < path.arg_count() && arg_index < BOOT_INIT_ARG_COUNT {
        if let Some(arg) = path.arg(arg_index) {
            INIT_ARG_LEN[arg_index].store(arg.len(), Ordering::Relaxed);
            let mut index = 0usize;
            while index < arg.len()
                && byte_offset < INIT_ARG_BYTES.len() * core::mem::size_of::<usize>()
            {
                store_packed_byte(&INIT_ARG_BYTES, byte_offset, arg[index]);
                byte_offset += 1;
                index += 1;
            }
            byte_offset += 1;
        }
        arg_index += 1;
    }
}

fn store_packed_byte(slots: &[AtomicUsize], offset: usize, byte: u8) {
    let width = core::mem::size_of::<usize>();
    let word_offset = offset / width;
    if word_offset >= slots.len() {
        return;
    }
    let byte_offset = offset % width;
    slots[word_offset].fetch_or((byte as usize) << (byte_offset * 8), Ordering::Relaxed);
}
