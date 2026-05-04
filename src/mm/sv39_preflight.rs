use crate::mm::page_table::{PTE_R, PTE_U, PTE_W, PTE_X};

const PAGE_SIZE: usize = 4096;
const PTE_V: usize = 1 << 0;
const PTE_A: usize = 1 << 6;
const PTE_D: usize = 1 << 7;

const USER_CODE_VA: usize = 0x1000_0000;
const USER_STACK_VA: usize = 0x1000_f000;
const DUMMY_CODE_PA: usize = 0x8100_0000;
const DUMMY_STACK_PA: usize = 0x8100_1000;

pub fn init() {
    crate::println!("[mm::sv39_preflight] init v34f");
}

pub fn test() {
    test_page_table_dry_run();
}

pub fn test_page_table_dry_run() {
    crate::println!("[sv39-preflight-v34f] pure dry-run begin");

    let mut sim = SimSv39::new();

    sim.map_4k(USER_CODE_VA, DUMMY_CODE_PA, PTE_R | PTE_X | PTE_U);
    sim.map_4k(USER_STACK_VA, DUMMY_STACK_PA, PTE_R | PTE_W | PTE_U);

    let (translated_code_pa, code_flags) = sim
        .translate(USER_CODE_VA)
        .expect("[sv39-preflight-v34f] translate user code failed");
    let (translated_stack_pa, stack_flags) = sim
        .translate(USER_STACK_VA)
        .expect("[sv39-preflight-v34f] translate user stack failed");

    crate::println!("[sv39-preflight-v34f] code va = {:#x}", USER_CODE_VA);
    crate::println!("[sv39-preflight-v34f] code pa = {:#x}", translated_code_pa);
    crate::println!("[sv39-preflight-v34f] code flags = {:#x}", code_flags);

    crate::println!("[sv39-preflight-v34f] stack va = {:#x}", USER_STACK_VA);
    crate::println!("[sv39-preflight-v34f] stack pa = {:#x}", translated_stack_pa);
    crate::println!("[sv39-preflight-v34f] stack flags = {:#x}", stack_flags);

    assert_eq!(translated_code_pa, DUMMY_CODE_PA);
    assert_eq!(translated_stack_pa, DUMMY_STACK_PA);

    assert!(code_flags & PTE_U != 0);
    assert!(code_flags & PTE_R != 0);
    assert!(code_flags & PTE_X != 0);
    assert!(code_flags & PTE_W == 0);

    assert!(stack_flags & PTE_U != 0);
    assert!(stack_flags & PTE_R != 0);
    assert!(stack_flags & PTE_W != 0);
    assert!(stack_flags & PTE_X == 0);

    crate::println!("[sv39-preflight-v34f] code page flags ok");
    crate::println!("[sv39-preflight-v34f] stack page flags ok");
    crate::println!("[sv39-preflight-v34f] pure dry-run passed");
}

#[derive(Clone, Copy)]
struct SimPte {
    bits: usize,
}

impl SimPte {
    const fn empty() -> Self {
        Self { bits: 0 }
    }

    fn new_leaf(pa: usize, flags: usize) -> Self {
        assert_eq!(pa % PAGE_SIZE, 0);
        let ppn = pa / PAGE_SIZE;
        Self {
            bits: (ppn << 10) | flags | PTE_V | PTE_A | PTE_D,
        }
    }

    fn new_branch(table_id: usize) -> Self {
        Self {
            bits: (table_id << 10) | PTE_V,
        }
    }

    fn is_valid(self) -> bool {
        self.bits & PTE_V != 0
    }

    fn is_leaf(self) -> bool {
        self.bits & (PTE_R | PTE_W | PTE_X) != 0
    }

    fn ppn(self) -> usize {
        self.bits >> 10
    }

    fn flags(self) -> usize {
        self.bits & 0x3ff
    }
}

struct SimSv39 {
    tables: [[SimPte; 512]; 5],
    next_table: usize,
}

impl SimSv39 {
    fn new() -> Self {
        Self {
            tables: [[SimPte::empty(); 512]; 5],
            next_table: 1,
        }
    }

    fn alloc_table(&mut self) -> usize {
        let id = self.next_table;
        self.next_table += 1;
        assert!(id < self.tables.len());
        self.tables[id] = [SimPte::empty(); 512];
        id
    }

    fn map_4k(&mut self, va: usize, pa: usize, flags: usize) {
        assert_eq!(va % PAGE_SIZE, 0);
        assert_eq!(pa % PAGE_SIZE, 0);

        let idx = vpn_indexes(va / PAGE_SIZE);
        let mut table_id = 0usize;

        for level in (1..3).rev() {
            let entry_idx = idx[level];
            let pte = self.tables[table_id][entry_idx];

            if !pte.is_valid() {
                let new_table = self.alloc_table();
                self.tables[table_id][entry_idx] = SimPte::new_branch(new_table);
                table_id = new_table;
            } else {
                assert!(!pte.is_leaf());
                table_id = pte.ppn();
            }
        }

        let leaf_idx = idx[0];
        assert!(!self.tables[table_id][leaf_idx].is_valid());
        self.tables[table_id][leaf_idx] = SimPte::new_leaf(pa, flags);
    }

    fn translate(&self, va: usize) -> Option<(usize, usize)> {
        let vpn = va / PAGE_SIZE;
        let offset = va % PAGE_SIZE;
        let idx = vpn_indexes(vpn);
        let mut table_id = 0usize;

        for level in (0..3).rev() {
            let pte = self.tables[table_id][idx[level]];
            if !pte.is_valid() {
                return None;
            }

            if pte.is_leaf() {
                return Some((pte.ppn() * PAGE_SIZE + offset, pte.flags()));
            }

            table_id = pte.ppn();
        }

        None
    }
}

fn vpn_indexes(vpn: usize) -> [usize; 3] {
    [
        vpn & 0x1ff,
        (vpn >> 9) & 0x1ff,
        (vpn >> 18) & 0x1ff,
    ]
}
