use core::sync::atomic::{AtomicUsize, Ordering};

use crate::arch::contract::{
    BoundaryMode, FatalReason, HaltReason, HardwareReadiness, ReadinessReason, TrapInstallState,
    TrapVector,
};
use crate::core::mm::{copy_to_user, PageTableRoot, PhysFrame};
use crate::core::syscall::{ForkRequest, SyscallError, SyscallFrame, SyscallOutcome};
use crate::core::task::{
    single_begin_child, single_enter_child, single_enter_pid, single_is_active_child, single_pid,
    single_set_tid_address, ExitState, Process,
};
use crate::kernel::syscall_runtime::{
    active_runtime_snapshot, dispatch_single_with_memory, dispatch_with_memory,
    restore_active_runtime_snapshot, ActiveRuntimeSnapshot,
};

#[cfg(target_arch = "riscv64")]
core::arch::global_asm!(
    r#"
    .section .text.trap, "ax"
    .align 2
    .globl __riscv64_trap_vector
__riscv64_trap_vector:
    csrrw sp, sscratch, sp
    addi sp, sp, -16
    sd t6, 0(sp)
    la t6, RISCV64_USER_TRAP_FRAME

    sd ra, 8(t6)
    sd t5, 240(t6)
    csrr t5, sscratch
    sd t5, 16(t6)
    la t5, __riscv64_user_trap_stack_top
    csrw sscratch, t5
    sd gp, 24(t6)
    sd tp, 32(t6)
    sd t0, 40(t6)
    sd t1, 48(t6)
    sd t2, 56(t6)
    sd s0, 64(t6)
    sd s1, 72(t6)
    sd a0, 80(t6)
    sd a1, 88(t6)
    sd a2, 96(t6)
    sd a3, 104(t6)
    sd a4, 112(t6)
    sd a5, 120(t6)
    sd a6, 128(t6)
    sd a7, 136(t6)
    sd s2, 144(t6)
    sd s3, 152(t6)
    sd s4, 160(t6)
    sd s5, 168(t6)
    sd s6, 176(t6)
    sd s7, 184(t6)
    sd s8, 192(t6)
    sd s9, 200(t6)
    sd s10, 208(t6)
    sd s11, 216(t6)
    sd t3, 224(t6)
    sd t4, 232(t6)
    ld t5, 0(sp)
    sd t5, 248(t6)

    csrr t5, sepc
    sd t5, 256(t6)
    csrr t5, sstatus
    sd t5, 264(t6)
    csrr t5, scause
    sd t5, 272(t6)
    csrr t5, stval
    sd t5, 280(t6)

    mv a0, t6
    call riscv64_user_trap_dispatch
    bnez a0, 2f

    la t6, RISCV64_USER_TRAP_FRAME
    ld t5, 256(t6)
    csrw sepc, t5
    ld t5, 264(t6)
    csrw sstatus, t5
    la t5, __riscv64_user_trap_stack_top
    csrw sscratch, t5

    ld ra, 8(t6)
    ld gp, 24(t6)
    ld tp, 32(t6)
    ld t0, 40(t6)
    ld t1, 48(t6)
    ld t2, 56(t6)
    ld s0, 64(t6)
    ld s1, 72(t6)
    ld a0, 80(t6)
    ld a1, 88(t6)
    ld a2, 96(t6)
    ld a3, 104(t6)
    ld a4, 112(t6)
    ld a5, 120(t6)
    ld a6, 128(t6)
    ld a7, 136(t6)
    ld s2, 144(t6)
    ld s3, 152(t6)
    ld s4, 160(t6)
    ld s5, 168(t6)
    ld s6, 176(t6)
    ld s7, 184(t6)
    ld s8, 192(t6)
    ld s9, 200(t6)
    ld s10, 208(t6)
    ld s11, 216(t6)
    ld t3, 224(t6)
    ld t4, 232(t6)
    ld t5, 240(t6)
    ld sp, 16(t6)
    ld t6, 248(t6)
    sret
2:
    call riscv64_user_trap_stop
1:
    wfi
    j 1b

    .section .bss.trap, "aw", @nobits
    .align 12
    .globl __riscv64_user_trap_stack
__riscv64_user_trap_stack:
    .space 65536
    .globl __riscv64_user_trap_stack_top
__riscv64_user_trap_stack_top:
"#
);

#[cfg(target_arch = "riscv64")]
unsafe extern "C" {
    fn __riscv64_trap_vector();
    fn __riscv64_user_trap_stack_top();
}

const SSTATUS_SPP: usize = 1 << 8;
const USER_TRAP_ACTION_RETURN: usize = 0;
const USER_TRAP_ACTION_EXIT: usize = 1;
const USER_TRAP_ACTION_FATAL: usize = 2;
const PENDING_PARENT_DEPTH: usize = 8;

macro_rules! atomic_usize_parent_slots {
    () => {
        [
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
            AtomicUsize::new(0),
        ]
    };
}

#[cfg(target_arch = "riscv64")]
#[no_mangle]
static mut RISCV64_USER_TRAP_FRAME: RiscvUserTrapFrame = RiscvUserTrapFrame::empty();

static mut PENDING_PARENT_TRAP_FRAMES: [RiscvUserTrapFrame; PENDING_PARENT_DEPTH] =
    [RiscvUserTrapFrame::empty(); PENDING_PARENT_DEPTH];

static PENDING_PARENT_ROOT_FRAMES: [AtomicUsize; PENDING_PARENT_DEPTH] =
    atomic_usize_parent_slots!();

static PENDING_PARENT_PIDS: [AtomicUsize; PENDING_PARENT_DEPTH] = atomic_usize_parent_slots!();

static mut PENDING_PARENT_RUNTIMES: [ActiveRuntimeSnapshot; PENDING_PARENT_DEPTH] =
    [ActiveRuntimeSnapshot::empty(); PENDING_PARENT_DEPTH];

static PENDING_PARENT_STACK_DEPTH: AtomicUsize = AtomicUsize::new(0);

pub const fn readiness() -> HardwareReadiness {
    HardwareReadiness::NotReady(ReadinessReason::TrapVectorNotInstalled)
}

pub fn install_trap_vector(mode: BoundaryMode) -> TrapInstallState {
    let vector = match trap_vector() {
        Ok(vector) => vector,
        Err(blocker) => return TrapInstallState::NotReady(blocker),
    };

    match mode {
        BoundaryMode::Inspect => TrapInstallState::Planned(vector),
        BoundaryMode::Prepare => TrapInstallState::Prepared(vector),
        BoundaryMode::ApplyUnsafe => apply_trap_vector(vector),
    }
}

#[cfg(target_arch = "riscv64")]
fn trap_vector() -> Result<TrapVector, crate::arch::contract::TrapVectorBlocker> {
    TrapVector::new(__riscv64_trap_vector as *const () as usize)
}

#[cfg(not(target_arch = "riscv64"))]
fn trap_vector() -> Result<TrapVector, crate::arch::contract::TrapVectorBlocker> {
    TrapVector::new(0)
}

#[cfg(target_arch = "riscv64")]
fn apply_trap_vector(vector: TrapVector) -> TrapInstallState {
    unsafe {
        write_sscratch(__riscv64_user_trap_stack_top as *const () as usize);
        write_stvec(vector.address());
    }

    TrapInstallState::Applied(vector)
}

#[cfg(not(target_arch = "riscv64"))]
fn apply_trap_vector(_vector: TrapVector) -> TrapInstallState {
    TrapInstallState::Unsupported(
        crate::arch::contract::TrapVectorUnsupported::HardwareExecutionNotVerified,
    )
}

#[cfg(target_arch = "riscv64")]
unsafe fn write_stvec(address: usize) {
    core::arch::asm!("csrw stvec, {0}", in(reg) address, options(nostack, preserves_flags));
}

#[cfg(target_arch = "riscv64")]
unsafe fn write_sscratch(address: usize) {
    core::arch::asm!("csrw sscratch, {0}", in(reg) address, options(nostack, preserves_flags));
}

#[cfg_attr(target_arch = "riscv64", no_mangle)]
pub extern "C" fn riscv64_trap_entry(scause: usize, sepc: usize, stval: usize) -> ! {
    let cause = RiscvTrapCause::decode(scause);
    let _trap = FatalTrap {
        cause,
        exception_pc: sepc,
        trap_value: stval,
    };

    super::halt::halt(HaltReason::Fatal(FatalReason::Trap))
}

#[cfg_attr(target_arch = "riscv64", no_mangle)]
pub extern "C" fn riscv64_user_trap_dispatch(frame: *mut RiscvUserTrapFrame) -> usize {
    if frame.is_null() {
        return USER_TRAP_ACTION_FATAL;
    }

    let trap = unsafe { &mut *frame };
    match dispatch_user_trap(trap) {
        RiscvTrapAction::ReturnToUser => USER_TRAP_ACTION_RETURN,
        RiscvTrapAction::ProcessExited(_) => USER_TRAP_ACTION_EXIT,
        RiscvTrapAction::UnsupportedTrap(_) => USER_TRAP_ACTION_FATAL,
    }
}

#[cfg_attr(target_arch = "riscv64", no_mangle)]
pub extern "C" fn riscv64_user_trap_stop(action: usize) -> ! {
    if action == USER_TRAP_ACTION_EXIT {
        super::halt::halt(HaltReason::NoRunnableWork)
    } else {
        super::halt::halt(HaltReason::Fatal(FatalReason::Trap))
    }
}

pub fn handle_user_ecall(trap: &mut RiscvUserTrapFrame, process: &mut Process) -> RiscvTrapAction {
    let syscall = trap.syscall_frame();
    let memory = super::mmu::ActiveUserMemory;
    match dispatch_with_memory(syscall, process, &memory) {
        SyscallOutcome::Return(value) => {
            trap.advance_after_ecall();
            trap.write_return(value);
            RiscvTrapAction::ReturnToUser
        }
        SyscallOutcome::Exit(exit) => RiscvTrapAction::ProcessExited(exit),
        SyscallOutcome::Fork(request) => start_child_after_fork(trap, request),
        SyscallOutcome::Exec(request) => start_exec_from_syscall(trap, request),
    }
}

pub fn dispatch_user_trap(trap: &mut RiscvUserTrapFrame) -> RiscvTrapAction {
    if !trap.is_from_user() {
        return RiscvTrapAction::UnsupportedTrap(RiscvTrapBlocker::SupervisorTrap);
    }

    match trap.cause() {
        RiscvTrapCause::Exception(RiscvException::EnvironmentCallFromUser) => {
            let syscall = trap.syscall_frame();
            let memory = super::mmu::ActiveUserMemory;
            match dispatch_single_with_memory(syscall, &memory) {
                SyscallOutcome::Return(value) => {
                    trap.advance_after_ecall();
                    trap.write_return(value);
                    RiscvTrapAction::ReturnToUser
                }
                SyscallOutcome::Exit(exit) => {
                    if resume_parent_after_child_exit(trap, exit) {
                        RiscvTrapAction::ReturnToUser
                    } else {
                        RiscvTrapAction::ProcessExited(exit)
                    }
                }
                SyscallOutcome::Fork(request) => start_child_after_fork(trap, request),
                SyscallOutcome::Exec(request) => start_exec_from_syscall(trap, request),
            }
        }
        _ => RiscvTrapAction::UnsupportedTrap(RiscvTrapBlocker::UnsupportedTrapCause),
    }
}

fn start_exec_from_syscall(
    trap: &mut RiscvUserTrapFrame,
    request: crate::core::syscall::ExecRequest,
) -> RiscvTrapAction {
    match crate::kernel::syscall_runtime::exec_from_syscall(request) {
        Ok(pending) => {
            trap.write_exec_entry(pending);
            RiscvTrapAction::ReturnToUser
        }
        Err(errno) => {
            trap.advance_after_ecall();
            trap.write_return(errno);
            RiscvTrapAction::ReturnToUser
        }
    }
}

fn start_child_after_fork(trap: &mut RiscvUserTrapFrame, request: ForkRequest) -> RiscvTrapAction {
    let parent_depth = PENDING_PARENT_STACK_DEPTH.load(Ordering::Acquire);
    if parent_depth >= PENDING_PARENT_DEPTH {
        trap.advance_after_ecall();
        trap.write_return(SyscallError::NoMemory.errno());
        return RiscvTrapAction::ReturnToUser;
    }

    let parent_root = match super::mmu::active_user_root() {
        Ok(root) => root,
        Err(error) => {
            trap.advance_after_ecall();
            trap.write_return(map_user_map_error(error));
            return RiscvTrapAction::ReturnToUser;
        }
    };
    let child_root = match super::mmu::clone_active_user_root() {
        Ok(root) => root,
        Err(error) => {
            trap.advance_after_ecall();
            trap.write_return(map_user_map_error(error));
            return RiscvTrapAction::ReturnToUser;
        }
    };
    let child_pid = match single_begin_child() {
        Some(pid) => pid,
        None => {
            trap.advance_after_ecall();
            trap.write_return(SyscallError::NoMemory.errno());
            return RiscvTrapAction::ReturnToUser;
        }
    };

    let mut parent_frame = *trap;
    parent_frame.advance_after_ecall();
    parent_frame.write_return(child_pid.value() as isize);
    let parent_runtime = active_runtime_snapshot();

    let mut child_frame = *trap;
    child_frame.advance_after_ecall();
    child_frame.write_return(0);
    if request.child_stack() != 0 {
        child_frame.write_stack_pointer(request.child_stack());
    }

    unsafe {
        core::ptr::addr_of_mut!(PENDING_PARENT_TRAP_FRAMES)
            .cast::<RiscvUserTrapFrame>()
            .add(parent_depth)
            .write(parent_frame);
    }
    PENDING_PARENT_ROOT_FRAMES[parent_depth].store(parent_root.frame().start(), Ordering::Release);
    PENDING_PARENT_PIDS[parent_depth].store(single_pid().value(), Ordering::Release);
    unsafe {
        core::ptr::addr_of_mut!(PENDING_PARENT_RUNTIMES)
            .cast::<ActiveRuntimeSnapshot>()
            .add(parent_depth)
            .write(parent_runtime);
    }
    PENDING_PARENT_STACK_DEPTH.store(parent_depth + 1, Ordering::Release);
    single_enter_child(child_pid);
    super::mmu::switch_to_user_root(child_root);
    if request.clear_child_tid() {
        single_set_tid_address(request.child_tid());
    }
    if request.set_child_tid() {
        let tid = (child_pid.value() as u32).to_le_bytes();
        let memory = super::mmu::ActiveUserMemory;
        let _ = copy_to_user(&memory, request.child_tid(), &tid);
    }
    *trap = child_frame;
    RiscvTrapAction::ReturnToUser
}

fn resume_parent_after_child_exit(trap: &mut RiscvUserTrapFrame, exit: ExitState) -> bool {
    if !single_is_active_child(exit.pid()) {
        return false;
    }

    let depth = PENDING_PARENT_STACK_DEPTH.load(Ordering::Acquire);
    if depth == 0 {
        return false;
    }
    let index = depth - 1;
    let parent_frame = unsafe {
        core::ptr::addr_of!(PENDING_PARENT_TRAP_FRAMES)
            .cast::<RiscvUserTrapFrame>()
            .add(index)
            .read()
    };
    let parent_root =
        match PhysFrame::new(PENDING_PARENT_ROOT_FRAMES[index].swap(0, Ordering::AcqRel)) {
            Ok(frame) => PageTableRoot::new(frame),
            Err(_) => return false,
        };
    let parent_pid = PENDING_PARENT_PIDS[index].swap(0, Ordering::AcqRel);
    let parent_runtime = unsafe {
        core::ptr::addr_of!(PENDING_PARENT_RUNTIMES)
            .cast::<ActiveRuntimeSnapshot>()
            .add(index)
            .read()
    };
    PENDING_PARENT_STACK_DEPTH.store(index, Ordering::Release);
    single_enter_pid(crate::core::task::Pid::new(parent_pid));
    restore_active_runtime_snapshot(parent_runtime);
    super::mmu::switch_to_user_root(parent_root);
    *trap = parent_frame;
    true
}

const fn map_user_map_error(error: crate::core::mm::UserMapError) -> isize {
    match error {
        crate::core::mm::UserMapError::FrameExhausted => SyscallError::NoMemory.errno(),
        crate::core::mm::UserMapError::NotReady => SyscallError::NoDevice.errno(),
        crate::core::mm::UserMapError::Unsupported => SyscallError::NotSupported.errno(),
        crate::core::mm::UserMapError::AddressOverflow
        | crate::core::mm::UserMapError::AlreadyMapped
        | crate::core::mm::UserMapError::InvalidRange
        | crate::core::mm::UserMapError::PermissionDenied => SyscallError::Invalid.errno(),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RiscvTrapCause {
    Exception(RiscvException),
    Interrupt(RiscvInterrupt),
}

impl RiscvTrapCause {
    const fn decode(scause: usize) -> Self {
        let interrupt = (scause & (1usize << (usize::BITS - 1))) != 0;
        let code = scause & !(1usize << (usize::BITS - 1));

        if interrupt {
            Self::Interrupt(RiscvInterrupt::from_code(code))
        } else {
            Self::Exception(RiscvException::from_code(code))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RiscvException {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EnvironmentCallFromUser,
    EnvironmentCallFromSupervisor,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    Other(usize),
}

impl RiscvException {
    const fn from_code(code: usize) -> Self {
        match code {
            0 => Self::InstructionAddressMisaligned,
            1 => Self::InstructionAccessFault,
            2 => Self::IllegalInstruction,
            3 => Self::Breakpoint,
            4 => Self::LoadAddressMisaligned,
            5 => Self::LoadAccessFault,
            6 => Self::StoreAddressMisaligned,
            7 => Self::StoreAccessFault,
            8 => Self::EnvironmentCallFromUser,
            9 => Self::EnvironmentCallFromSupervisor,
            12 => Self::InstructionPageFault,
            13 => Self::LoadPageFault,
            15 => Self::StorePageFault,
            _ => Self::Other(code),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RiscvInterrupt {
    SupervisorSoftware,
    SupervisorTimer,
    SupervisorExternal,
    Other(usize),
}

impl RiscvInterrupt {
    const fn from_code(code: usize) -> Self {
        match code {
            1 => Self::SupervisorSoftware,
            5 => Self::SupervisorTimer,
            9 => Self::SupervisorExternal,
            _ => Self::Other(code),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FatalTrap {
    cause: RiscvTrapCause,
    exception_pc: usize,
    trap_value: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct RiscvUserTrapFrame {
    registers: [usize; 32],
    exception_pc: usize,
    status: usize,
    cause: usize,
    trap_value: usize,
}

impl RiscvUserTrapFrame {
    pub const fn empty() -> Self {
        Self {
            registers: [0; 32],
            exception_pc: 0,
            status: 0,
            cause: 0,
            trap_value: 0,
        }
    }

    pub const fn new(
        registers: [usize; 32],
        exception_pc: usize,
        status: usize,
        cause: usize,
        trap_value: usize,
    ) -> Self {
        Self {
            registers,
            exception_pc,
            status,
            cause,
            trap_value,
        }
    }

    pub const fn exception_pc(self) -> usize {
        self.exception_pc
    }

    pub const fn register(self, index: usize) -> usize {
        self.registers[index]
    }

    pub const fn status(self) -> usize {
        self.status
    }

    pub const fn raw_cause(self) -> usize {
        self.cause
    }

    pub const fn trap_value(self) -> usize {
        self.trap_value
    }

    pub const fn is_from_user(self) -> bool {
        self.status & SSTATUS_SPP == 0
    }

    pub fn cause(&self) -> RiscvTrapCause {
        RiscvTrapCause::decode(self.cause)
    }

    pub fn syscall_frame(&self) -> SyscallFrame {
        SyscallFrame::new(
            self.registers[17],
            [
                self.registers[10],
                self.registers[11],
                self.registers[12],
                self.registers[13],
                self.registers[14],
                self.registers[15],
            ],
        )
    }

    pub fn advance_after_ecall(&mut self) {
        self.exception_pc = self.exception_pc.saturating_add(4);
    }

    pub fn write_return(&mut self, value: isize) {
        self.registers[10] = value as usize;
    }

    pub fn write_stack_pointer(&mut self, value: usize) {
        self.registers[2] = value;
    }

    pub fn write_exec_entry(&mut self, pending: crate::core::task::PendingUserEntry) {
        let address_space = pending.address_space();
        let layout = address_space.plan();
        let registers = pending.registers();
        self.exception_pc = layout.entry().value();
        self.write_stack_pointer(layout.initial_stack_pointer());
        self.registers[10] = registers.arg0();
        self.registers[11] = registers.arg1();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RiscvTrapAction {
    ReturnToUser,
    ProcessExited(ExitState),
    UnsupportedTrap(RiscvTrapBlocker),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RiscvTrapBlocker {
    SupervisorTrap,
    UnsupportedTrapCause,
}
