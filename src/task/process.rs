#![allow(dead_code)]

use core::sync::atomic::{AtomicUsize, Ordering};

use crate::loader::process_image::{build_init_process_info, ProcessInitInfo};

pub const INIT_PID: usize = 1;

static NEXT_PID: AtomicUsize = AtomicUsize::new(2);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProcessState {
    Empty,
    Ready,
    Running,
    Zombie,
}

#[derive(Copy, Clone, Debug)]
pub struct Process {
    pub pid: usize,
    pub ppid: usize,
    pub state: ProcessState,
    pub entry: usize,
    pub user_sp: usize,
    pub exit_code: isize,
    pub name: &'static str,
}

impl Process {
    pub const fn empty() -> Self {
        Self {
            pid: 0,
            ppid: 0,
            state: ProcessState::Empty,
            entry: 0,
            user_sp: 0,
            exit_code: 0,
            name: "",
        }
    }

    pub const fn is_alive(&self) -> bool {
        matches!(self.state, ProcessState::Ready | ProcessState::Running)
    }
}

pub fn alloc_pid() -> usize {
    NEXT_PID.fetch_add(1, Ordering::SeqCst)
}

pub fn make_init_process(info: ProcessInitInfo) -> Process {
    Process {
        pid: info.pid,
        ppid: info.ppid,
        state: ProcessState::Ready,
        entry: info.entry(),
        user_sp: info.user_sp_top(),
        exit_code: 0,
        name: info.argv0,
    }
}

pub fn make_zombie(mut process: Process, code: isize) -> Process {
    process.state = ProcessState::Zombie;
    process.exit_code = code;
    process
}

pub fn init() {
    crate::println!("[task::process] init");
}

pub fn self_test() {
    crate::println!("[process-v53c] self-test begin");

    match build_init_process_info() {
        Ok(info) => {
            let init = make_init_process(info);
            crate::println!("[process-v53c] init pid = {}", init.pid);
            crate::println!("[process-v53c] init ppid = {}", init.ppid);
            crate::println!("[process-v53c] init entry = {:#x}", init.entry);
            crate::println!("[process-v53c] init sp = {:#x}", init.user_sp);
            crate::println!("[process-v53c] init name = {}", init.name);

            if init.pid == INIT_PID && init.ppid == 0 && init.is_alive() && init.entry != 0 && init.user_sp != 0 {
                let next = alloc_pid();
                let zombie = make_zombie(init, 0);
                crate::println!("[process-v53c] next pid sample = {}", next);
                crate::println!("[process-v53c] zombie exit = {}", zombie.exit_code);
                crate::println!("[process-v53c] self-test passed");
            } else {
                crate::println!("[process-v53c] self-test degraded");
            }
        }
        Err(_) => {
            crate::println!("[process-v53c] self-test skipped: init image unavailable");
        }
    }
}
