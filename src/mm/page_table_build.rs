use crate::config::PAGE_SIZE;
use crate::mm::page_table::{AddressSpace, PTE_R, PTE_U, PTE_W, PTE_X};

const KERNEL_TEXT_VA: usize = 0x8020_0000;
const KERNEL_RODATA_VA: usize = 0x8020_5000;
const KERNEL_DATA_VA: usize = 0x8020_8000;
const KERNEL_STACK_VA: usize = 0x8021_0000;

const USER_TEXT_VA: usize = 0x1000_0000;
const USER_STACK_VA: usize = 0x1000_f000;

const USER_TEXT_PA_DUMMY: usize = 0x8120_0000;
const USER_STACK_PA_DUMMY: usize = 0x8120_1000;

pub fn init() {
    crate::println!("[mm::page_table_build] scaffold init v39");
}

pub fn test() {
    test_kernel_identity_build();
    test_user_mapping_build();
    crate::println!("[page-table-build-v39] real page table build passed");
}

fn test_kernel_identity_build() {
    crate::println!("[page-table-build-v39] kernel identity build begin");

    let mut space = AddressSpace::new();

    space.map(KERNEL_TEXT_VA, KERNEL_TEXT_VA, PTE_R | PTE_X);
    space.map(KERNEL_RODATA_VA, KERNEL_RODATA_VA, PTE_R);
    space.map(KERNEL_DATA_VA, KERNEL_DATA_VA, PTE_R | PTE_W);
    space.map(KERNEL_STACK_VA, KERNEL_STACK_VA, PTE_R | PTE_W);

    check_translation(&space, KERNEL_TEXT_VA, KERNEL_TEXT_VA, PTE_R | PTE_X, PTE_W | PTE_U, "kernel text");
    check_translation(&space, KERNEL_RODATA_VA, KERNEL_RODATA_VA, PTE_R, PTE_W | PTE_X | PTE_U, "kernel rodata");
    check_translation(&space, KERNEL_DATA_VA, KERNEL_DATA_VA, PTE_R | PTE_W, PTE_X | PTE_U, "kernel data");
    check_translation(&space, KERNEL_STACK_VA, KERNEL_STACK_VA, PTE_R | PTE_W, PTE_X | PTE_U, "kernel stack");

    crate::println!("[page-table-build-v39] kernel identity build passed");
}

fn test_user_mapping_build() {
    crate::println!("[page-table-build-v39] user mapping build begin");

    let mut space = AddressSpace::new();

    space.map(USER_TEXT_VA, USER_TEXT_PA_DUMMY, PTE_R | PTE_X | PTE_U);
    space.map(USER_STACK_VA, USER_STACK_PA_DUMMY, PTE_R | PTE_W | PTE_U);

    check_translation(&space, USER_TEXT_VA, USER_TEXT_PA_DUMMY, PTE_R | PTE_X | PTE_U, PTE_W, "user text");
    check_translation(&space, USER_STACK_VA, USER_STACK_PA_DUMMY, PTE_R | PTE_W | PTE_U, PTE_X, "user stack");

    assert!(space.translate(USER_STACK_VA - PAGE_SIZE).is_none());

    crate::println!("[page-table-build-v39] user mapping build passed");
}

fn check_translation(
    space: &AddressSpace,
    va: usize,
    expected_pa: usize,
    required_flags: usize,
    forbidden_flags: usize,
    label: &str,
) {
    let (translated_pa, flags) = space
        .translate(va)
        .expect("[page-table-build-v39] translate failed");

    crate::println!(
        "[page-table-build-v39] {} va={:#x} pa={:#x} flags={:#x}",
        label,
        va,
        translated_pa,
        flags
    );

    assert_eq!(translated_pa, expected_pa);
    assert_eq!(flags & required_flags, required_flags);
    assert_eq!(flags & forbidden_flags, 0);
}
