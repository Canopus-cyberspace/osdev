use crate::arch::contract::{
    Architecture, BootInitBlocker, BootInitPath, BspServices, BspSnapshot, EarlyBootInfo,
    FatalConsole, HardwareReadiness, MmuServices, TrapServices, UserEntryServices,
};
use crate::core::mm::{BootMemory, KernelLayout};

pub fn early_boot_info(arg0: usize, arg1: usize) -> EarlyBootInfo {
    EarlyBootInfo::new(Architecture::LoongArch64, arg0, arg0, arg1)
}

pub fn bsp_services(boot: EarlyBootInfo) -> BspServices {
    let snapshot = BspSnapshot::new(
        boot,
        HardwareReadiness::Ready,
        super::trap::readiness(),
        super::timer::readiness(),
        super::mmu::readiness(),
        super::user_entry::readiness(),
        super::block::readiness(),
    );

    BspServices::new(
        snapshot,
        FatalConsole::new(super::console::write_fatal),
        TrapServices::new(super::trap::install_trap_vector),
        MmuServices::new(
            super::mmu::activate_kernel_address_space,
            super::mmu::prepare_user_address_space,
        ),
        UserEntryServices::new(super::user_entry::enter_user),
        crate::arch::contract::BlockServices::new(super::block::read_sector),
        discover_boot_memory,
        discover_boot_init_path,
        super::halt::halt,
    )
}

pub fn discover_boot_memory(_boot: EarlyBootInfo, _layout: KernelLayout) -> BootMemory {
    BootMemory::discovery_required()
}

pub fn discover_boot_init_path(_boot: EarlyBootInfo) -> Result<BootInitPath, BootInitBlocker> {
    Err(BootInitBlocker::UnsupportedBootInput)
}
