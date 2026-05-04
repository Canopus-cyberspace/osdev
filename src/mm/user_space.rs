use crate::config::PAGE_SIZE;

pub const USER_TEXT_BASE: usize = 0x1000_0000;
pub const USER_STACK_TOP: usize = 0x2000_0000;
pub const USER_STACK_SIZE: usize = PAGE_SIZE * 4;
pub const MAX_USER_REGIONS: usize = 16;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MapPermission {
    bits: usize,
}

impl MapPermission {
    pub const READ: Self = Self { bits: 1 << 0 };
    pub const WRITE: Self = Self { bits: 1 << 1 };
    pub const EXECUTE: Self = Self { bits: 1 << 2 };
    pub const USER: Self = Self { bits: 1 << 3 };

    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    pub const fn bits(self) -> usize {
        self.bits
    }

    pub const fn union(self, rhs: Self) -> Self {
        Self {
            bits: self.bits | rhs.bits,
        }
    }

    pub const fn contains(self, rhs: Self) -> bool {
        (self.bits & rhs.bits) == rhs.bits
    }
}

#[derive(Copy, Clone)]
pub struct UserRegion {
    pub start: usize,
    pub end: usize,
    pub perm: MapPermission,
}

impl UserRegion {
    pub const fn new(start: usize, end: usize, perm: MapPermission) -> Self {
        Self { start, end, perm }
    }

    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    pub const fn contains_addr(&self, addr: usize) -> bool {
        self.start <= addr && addr < self.end
    }
}

pub struct UserAddressSpace {
    entry: usize,
    stack_top: usize,
    regions: [Option<UserRegion>; MAX_USER_REGIONS],
    region_count: usize,
}

impl UserAddressSpace {
    pub const fn empty() -> Self {
        Self {
            entry: 0,
            stack_top: 0,
            regions: [None; MAX_USER_REGIONS],
            region_count: 0,
        }
    }

    pub fn new(entry: usize, stack_top: usize) -> Self {
        let mut space = Self::empty();
        space.entry = entry;
        space.stack_top = stack_top;
        space
    }

    pub const fn entry(&self) -> usize {
        self.entry
    }

    pub const fn stack_top(&self) -> usize {
        self.stack_top
    }

    pub const fn region_count(&self) -> usize {
        self.region_count
    }

    pub fn add_region(&mut self, region: UserRegion) -> Result<(), &'static str> {
        if region.start >= region.end {
            return Err("invalid user region range");
        }
        if region.start % PAGE_SIZE != 0 || region.end % PAGE_SIZE != 0 {
            return Err("user region is not page aligned");
        }
        if self.region_count >= MAX_USER_REGIONS {
            return Err("too many user regions");
        }

        self.regions[self.region_count] = Some(region);
        self.region_count += 1;
        Ok(())
    }

    pub fn find_region(&self, addr: usize) -> Option<UserRegion> {
        let mut i = 0;
        while i < self.region_count {
            if let Some(region) = self.regions[i] {
                if region.contains_addr(addr) {
                    return Some(region);
                }
            }
            i += 1;
        }
        None
    }
}

pub fn init() {
    crate::println!("[mm::user_space] scaffold init v33");
}

pub fn build_embedded_umode_metadata(entry: usize) -> UserAddressSpace {
    let mut space = UserAddressSpace::new(entry, USER_STACK_TOP);

    let text_perm = MapPermission::READ
        .union(MapPermission::EXECUTE)
        .union(MapPermission::USER);
    let stack_perm = MapPermission::READ
        .union(MapPermission::WRITE)
        .union(MapPermission::USER);

    space
        .add_region(UserRegion::new(
            USER_TEXT_BASE,
            USER_TEXT_BASE + PAGE_SIZE,
            text_perm,
        ))
        .expect("add embedded user text region failed");

    space
        .add_region(UserRegion::new(
            USER_STACK_TOP - USER_STACK_SIZE,
            USER_STACK_TOP,
            stack_perm,
        ))
        .expect("add embedded user stack region failed");

    space
}

pub fn test_user_address_space_metadata() {
    crate::println!("[mm::user_space] metadata test begin v33");

    let entry = USER_TEXT_BASE;
    let space = build_embedded_umode_metadata(entry);

    assert_eq!(space.entry(), USER_TEXT_BASE);
    assert_eq!(space.stack_top(), USER_STACK_TOP);
    assert_eq!(space.region_count(), 2);

    let text = space.find_region(USER_TEXT_BASE).expect("text region missing");
    assert!(text.perm.contains(MapPermission::READ));
    assert!(text.perm.contains(MapPermission::EXECUTE));
    assert!(text.perm.contains(MapPermission::USER));
    assert!(!text.perm.contains(MapPermission::WRITE));
    assert_eq!(text.len(), PAGE_SIZE);

    let stack_addr = USER_STACK_TOP - 16;
    let stack = space.find_region(stack_addr).expect("stack region missing");
    assert!(stack.perm.contains(MapPermission::READ));
    assert!(stack.perm.contains(MapPermission::WRITE));
    assert!(stack.perm.contains(MapPermission::USER));
    assert!(!stack.perm.contains(MapPermission::EXECUTE));
    assert_eq!(stack.len(), USER_STACK_SIZE);

    assert!(space.find_region(0).is_none());

    crate::println!("[mm::user_space] metadata test passed v33");
}
