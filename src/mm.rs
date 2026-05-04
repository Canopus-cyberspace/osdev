const PAGE_SIZE: usize = 4096;
const MEMORY_END: usize = 0x8800_0000;

static mut FRAME_ALLOCATOR: FrameAllocator = FrameAllocator {
    current: 0,
    end: 0,
    recycled: [0; 1024],
    recycled_len: 0,
};

pub fn init() {
    extern "C" {
        fn ekernel();
    }

    let start = align_up(ekernel as *const () as usize, PAGE_SIZE);
    let end = MEMORY_END;

    unsafe {
        FRAME_ALLOCATOR.init(start, end);
    }

    crate::println!("[mm] frame allocator initialized");
    crate::println!("[mm] start = {:#x}", start);
    crate::println!("[mm] end   = {:#x}", end);
}

pub fn test() {
    crate::println!("[mm] frame allocator test begin");

    let a = frame_alloc().unwrap();
    let b = frame_alloc().unwrap();
    let c = frame_alloc().unwrap();

    crate::println!("[mm] alloc a = {:#x}", a);
    crate::println!("[mm] alloc b = {:#x}", b);
    crate::println!("[mm] alloc c = {:#x}", c);

    frame_dealloc(b);
    crate::println!("[mm] dealloc b = {:#x}", b);

    let d = frame_alloc().unwrap();
    crate::println!("[mm] alloc d = {:#x}", d);

    assert_eq!(b, d);

    crate::println!("[mm] frame allocator test passed");
}

pub fn frame_alloc() -> Option<usize> {
    unsafe {
        FRAME_ALLOCATOR.alloc()
    }
}

pub fn frame_dealloc(ppn: usize) {
    unsafe {
        FRAME_ALLOCATOR.dealloc(ppn);
    }
}

struct FrameAllocator {
    current: usize,
    end: usize,
    recycled: [usize; 1024],
    recycled_len: usize,
}

impl FrameAllocator {
    fn init(&mut self, start: usize, end: usize) {
        self.current = start / PAGE_SIZE;
        self.end = end / PAGE_SIZE;
        self.recycled_len = 0;
    }

    fn alloc(&mut self) -> Option<usize> {
        if self.recycled_len > 0 {
            self.recycled_len -= 1;
            return Some(self.recycled[self.recycled_len]);
        }

        if self.current < self.end {
            let ppn = self.current;
            self.current += 1;
            Some(ppn)
        } else {
            None
        }
    }

    fn dealloc(&mut self, ppn: usize) {
        if self.recycled_len >= self.recycled.len() {
            panic!("[mm] recycled frame stack overflow");
        }

        self.recycled[self.recycled_len] = ppn;
        self.recycled_len += 1;
    }
}

const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}
