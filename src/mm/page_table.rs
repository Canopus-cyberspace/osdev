use crate::config::PAGE_SIZE;

const PTE_COUNT: usize = 512;

pub const PTE_V: usize = 1 << 0;
pub const PTE_R: usize = 1 << 1;
pub const PTE_W: usize = 1 << 2;
pub const PTE_X: usize = 1 << 3;
pub const PTE_U: usize = 1 << 4;
pub const PTE_A: usize = 1 << 6;
pub const PTE_D: usize = 1 << 7;

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
        let root_ppn = crate::mm::frame_alloc().expect("[page_table] alloc root failed");
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
                return Some((pte.ppn() * PAGE_SIZE + offset, flags));
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
                let new_ppn = crate::mm::frame_alloc().expect("[page_table] alloc lower failed");
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
    unsafe { &mut *(pa as *mut PageTable) }
}

fn vpn_indexes(vpn: usize) -> [usize; 3] {
    [
        vpn & 0x1ff,
        (vpn >> 9) & 0x1ff,
        (vpn >> 18) & 0x1ff,
    ]
}

pub fn test() {
    crate::println!("[mm::page_table] test begin");

    let mut space = AddressSpace::new();

    let va = 0x1000_0000usize;
    let ppn = crate::mm::frame_alloc().expect("[page_table] alloc data frame failed");
    let pa = ppn * PAGE_SIZE;

    space.map(va, pa, PTE_R | PTE_W | PTE_X | PTE_U);

    let result = space.translate(va).expect("[page_table] translate failed");

    crate::println!("[mm::page_table] root ppn = {:#x}", space.root_ppn());
    crate::println!("[mm::page_table] va       = {:#x}", va);
    crate::println!("[mm::page_table] pa       = {:#x}", result.0);
    crate::println!("[mm::page_table] flags    = {:#x}", result.1);

    assert_eq!(result.0, pa);
    assert!(result.1 & PTE_U != 0);

    crate::println!("[mm::page_table] test passed");
}
