const USER_STACK_TOP: usize = 0x4002_0000;
const USER_STACK_PAGES: usize = 4;
const USER_STACK_SIZE: usize = USER_STACK_PAGES * PAGE_SIZE;
const USER_HEAP_START: usize = 0x4003_0000;
const USER_HEAP_PAGES: usize = 4;
const USER_HEAP_SIZE: usize = USER_HEAP_PAGES * PAGE_SIZE;
const USER_HEAP_END: usize = USER_HEAP_START + USER_HEAP_SIZE;
const USER_MMAP_START: usize = 0x4004_0000;
const USER_MMAP_PAGES: usize = 4;
const USER_MMAP_SIZE: usize = USER_MMAP_PAGES * PAGE_SIZE;
const USER_MMAP_END: usize = USER_MMAP_START + USER_MMAP_SIZE;

const PTE_V: usize = 1 << 0;
const PTE_R: usize = 1 << 1;
const PTE_W: usize = 1 << 2;
const PTE_X: usize = 1 << 3;
const PTE_U: usize = 1 << 4;
const PTE_A: usize = 1 << 6;
const PTE_D: usize = 1 << 7;

const SATP_MODE_SV39: usize = 8usize << 60;
const SSTATUS_SPP: usize = 1 << 8;
const SSTATUS_SPIE: usize = 1 << 5;
const SSTATUS_SUM: usize = 1 << 18;

const KERNEL_LEAF: usize = PTE_V | PTE_R | PTE_W | PTE_X | PTE_A | PTE_D;
const USER_TEXT_FLAGS: usize = PTE_V | PTE_R | PTE_X | PTE_U | PTE_A | PTE_D;
const USER_STACK_FLAGS: usize = PTE_V | PTE_R | PTE_W | PTE_U | PTE_A | PTE_D;

#[repr(C, align(4096))]
struct PageTable512([usize; 512]);

#[repr(C, align(4096))]
struct UserStack([u8; USER_STACK_SIZE]);

#[repr(C, align(4096))]
struct UserHeap([u8; USER_HEAP_SIZE]);

#[repr(C, align(4096))]
struct UserMmapArea([u8; USER_MMAP_SIZE]);

const REAL_UMODE_IMAGE_SIZE: usize = PAGE_SIZE * 512;

#[repr(C, align(4096))]
struct RealUmodeImage([u8; REAL_UMODE_IMAGE_SIZE]);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TrapContext {
    pub regs: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
}

static mut ROOT_TABLE: PageTable512 = PageTable512([0; 512]);
static mut USER_L1_TABLE: PageTable512 = PageTable512([0; 512]);
static mut USER_L0_TABLE: PageTable512 = PageTable512([0; 512]);
static mut LOW_L1_TABLE: PageTable512 = PageTable512([0; 512]);
static mut LOW_L0_TABLE: PageTable512 = PageTable512([0; 512]);
static mut USER_STACK: UserStack = UserStack([0; USER_STACK_SIZE]);
static mut USER_HEAP: UserHeap = UserHeap([0; USER_HEAP_SIZE]);
static mut USER_MMAP_AREA: UserMmapArea = UserMmapArea([0; USER_MMAP_SIZE]);
static mut REAL_UMODE_IMAGE: RealUmodeImage = RealUmodeImage([0; REAL_UMODE_IMAGE_SIZE]);
static mut USER_BRK: usize = USER_HEAP_START;
static mut USER_MMAP_ACTIVE: bool = false;
static mut INIT_CONTEXT: TrapContext = TrapContext {
    regs: [0; 32],
    sstatus: 0,
    sepc: 0,
};
static EXIT_SEEN: AtomicBool = AtomicBool::new(false);
