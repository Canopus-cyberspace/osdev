#[derive(Clone, Copy, Debug)]
pub struct MapPermission {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub user: bool,
}

impl MapPermission {
    pub const fn new(readable: bool, writable: bool, executable: bool, user: bool) -> Self {
        Self {
            readable,
            writable,
            executable,
            user,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UserRegion {
    pub name: &'static str,
    pub start: usize,
    pub end: usize,
    pub perm: MapPermission,
}

pub struct UserAddressSpace {
    regions: [Option<UserRegion>; 8],
    len: usize,
}

impl UserAddressSpace {
    pub const fn new() -> Self {
        Self {
            regions: [None; 8],
            len: 0,
        }
    }

    pub fn push_region(&mut self, region: UserRegion) {
        assert!(self.len < self.regions.len());
        assert!(region.start < region.end);
        self.regions[self.len] = Some(region);
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn region(&self, index: usize) -> Option<UserRegion> {
        if index >= self.len {
            return None;
        }
        self.regions[index]
    }
}

pub fn init() {
    crate::println!("[mm::user_space] scaffold init v33");
}

pub fn test() {
    crate::println!("[mm::user_space] metadata test begin v33");

    let mut space = UserAddressSpace::new();

    space.push_region(UserRegion {
        name: "user_text",
        start: 0x1000_0000,
        end: 0x1000_1000,
        perm: MapPermission::new(true, false, true, true),
    });

    space.push_region(UserRegion {
        name: "user_stack",
        start: 0x1000_f000,
        end: 0x1001_0000,
        perm: MapPermission::new(true, true, false, true),
    });

    assert_eq!(space.len(), 2);

    let text = space.region(0).expect("missing user_text region");
    let stack = space.region(1).expect("missing user_stack region");

    assert_eq!(text.name, "user_text");
    assert!(text.perm.readable);
    assert!(!text.perm.writable);
    assert!(text.perm.executable);
    assert!(text.perm.user);

    assert_eq!(stack.name, "user_stack");
    assert!(stack.perm.readable);
    assert!(stack.perm.writable);
    assert!(!stack.perm.executable);
    assert!(stack.perm.user);

    crate::println!("[mm::user_space] metadata test passed v33");
}
