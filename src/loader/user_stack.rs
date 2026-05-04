#![allow(dead_code)]
#![allow(static_mut_refs)]

use crate::config::{PAGE_SIZE, USER_STACK_SIZE};
use crate::loader::elf::ElfError;
use crate::loader::process_image::{build_init_process_info, ProcessInitInfo};

pub const AT_NULL: usize = 0;
pub const AT_PHDR: usize = 3;
pub const AT_PHENT: usize = 4;
pub const AT_PHNUM: usize = 5;
pub const AT_PAGESZ: usize = 6;
pub const AT_ENTRY: usize = 9;

#[repr(align(16))]
struct DryRunStack([u8; USER_STACK_SIZE]);

static mut DRY_RUN_STACK: DryRunStack = DryRunStack([0; USER_STACK_SIZE]);

#[derive(Copy, Clone, Debug)]
pub struct InitialUserStackLayout {
    pub stack_bottom: usize,
    pub stack_top: usize,
    pub initial_sp: usize,
    pub argc: usize,
    pub argv0_ptr: usize,
    pub argv_null_slot: usize,
    pub envp_null_slot: usize,
    pub auxv_start: usize,
    pub auxv_count: usize,
}

impl InitialUserStackLayout {
    pub const fn is_aligned(&self) -> bool {
        self.initial_sp % 16 == 0
    }

    pub const fn contains_argv0(&self) -> bool {
        self.argv0_ptr >= self.stack_bottom && self.argv0_ptr < self.stack_top
    }
}

pub fn build_initial_user_stack_dry_run() -> Result<InitialUserStackLayout, ElfError> {
    let info = build_init_process_info()?;
    Ok(build_stack_for_process(&info))
}

fn build_stack_for_process(info: &ProcessInitInfo) -> InitialUserStackLayout {
    let stack_bottom = info.user_stack_bottom;
    let stack_top = info.user_stack_top;
    let mut offset = USER_STACK_SIZE;

    unsafe {
        let base = core::ptr::addr_of_mut!(DRY_RUN_STACK) as *mut u8;
        let buf = core::slice::from_raw_parts_mut(base, USER_STACK_SIZE);
        buf.fill(0);

        let argv0 = info.argv0.as_bytes();
        offset = push_bytes(buf, offset, b"\0");
        offset = push_bytes(buf, offset, argv0);
        let argv0_ptr = stack_bottom + offset;

        offset = align_down(offset, 16);

        offset = push_usize(buf, offset, 0);
        offset = push_usize(buf, offset, AT_NULL);

        offset = push_usize(buf, offset, info.entry());
        offset = push_usize(buf, offset, AT_ENTRY);

        offset = push_usize(buf, offset, PAGE_SIZE);
        offset = push_usize(buf, offset, AT_PAGESZ);

        offset = push_usize(buf, offset, 56);
        offset = push_usize(buf, offset, AT_PHENT);

        offset = push_usize(buf, offset, 1);
        offset = push_usize(buf, offset, AT_PHNUM);

        offset = push_usize(buf, offset, info.program.load_vaddr());
        offset = push_usize(buf, offset, AT_PHDR);

        let auxv_start = stack_bottom + offset;

        offset = push_usize(buf, offset, 0);
        let envp_null_slot = stack_bottom + offset;

        offset = push_usize(buf, offset, 0);
        let argv_null_slot = stack_bottom + offset;

        offset = push_usize(buf, offset, argv0_ptr);

        offset = push_usize(buf, offset, 1);
        let initial_sp = stack_bottom + offset;

        InitialUserStackLayout {
            stack_bottom,
            stack_top,
            initial_sp,
            argc: 1,
            argv0_ptr,
            argv_null_slot,
            envp_null_slot,
            auxv_start,
            auxv_count: 6,
        }
    }
}

unsafe fn push_bytes(buf: &mut [u8], offset: usize, bytes: &[u8]) -> usize {
    assert!(offset >= bytes.len());
    let new_offset = offset - bytes.len();
    let mut i = 0;

    while i < bytes.len() {
        buf[new_offset + i] = bytes[i];
        i += 1;
    }

    new_offset
}

unsafe fn push_usize(buf: &mut [u8], offset: usize, value: usize) -> usize {
    assert!(offset >= core::mem::size_of::<usize>());
    let new_offset = offset - core::mem::size_of::<usize>();
    let bytes = value.to_le_bytes();
    let mut i = 0;

    while i < bytes.len() {
        buf[new_offset + i] = bytes[i];
        i += 1;
    }

    new_offset
}

const fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

pub fn self_test() {
    crate::println!("[user-stack-v53] dry-run begin");

    let layout = build_initial_user_stack_dry_run()
        .expect("[user-stack-v53] build initial user stack dry-run failed");

    crate::println!("[user-stack-v53] stack = {:#x}..{:#x}", layout.stack_bottom, layout.stack_top);
    crate::println!("[user-stack-v53] initial sp = {:#x}", layout.initial_sp);
    crate::println!("[user-stack-v53] argc = {}", layout.argc);
    crate::println!("[user-stack-v53] argv0 ptr = {:#x}", layout.argv0_ptr);
    crate::println!("[user-stack-v53] argv null slot = {:#x}", layout.argv_null_slot);
    crate::println!("[user-stack-v53] envp null slot = {:#x}", layout.envp_null_slot);
    crate::println!("[user-stack-v53] auxv start = {:#x}", layout.auxv_start);
    crate::println!("[user-stack-v53] auxv count = {}", layout.auxv_count);

    assert_eq!(layout.argc, 1);
    assert!(layout.is_aligned());
    assert!(layout.contains_argv0());
    assert!(layout.stack_bottom < layout.initial_sp);
    assert!(layout.initial_sp < layout.stack_top);
    assert!(layout.argv_null_slot < layout.envp_null_slot);
    assert!(layout.envp_null_slot < layout.auxv_start);
    assert_eq!(layout.auxv_count, 6);

    crate::println!("[user-stack-v53] dry-run passed");
}
