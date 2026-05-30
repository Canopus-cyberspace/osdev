use super::block::BlockServices;
use super::console::FatalConsole;
use super::halt::{FatalReason, HaltReason};
use super::mmu::{KernelMmuRequest, KernelMmuState, MmuServices};
use super::readiness::HardwareReadiness;
use super::trap::{TrapInstallState, TrapServices};
use super::user_entry::{UserEntryServices, UserEntryState};
use super::BoundaryMode;
use crate::core::block::BlockProvider;
use crate::core::mm::{BootFrameAllocator, BootMemory, KernelLayout, UserAddressSpaceLoadPlan};
use crate::core::task::PendingUserEntry;

pub const BOOT_INIT_PATH_MAX: usize = 256;
pub const BOOT_INIT_ARG_MAX: usize = 128;
pub const BOOT_INIT_ARG_COUNT: usize = 7;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Architecture {
    Riscv64,
    LoongArch64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootInitPath {
    bytes: [u8; BOOT_INIT_PATH_MAX],
    len: usize,
    arg_bytes: [[u8; BOOT_INIT_ARG_MAX]; BOOT_INIT_ARG_COUNT],
    arg_lens: [usize; BOOT_INIT_ARG_COUNT],
    arg_count: usize,
}

impl BootInitPath {
    pub fn new(path: &[u8]) -> Result<Self, BootInitBlocker> {
        if path.is_empty() {
            return Err(BootInitBlocker::VfsPathMissing);
        }
        if path[0] != b'/' {
            return Err(BootInitBlocker::InitPathNotAbsolute);
        }
        if path.len() > BOOT_INIT_PATH_MAX {
            return Err(BootInitBlocker::InitPathTooLong);
        }

        let mut bytes = [0u8; BOOT_INIT_PATH_MAX];
        let mut index = 0usize;
        while index < path.len() {
            if path[index] == 0 {
                return Err(BootInitBlocker::VfsPathMissing);
            }
            bytes[index] = path[index];
            index += 1;
        }

        Ok(Self {
            bytes,
            len: path.len(),
            arg_bytes: [[0u8; BOOT_INIT_ARG_MAX]; BOOT_INIT_ARG_COUNT],
            arg_lens: [0; BOOT_INIT_ARG_COUNT],
            arg_count: 0,
        })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn push_arg(&mut self, arg: &[u8]) -> Result<(), BootInitBlocker> {
        if arg.is_empty() {
            return Err(BootInitBlocker::UnsupportedBootInput);
        }
        if self.arg_count >= BOOT_INIT_ARG_COUNT {
            return Err(BootInitBlocker::TooManyInitArgs);
        }
        if arg.len() > BOOT_INIT_ARG_MAX {
            return Err(BootInitBlocker::InitArgTooLong);
        }

        let slot = self.arg_count;
        let mut index = 0usize;
        while index < arg.len() {
            if arg[index] == 0 {
                return Err(BootInitBlocker::UnsupportedBootInput);
            }
            self.arg_bytes[slot][index] = arg[index];
            index += 1;
        }
        self.arg_lens[slot] = arg.len();
        self.arg_count += 1;
        Ok(())
    }

    pub const fn arg_count(&self) -> usize {
        self.arg_count
    }

    pub fn arg(&self, index: usize) -> Option<&[u8]> {
        if index < self.arg_count {
            Some(&self.arg_bytes[index][..self.arg_lens[index]])
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootInitBlocker {
    FirmwareTableMalformed,
    InitArgTooLong,
    InitPathNotAbsolute,
    InitPathTooLong,
    NoBootInitPath,
    TooManyInitArgs,
    UnsupportedBootInput,
    VfsPathMissing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyBootInfo {
    architecture: Architecture,
    primary_cpu_id: usize,
    firmware_arg0: usize,
    firmware_arg1: usize,
}

impl EarlyBootInfo {
    pub const fn new(
        architecture: Architecture,
        primary_cpu_id: usize,
        firmware_arg0: usize,
        firmware_arg1: usize,
    ) -> Self {
        Self {
            architecture,
            primary_cpu_id,
            firmware_arg0,
            firmware_arg1,
        }
    }

    pub const fn architecture(self) -> Architecture {
        self.architecture
    }

    pub const fn primary_cpu_id(self) -> usize {
        self.primary_cpu_id
    }

    pub const fn firmware_arg0(self) -> usize {
        self.firmware_arg0
    }

    pub const fn firmware_arg1(self) -> usize {
        self.firmware_arg1
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BspSnapshot {
    boot: EarlyBootInfo,
    console: HardwareReadiness,
    trap: HardwareReadiness,
    timer: HardwareReadiness,
    mmu: HardwareReadiness,
    user_entry: HardwareReadiness,
    block: HardwareReadiness,
}

impl BspSnapshot {
    pub const fn new(
        boot: EarlyBootInfo,
        console: HardwareReadiness,
        trap: HardwareReadiness,
        timer: HardwareReadiness,
        mmu: HardwareReadiness,
        user_entry: HardwareReadiness,
        block: HardwareReadiness,
    ) -> Self {
        Self {
            boot,
            console,
            trap,
            timer,
            mmu,
            user_entry,
            block,
        }
    }

    pub const fn boot(self) -> EarlyBootInfo {
        self.boot
    }

    pub const fn console(self) -> HardwareReadiness {
        self.console
    }

    pub const fn trap(self) -> HardwareReadiness {
        self.trap
    }

    pub const fn timer(self) -> HardwareReadiness {
        self.timer
    }

    pub const fn mmu(self) -> HardwareReadiness {
        self.mmu
    }

    pub const fn user_entry(self) -> HardwareReadiness {
        self.user_entry
    }

    pub const fn block(self) -> HardwareReadiness {
        self.block
    }
}

#[derive(Clone, Copy)]
pub struct BspServices {
    snapshot: BspSnapshot,
    fatal_console: FatalConsole,
    trap: TrapServices,
    mmu: MmuServices,
    user_entry: UserEntryServices,
    block: BlockServices,
    boot_memory: fn(EarlyBootInfo, KernelLayout) -> BootMemory,
    boot_init_path: fn(EarlyBootInfo) -> Result<BootInitPath, BootInitBlocker>,
    halt: fn(HaltReason) -> !,
}

impl BspServices {
    pub const fn new(
        snapshot: BspSnapshot,
        fatal_console: FatalConsole,
        trap: TrapServices,
        mmu: MmuServices,
        user_entry: UserEntryServices,
        block: BlockServices,
        boot_memory: fn(EarlyBootInfo, KernelLayout) -> BootMemory,
        boot_init_path: fn(EarlyBootInfo) -> Result<BootInitPath, BootInitBlocker>,
        halt: fn(HaltReason) -> !,
    ) -> Self {
        Self {
            snapshot,
            fatal_console,
            trap,
            mmu,
            user_entry,
            block,
            boot_memory,
            boot_init_path,
            halt,
        }
    }

    pub const fn snapshot(self) -> BspSnapshot {
        self.snapshot
    }

    pub fn write_fatal(self, bytes: &[u8]) -> usize {
        self.fatal_console.write(bytes)
    }

    pub const fn console_writer(self) -> fn(&[u8]) -> usize {
        self.fatal_console.writer()
    }

    pub fn install_trap_vector(self, mode: BoundaryMode) -> TrapInstallState {
        self.trap.install(mode)
    }

    pub fn activate_kernel_mmu(
        self,
        request: KernelMmuRequest,
        mode: BoundaryMode,
    ) -> KernelMmuState {
        self.mmu.activate_kernel(request, mode)
    }

    pub fn prepare_user_mmu(
        self,
        frames: &mut BootFrameAllocator,
        load: UserAddressSpaceLoadPlan<'_>,
        mode: BoundaryMode,
    ) -> super::mmu::UserMmuState {
        self.mmu.prepare_user(frames, load, mode)
    }

    pub fn enter_user(self, pending: PendingUserEntry, mode: BoundaryMode) -> UserEntryState {
        self.user_entry.enter(pending, mode)
    }

    pub const fn block_provider(self) -> BlockProvider {
        self.block.provider()
    }

    pub fn discover_boot_memory(self, layout: KernelLayout) -> BootMemory {
        (self.boot_memory)(self.snapshot.boot(), layout)
    }

    pub fn discover_boot_init_path(self) -> Result<BootInitPath, BootInitBlocker> {
        (self.boot_init_path)(self.snapshot.boot())
    }

    pub fn halt(self, reason: HaltReason) -> ! {
        (self.halt)(reason)
    }

    pub fn fatal(self, reason: FatalReason) -> ! {
        self.halt(HaltReason::Fatal(reason))
    }
}
