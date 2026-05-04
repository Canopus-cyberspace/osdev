use crate::config::{MEMORY_END, PAGE_SIZE};

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

    crate::println!("[mm::frame] start = {:#x}", start);
    crate::println!("[mm::frame] end   = {:#x}", end);
}

pub fn frame_alloc() -> Option<usize> {
    unsafe { FRAME_ALLOCATOR.alloc() }
}

pub fn frame_dealloc(ppn: usize) {
    unsafe {
        FRAME_ALLOCATOR.dealloc(ppn);
    }
}

pub fn test() {
    crate::println!("[mm::frame] test begin");

    let a = frame_alloc().unwrap();
    let b = frame_alloc().unwrap();
    let c = frame_alloc().unwrap();

    crate::println!("[mm::frame] alloc a = {:#x}", a);
    crate::println!("[mm::frame] alloc b = {:#x}", b);
    crate::println!("[mm::frame] alloc c = {:#x}", c);

    frame_dealloc(b);

    let d = frame_alloc().unwrap();

    crate::println!("[mm::frame] realloc d = {:#x}", d);
    assert_eq!(b, d);

    crate::println!("[mm::frame] test passed");
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
            panic!("[mm::frame] recycled stack overflow");
        }

        self.recycled[self.recycled_len] = ppn;
        self.recycled_len += 1;
    }
}

const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}
