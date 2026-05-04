#![allow(dead_code)]

use crate::config::{PAGE_SIZE, USER_STACK_SIZE};
use crate::loader::elf::ElfError;
use crate::loader::init_image::{load_init_image_to_page, LoadedInitImage};

pub const INIT_PID: usize = 1;
pub const INIT_PPID: usize = 0;
pub const INIT_USER_STACK_TOP: usize = 0x4002_0000;
pub const INIT_USER_STACK_BOTTOM: usize = INIT_USER_STACK_TOP - USER_STACK_SIZE;
pub const INIT_ARGV0: &str = "/init";

#[derive(Copy, Clone, Debug)]
pub struct UserProgram {
    pub name: &'static str,
    pub image: LoadedInitImage,
}

impl UserProgram {
    pub const fn entry(&self) -> usize {
        self.image.entry
    }

    pub const fn load_vaddr(&self) -> usize {
        self.image.vaddr
    }

    pub const fn load_pa(&self) -> usize {
        self.image.load_pa
    }

    pub const fn page_count(&self) -> usize {
        self.image.page_count
    }

    pub const fn mem_size(&self) -> usize {
        self.image.memsz
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ProcessInitInfo {
    pub pid: usize,
    pub ppid: usize,
    pub program: UserProgram,
    pub user_stack_bottom: usize,
    pub user_stack_top: usize,
    pub argv0: &'static str,
    pub envp_count: usize,
    pub auxv_placeholder: bool,
    pub page_table_root_ppn: usize,
}

impl ProcessInitInfo {
    pub const fn entry(&self) -> usize {
        self.program.entry()
    }

    pub const fn user_sp_top(&self) -> usize {
        self.user_stack_top
    }

    pub const fn has_argv(&self) -> bool {
        !self.argv0.is_empty()
    }

    pub const fn has_auxv_placeholder(&self) -> bool {
        self.auxv_placeholder
    }
}

pub fn build_init_process_info() -> Result<ProcessInitInfo, ElfError> {
    let image = load_init_image_to_page()?;
    let program = UserProgram {
        name: INIT_ARGV0,
        image,
    };

    Ok(ProcessInitInfo {
        pid: INIT_PID,
        ppid: INIT_PPID,
        program,
        user_stack_bottom: INIT_USER_STACK_BOTTOM,
        user_stack_top: INIT_USER_STACK_TOP,
        argv0: INIT_ARGV0,
        envp_count: 0,
        auxv_placeholder: true,
        page_table_root_ppn: 0,
    })
}

pub fn self_test() {
    crate::println!("[process-init-v53] self-test begin");

    let info = build_init_process_info()
        .expect("[process-init-v53] build init process info failed");

    crate::println!("[process-init-v53] pid   = {}", info.pid);
    crate::println!("[process-init-v53] ppid  = {}", info.ppid);
    crate::println!("[process-init-v53] argv0 = {}", info.argv0);
    crate::println!("[process-init-v53] entry = {:#x}", info.entry());
    crate::println!("[process-init-v53] load va = {:#x}", info.program.load_vaddr());
    crate::println!("[process-init-v53] load pa = {:#x}", info.program.load_pa());
    crate::println!("[process-init-v53] pages = {}", info.program.page_count());
    crate::println!(
        "[process-init-v53] stack = {:#x}..{:#x}",
        info.user_stack_bottom,
        info.user_stack_top
    );

    assert_eq!(info.pid, INIT_PID);
    assert_eq!(info.ppid, INIT_PPID);
    assert!(info.has_argv());
    assert!(info.has_auxv_placeholder());
    assert_eq!(info.user_stack_bottom % PAGE_SIZE, 0);
    assert_eq!(info.user_stack_top % PAGE_SIZE, 0);
    assert_eq!(info.user_stack_top - info.user_stack_bottom, USER_STACK_SIZE);
    assert!(info.entry() >= info.program.load_vaddr());
    assert!(info.program.page_count() >= 1);

    crate::println!("[process-init-v53] self-test passed");
}
