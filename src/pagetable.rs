const PAGE_SIZE: usize = 4096;
const PAGE_SIZE_2M: usize = 2 * 1024 * 1024;
const PTE_COUNT: usize = 512;

pub const PTE_V: usize = 1 << 0;
pub const PTE_R: usize = 1 << 1;
pub const PTE_W: usize = 1 << 2;
pub const PTE_X: usize = 1 << 3;
pub const PTE_U: usize = 1 << 4;
pub const PTE_A: usize = 1 << 6;
pub const PTE_D: usize = 1 << 7;

const USER_BASE: usize = 0x1000_0000;
const USER_STACK_TOP: usize = 0x1001_0000;
const USER_STACK_SIZE: usize = PAGE_SIZE;

#[derive(Copy, Clone)]
#[repr(transparent)]
struct PageTableEntry {
    bits: usize,
}

impl PageTableEntry {
    fn new(ppn: usize, flags: usize) -> Self {
        let mut real_flags = flags;

        if flags & (PTE_R | PTE_W | PTE_X) != 0 {
            real_flags |= PTE_A | PTE_D;
        }

        Self {
            bits: (ppn << 10) | real_flags,
        }
    }

    fn is_valid(&self) -> bool {
        self.bits & PTE_V != 0
    }

    fn ppn(&self) -> usize {
        self.bits >> 10
    }

    fn flags(&self) -> usize {
        self.bits & 0x3ff
    }
}

#[repr(C, align(4096))]
struct PageTable {
    entries: [PageTableEntry; PTE_COUNT],
}

pub struct AddressSpace {
    root_ppn: usize,
}

impl AddressSpace {
    pub fn new() -> Self {
        let root_ppn = crate::mm::frame_alloc().expect("[pagetable] alloc root page table failed");
        zero_frame(root_ppn);
        Self { root_ppn }
    }

    pub fn root_ppn(&self) -> usize {
        self.root_ppn
    }

    pub fn map(&mut self, va: usize, pa: usize, flags: usize) {
        assert_eq!(va % PAGE_SIZE, 0);
        assert_eq!(pa % PAGE_SIZE, 0);

        let vpn = va / PAGE_SIZE;
        let ppn = pa / PAGE_SIZE;

        let pte = self.find_pte_create(vpn);
        assert!(!pte.is_valid());

        *pte = PageTableEntry::new(ppn, flags | PTE_V);
    }

    pub fn map_1g(&mut self, va: usize, pa: usize, flags: usize) {
        const PAGE_SIZE_1G: usize = 1024 * 1024 * 1024;

        assert_eq!(va % PAGE_SIZE_1G, 0);
        assert_eq!(pa % PAGE_SIZE_1G, 0);

        let vpn = va / PAGE_SIZE;
        let ppn = pa / PAGE_SIZE;
        let indexes = vpn_indexes(vpn);

        let root = ppn_to_table(self.root_ppn);
        let pte = &mut root.entries[indexes[2]];

        assert!(!pte.is_valid());

        *pte = PageTableEntry::new(ppn, flags | PTE_V);
    }

    pub fn map_2m(&mut self, va: usize, pa: usize, flags: usize) {
        assert_eq!(va % PAGE_SIZE_2M, 0);
        assert_eq!(pa % PAGE_SIZE_2M, 0);

        let vpn = va / PAGE_SIZE;
        let ppn = pa / PAGE_SIZE;
        let indexes = vpn_indexes(vpn);

        let root = ppn_to_table(self.root_ppn);
        let root_pte = &mut root.entries[indexes[2]];

        if !root_pte.is_valid() {
            let new_ppn = crate::mm::frame_alloc().expect("[pagetable] alloc level1 page table failed");
            zero_frame(new_ppn);
            *root_pte = PageTableEntry::new(new_ppn, PTE_V);
        }

        let level1 = ppn_to_table(root_pte.ppn());
        let pte = &mut level1.entries[indexes[1]];
        assert!(!pte.is_valid());

        *pte = PageTableEntry::new(ppn, flags | PTE_V);
    }

    pub fn translate(&self, va: usize) -> Option<(usize, usize)> {
        let vpn = va / PAGE_SIZE;
        let offset = va % PAGE_SIZE;
        let indexes = vpn_indexes(vpn);

        let mut ppn = self.root_ppn;

        for level in (0..3).rev() {
            let table = ppn_to_table(ppn);
            let pte = table.entries[indexes[level]];

            if !pte.is_valid() {
                return None;
            }

            let flags = pte.flags();
            let is_leaf = flags & (PTE_R | PTE_W | PTE_X) != 0;

            if is_leaf {
                let pa = match level {
                    2 => {
                        let vpn_low = vpn & 0x3ffff;
                        pte.ppn() * PAGE_SIZE + vpn_low * PAGE_SIZE + offset
                    }
                    1 => {
                        let vpn_low = vpn & 0x1ff;
                        pte.ppn() * PAGE_SIZE + vpn_low * PAGE_SIZE + offset
                    }
                    0 => pte.ppn() * PAGE_SIZE + offset,
                    _ => unreachable!(),
                };

                return Some((pa, flags));
            }

            ppn = pte.ppn();
        }

        None
    }

    fn find_pte_create(&mut self, vpn: usize) -> &'static mut PageTableEntry {
        let indexes = vpn_indexes(vpn);
        let mut ppn = self.root_ppn;

        for level in (1..3).rev() {
            let table = ppn_to_table(ppn);
            let pte = &mut table.entries[indexes[level]];

            if !pte.is_valid() {
                let new_ppn = crate::mm::frame_alloc()
                    .expect("[pagetable] alloc lower page table failed");
                zero_frame(new_ppn);
                *pte = PageTableEntry::new(new_ppn, PTE_V);
            }

            ppn = pte.ppn();
        }

        let table = ppn_to_table(ppn);
        &mut table.entries[indexes[0]]
    }
}

fn zero_frame(ppn: usize) {
    let start = ppn * PAGE_SIZE;
    unsafe {
        core::slice::from_raw_parts_mut(start as *mut u8, PAGE_SIZE).fill(0);
    }
}

fn ppn_to_table(ppn: usize) -> &'static mut PageTable {
    let pa = ppn * PAGE_SIZE;
    unsafe {
        &mut *(pa as *mut PageTable)
    }
}

fn vpn_indexes(vpn: usize) -> [usize; 3] {
    [
        vpn & 0x1ff,
        (vpn >> 9) & 0x1ff,
        (vpn >> 18) & 0x1ff,
    ]
}

pub fn test() {
    crate::println!("[pagetable] test begin");

    let mut space = AddressSpace::new();

    let va = 0x1000_0000usize;
    let ppn = crate::mm::frame_alloc().expect("[pagetable] alloc data frame failed");
    let pa = ppn * PAGE_SIZE;

    space.map(va, pa, PTE_R | PTE_W | PTE_X | PTE_U);

    let result = space.translate(va).expect("[pagetable] translate failed");

    crate::println!("[pagetable] root ppn = {:#x}", space.root_ppn());
    crate::println!("[pagetable] va       = {:#x}", va);
    crate::println!("[pagetable] pa       = {:#x}", result.0);
    crate::println!("[pagetable] flags    = {:#x}", result.1);

    assert_eq!(result.0, pa);
    assert!(result.1 & PTE_R != 0);
    assert!(result.1 & PTE_W != 0);
    assert!(result.1 & PTE_X != 0);
    assert!(result.1 & PTE_U != 0);

    crate::println!("[pagetable] test passed");
}

const KERNEL_MAP_START: usize = 0x8000_0000;
const KERNEL_MAP_END: usize = 0x8040_0000;

static mut KERNEL_SPACE: Option<AddressSpace> = None;

pub fn init_kernel_space(user_image: &[u8]) -> (usize, usize) {
    crate::println!("[pagetable] init kernel address space");
    crate::println!("[debug] 1 before AddressSpace::new");

    let mut space = AddressSpace::new();

    crate::println!("[debug] 2 after AddressSpace::new root={:#x}", space.root_ppn());

    crate::println!("[debug] map kernel 1G identity");
    space.map_1g(0x8000_0000, 0x8000_0000, PTE_R | PTE_W | PTE_X);
    crate::println!("[debug] 3 kernel mapped by 1G page");

    crate::println!("[debug] 4 before map_user_image");
    map_user_image(&mut space, user_image);
    crate::println!("[debug] 5 after map_user_image");

    crate::println!("[debug] 6 before map_user_stack");
    map_user_stack(&mut space);
    crate::println!("[debug] 7 after map_user_stack");

    let root_ppn = space.root_ppn();

    crate::println!("[debug] 8 before save KERNEL_SPACE");

    unsafe {
        KERNEL_SPACE = Some(space);
    }

    crate::println!("[debug] 9 after save KERNEL_SPACE");
    crate::println!("[pagetable] root ppn for satp = {:#x}", root_ppn);

    crate::println!("[debug] 10 before activate_satp");
    activate_satp(root_ppn);
    crate::println!("[debug] 11 after activate_satp");

    let satp = read_satp();
    crate::println!("[pagetable] satp = {:#x}", satp);
    crate::println!("[pagetable] Sv39 enabled");

    (USER_BASE, USER_STACK_TOP)
}

fn map_user_image(space: &mut AddressSpace, image: &[u8]) {
    crate::println!("[pagetable] user image size = {}", image.len());

    let page_count = (image.len() + PAGE_SIZE - 1) / PAGE_SIZE;

    for i in 0..page_count {
        let ppn = crate::mm::frame_alloc().expect("[pagetable] alloc user image frame failed");
        zero_frame(ppn);

        let dst = (ppn * PAGE_SIZE) as *mut u8;
        let start = i * PAGE_SIZE;
        let end = core::cmp::min(start + PAGE_SIZE, image.len());
        let len = end - start;

        unsafe {
            core::ptr::copy_nonoverlapping(image[start..end].as_ptr(), dst, len);
        }

        space.map(USER_BASE + i * PAGE_SIZE, ppn * PAGE_SIZE, PTE_R | PTE_X | PTE_U);
    }

    crate::println!("[pagetable] user image mapped at {:#x}", USER_BASE);
}

fn map_user_stack(space: &mut AddressSpace) {
    let stack_bottom = USER_STACK_TOP - USER_STACK_SIZE;

    for i in 0..(USER_STACK_SIZE / PAGE_SIZE) {
        let ppn = crate::mm::frame_alloc().expect("[pagetable] alloc user stack frame failed");
        zero_frame(ppn);

        space.map(
            stack_bottom + i * PAGE_SIZE,
            ppn * PAGE_SIZE,
            PTE_R | PTE_W | PTE_U,
        );
    }

    crate::println!(
        "[pagetable] user stack mapped {:#x}..{:#x}",
        stack_bottom,
        USER_STACK_TOP
    );
}

fn activate_satp(root_ppn: usize) {
    const SATP_MODE_SV39: usize = 8usize << 60;
    let satp = SATP_MODE_SV39 | root_ppn;

    unsafe {
        core::arch::asm!(
            "csrw satp, {0}",
            "sfence.vma",
            in(reg) satp,
        );
    }
}

fn read_satp() -> usize {
    let value: usize;
    unsafe {
        core::arch::asm!("csrr {}, satp", out(reg) value);
    }
    value
}
