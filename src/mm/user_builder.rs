use crate::config::PAGE_SIZE;

const USER_TEXT_START: usize = 0x1000_0000;
const USER_TEXT_SIZE: usize = PAGE_SIZE;
const USER_STACK_TOP: usize = 0x1001_0000;
const USER_STACK_SIZE: usize = PAGE_SIZE;
const USER_STACK_BOTTOM: usize = USER_STACK_TOP - USER_STACK_SIZE;
const USER_GUARD_PAGE_START: usize = USER_STACK_BOTTOM - PAGE_SIZE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapPermission {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub user: bool,
}

impl MapPermission {
    pub const fn user_text() -> Self {
        Self {
            read: true,
            write: false,
            execute: true,
            user: true,
        }
    }

    pub const fn user_stack() -> Self {
        Self {
            read: true,
            write: true,
            execute: false,
            user: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserRegionKind {
    Text,
    Stack,
    Guard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserRegion {
    pub kind: UserRegionKind,
    pub start: usize,
    pub end: usize,
    pub perm: MapPermission,
    pub mapped: bool,
}

impl UserRegion {
    pub const fn new(
        kind: UserRegionKind,
        start: usize,
        end: usize,
        perm: MapPermission,
        mapped: bool,
    ) -> Self {
        Self {
            kind,
            start,
            end,
            perm,
            mapped,
        }
    }

    pub fn size(&self) -> usize {
        self.end - self.start
    }

    pub fn is_page_aligned(&self) -> bool {
        self.start % PAGE_SIZE == 0 && self.end % PAGE_SIZE == 0
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && other.start < self.end
    }
}

pub struct UserAddressSpaceBuilder {
    regions: [UserRegion; 3],
}

impl UserAddressSpaceBuilder {
    pub const fn new_smoke() -> Self {
        Self {
            regions: [
                UserRegion::new(
                    UserRegionKind::Text,
                    USER_TEXT_START,
                    USER_TEXT_START + USER_TEXT_SIZE,
                    MapPermission::user_text(),
                    true,
                ),
                UserRegion::new(
                    UserRegionKind::Guard,
                    USER_GUARD_PAGE_START,
                    USER_STACK_BOTTOM,
                    MapPermission {
                        read: false,
                        write: false,
                        execute: false,
                        user: true,
                    },
                    false,
                ),
                UserRegion::new(
                    UserRegionKind::Stack,
                    USER_STACK_BOTTOM,
                    USER_STACK_TOP,
                    MapPermission::user_stack(),
                    true,
                ),
            ],
        }
    }

    pub fn regions(&self) -> &[UserRegion] {
        &self.regions
    }

    pub fn validate(&self) {
        crate::println!("[user-mapping-builder-v38] validate begin");

        for region in self.regions() {
            crate::println!(
                "[user-mapping-builder-v38] region {:?} {:#x}..{:#x} mapped={} R={} W={} X={} U={}",
                region.kind,
                region.start,
                region.end,
                region.mapped,
                region.perm.read,
                region.perm.write,
                region.perm.execute,
                region.perm.user,
            );

            assert!(region.start < region.end);
            assert!(region.is_page_aligned());
            assert_eq!(region.size(), PAGE_SIZE);
        }

        for i in 0..self.regions.len() {
            for j in (i + 1)..self.regions.len() {
                assert!(!self.regions[i].overlaps(&self.regions[j]));
            }
        }

        let text = self.region(UserRegionKind::Text);
        assert!(text.mapped);
        assert!(text.perm.user);
        assert!(text.perm.read);
        assert!(text.perm.execute);
        assert!(!text.perm.write);

        let guard = self.region(UserRegionKind::Guard);
        assert!(!guard.mapped);
        assert!(guard.perm.user);
        assert!(!guard.perm.read);
        assert!(!guard.perm.write);
        assert!(!guard.perm.execute);

        let stack = self.region(UserRegionKind::Stack);
        assert!(stack.mapped);
        assert!(stack.perm.user);
        assert!(stack.perm.read);
        assert!(stack.perm.write);
        assert!(!stack.perm.execute);

        crate::println!("[user-mapping-builder-v38] text permissions ok");
        crate::println!("[user-mapping-builder-v38] guard page permissions ok");
        crate::println!("[user-mapping-builder-v38] stack permissions ok");
        crate::println!("[user-mapping-builder-v38] dry-run passed");
    }

    fn region(&self, kind: UserRegionKind) -> UserRegion {
        for region in self.regions() {
            if region.kind == kind {
                return *region;
            }
        }
        panic!("[user-mapping-builder-v38] region not found");
    }
}

pub fn init() {
    crate::println!("[mm::user_builder] scaffold init v38");
}

pub fn test() {
    let builder = UserAddressSpaceBuilder::new_smoke();
    builder.validate();
}
