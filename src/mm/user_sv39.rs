//! v44: preparation layer for restoring U-mode under Sv39.
//!
//! This module intentionally does **not** activate satp and does **not** touch
//! the currently stable kernel-only Sv39 trap smoke path. It only centralizes
//! the future user address-space plan so that v45 can map user text/stack in one
//! place instead of scattering constants across task/trap/syscall code.

use crate::config::PAGE_SIZE;
use crate::mm::page_table::{PTE_R, PTE_U, PTE_W, PTE_X};

pub const USER_TEXT_BASE: usize = 0x1000_0000;
pub const USER_STACK_TOP: usize = 0x1001_0000;
pub const USER_STACK_SIZE: usize = PAGE_SIZE;
pub const USER_STACK_BOTTOM: usize = USER_STACK_TOP - USER_STACK_SIZE;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UserMapKind {
    Text,
    Guard,
    Stack,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct UserMapPlan {
    pub kind: UserMapKind,
    pub va: usize,
    pub len: usize,
    pub flags: usize,
    pub mapped: bool,
}

impl UserMapPlan {
    pub const fn new(kind: UserMapKind, va: usize, len: usize, flags: usize, mapped: bool) -> Self {
        Self { kind, va, len, flags, mapped }
    }

    pub const fn end(&self) -> usize {
        self.va + self.len
    }

    pub const fn readable(&self) -> bool {
        self.flags & PTE_R != 0
    }

    pub const fn writable(&self) -> bool {
        self.flags & PTE_W != 0
    }

    pub const fn executable(&self) -> bool {
        self.flags & PTE_X != 0
    }

    pub const fn user_accessible(&self) -> bool {
        self.flags & PTE_U != 0
    }
}

pub fn user_sv39_plan() -> [UserMapPlan; 3] {
    [
        UserMapPlan::new(
            UserMapKind::Text,
            USER_TEXT_BASE,
            PAGE_SIZE,
            PTE_R | PTE_X | PTE_U,
            true,
        ),
        UserMapPlan::new(
            UserMapKind::Guard,
            USER_STACK_BOTTOM - PAGE_SIZE,
            PAGE_SIZE,
            0,
            false,
        ),
        UserMapPlan::new(
            UserMapKind::Stack,
            USER_STACK_BOTTOM,
            USER_STACK_SIZE,
            PTE_R | PTE_W | PTE_U,
            true,
        ),
    ]
}

pub fn check_user_sv39_plan() {
    crate::println!("[user-sv39-v44] plan check begin");

    let plan = user_sv39_plan();
    let text = plan[0];
    let guard = plan[1];
    let stack = plan[2];

    assert_eq!(text.kind, UserMapKind::Text);
    assert!(text.mapped);
    assert!(text.readable());
    assert!(!text.writable());
    assert!(text.executable());
    assert!(text.user_accessible());
    assert_eq!(text.va % PAGE_SIZE, 0);
    assert_eq!(text.len, PAGE_SIZE);

    assert_eq!(guard.kind, UserMapKind::Guard);
    assert!(!guard.mapped);
    assert_eq!(guard.flags, 0);
    assert_eq!(guard.end(), stack.va);

    assert_eq!(stack.kind, UserMapKind::Stack);
    assert!(stack.mapped);
    assert!(stack.readable());
    assert!(stack.writable());
    assert!(!stack.executable());
    assert!(stack.user_accessible());
    assert_eq!(stack.va % PAGE_SIZE, 0);
    assert_eq!(stack.end(), USER_STACK_TOP);

    crate::println!("[user-sv39-v44] text  {:#x}..{:#x}", text.va, text.end());
    crate::println!("[user-sv39-v44] guard {:#x}..{:#x}", guard.va, guard.end());
    crate::println!("[user-sv39-v44] stack {:#x}..{:#x}", stack.va, stack.end());
    crate::println!("[user-sv39-v44] plan check passed");
}
