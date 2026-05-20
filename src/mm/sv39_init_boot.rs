pub fn run_external_init_elf_smoke() -> ! {
    crate::println!("[external-init-v82] begin");
    crate::println!("[external-init-v82] ipc/msg/netio path enabled");

    let loaded =
        load_init_image_to_page().expect("[external-init-v82] load external init.elf failed");

    crate::println!("[external-init-v82] elf entry = {:#x}", loaded.entry);
    crate::println!("[external-init-v82] elf vaddr  = {:#x}", loaded.vaddr);
    crate::println!("[external-init-v82] load pa    = {:#x}", loaded.load_pa);
    crate::println!("[external-init-v82] pages      = {}", loaded.page_count);

    unsafe {
        build_page_table(loaded);
        crate::trap::riscv_asm::install_trap_entry();
        crate::trap::user_entry::activate_page_table(root_pa());

        crate::println!("[external-init-v82] after satp");
        crate::println!("[external-init-v82] read satp = {:#x}", read_satp());

        let cx = core::ptr::addr_of_mut!(INIT_CONTEXT);
        crate::trap::user_entry::enter_user(cx, loaded.entry, USER_STACK_TOP);
    }
}

unsafe fn build_page_table(loaded: LoadedInitImage) {
    ROOT_TABLE.0 = [0; 512];
    USER_L1_TABLE.0 = [0; 512];
    USER_L0_TABLE.0 = [0; 512];
    USER_STACK.0.fill(0);
    USER_HEAP.0.fill(0);
    USER_MMAP_AREA.0.fill(0);
    USER_BRK = USER_HEAP_START;
    USER_MMAP_ACTIVE = false;

    ROOT_TABLE.0[0] = leaf_1g_pte(0x0000_0000, KERNEL_LEAF);
    ROOT_TABLE.0[2] = leaf_1g_pte(0x8000_0000, KERNEL_LEAF);

    ROOT_TABLE.0[vpn2(loaded.vaddr)] = table_pte(core::ptr::addr_of!(USER_L1_TABLE) as usize);
    USER_L1_TABLE.0[vpn1(loaded.vaddr)] = table_pte(core::ptr::addr_of!(USER_L0_TABLE) as usize);

    let mut page = 0;
    while page < loaded.page_count {
        let va = loaded.vaddr + page * PAGE_SIZE;
        let pa = loaded.load_pa + page * PAGE_SIZE;
        map_user_4k(va, pa, USER_TEXT_FLAGS);
        page += 1;
    }

    let stack_pa = core::ptr::addr_of!(USER_STACK) as usize;
    let stack_base_va = USER_STACK_TOP - USER_STACK_SIZE;
    let mut i = 0;
    while i < USER_STACK_PAGES {
        let va = stack_base_va + i * PAGE_SIZE;
        let pa = stack_pa + i * PAGE_SIZE;
        map_user_4k(va, pa, USER_STACK_FLAGS);
        i += 1;
    }

    crate::println!(
        "[external-init-v82] user text mapped {:#x} pages {}",
        loaded.vaddr,
        loaded.page_count
    );
    let heap_pa = core::ptr::addr_of!(USER_HEAP) as usize;
    let mut hp = 0;
    while hp < USER_HEAP_PAGES {
        let va = USER_HEAP_START + hp * PAGE_SIZE;
        let pa = heap_pa + hp * PAGE_SIZE;
        map_user_4k(va, pa, USER_STACK_FLAGS);
        hp += 1;
    }

    crate::println!(
        "[brk-v60] user heap mapped {:#x}..{:#x}",
        USER_HEAP_START,
        USER_HEAP_END
    );

    let mmap_pa = core::ptr::addr_of!(USER_MMAP_AREA) as usize;
    let mut mp = 0;
    while mp < USER_MMAP_PAGES {
        let va = USER_MMAP_START + mp * PAGE_SIZE;
        let pa = mmap_pa + mp * PAGE_SIZE;
        map_user_4k(va, pa, USER_STACK_FLAGS);
        mp += 1;
    }

    crate::println!(
        "[mmap-v61] user mmap area mapped {:#x}..{:#x}",
        USER_MMAP_START,
        USER_MMAP_END
    );

    crate::println!(
        "[external-init-v82] user stack mapped {:#x}..{:#x}",
        stack_base_va,
        USER_STACK_TOP
    );
    crate::println!("[external-init-v82] root pa = {:#x}", root_pa());
}

unsafe fn map_user_4k(va: usize, pa: usize, flags: usize) {
    assert_eq!(va % PAGE_SIZE, 0);
    assert_eq!(pa % PAGE_SIZE, 0);
    USER_L0_TABLE.0[vpn0(va)] = leaf_4k_pte(pa, flags);
}

unsafe fn b01_install_low_user_mapping() {
    LOW_L1_TABLE.0.fill(0);
    LOW_L0_TABLE.0.fill(0);
    let mut i = 0usize;
    while i < 512 {
        LOW_L0_TABLE.0[i] = leaf_4k_pte(i * PAGE_SIZE, KERNEL_LEAF);
        i += 1;
    }
    LOW_L1_TABLE.0[0] = table_pte(core::ptr::addr_of!(LOW_L0_TABLE) as usize);
    i = 1;
    while i < 512 {
        LOW_L1_TABLE.0[i] = leaf_2m_pte(i * 512 * PAGE_SIZE, KERNEL_LEAF);
        i += 1;
    }
    ROOT_TABLE.0[0] = table_pte(core::ptr::addr_of!(LOW_L1_TABLE) as usize);
}

unsafe fn b01_restore_low_identity_mapping() {
    ROOT_TABLE.0[0] = leaf_1g_pte(0x0000_0000, KERNEL_LEAF);
}

unsafe fn b01_map_low_user_4k(va: usize, pa: usize, flags: usize) {
    assert_eq!(va % PAGE_SIZE, 0);
    assert_eq!(pa % PAGE_SIZE, 0);
    LOW_L0_TABLE.0[vpn0(va)] = leaf_4k_pte(pa, flags);
}
