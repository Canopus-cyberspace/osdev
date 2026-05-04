use crate::config::PAGE_SIZE;

const KERNEL_TEXT_R: bool = true;
const KERNEL_TEXT_W: bool = false;
const KERNEL_TEXT_X: bool = true;

const KERNEL_RODATA_R: bool = true;
const KERNEL_RODATA_W: bool = false;
const KERNEL_RODATA_X: bool = false;

const KERNEL_DATA_R: bool = true;
const KERNEL_DATA_W: bool = true;
const KERNEL_DATA_X: bool = false;

#[derive(Copy, Clone)]
pub struct MapPermission {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

impl MapPermission {
    pub const fn new(readable: bool, writable: bool, executable: bool) -> Self {
        Self {
            readable,
            writable,
            executable,
        }
    }
}

pub struct KernelMapRegion {
    pub name: &'static str,
    pub start: usize,
    pub end: usize,
    pub permission: MapPermission,
}

pub struct KernelAddressSpaceBuilder {
    regions: [KernelMapRegion; 4],
}

impl KernelAddressSpaceBuilder {
    pub fn new() -> Self {
        Self {
            regions: [
                KernelMapRegion {
                    name: "text",
                    start: align_down(addr_stext(), PAGE_SIZE),
                    end: align_up(addr_etext(), PAGE_SIZE),
                    permission: MapPermission::new(KERNEL_TEXT_R, KERNEL_TEXT_W, KERNEL_TEXT_X),
                },
                KernelMapRegion {
                    name: "rodata",
                    start: align_down(addr_srodata(), PAGE_SIZE),
                    end: align_up(addr_erodata(), PAGE_SIZE),
                    permission: MapPermission::new(KERNEL_RODATA_R, KERNEL_RODATA_W, KERNEL_RODATA_X),
                },
                KernelMapRegion {
                    name: "data_bss",
                    start: align_down(addr_sdata(), PAGE_SIZE),
                    end: align_up(addr_ebss(), PAGE_SIZE),
                    permission: MapPermission::new(KERNEL_DATA_R, KERNEL_DATA_W, KERNEL_DATA_X),
                },
                KernelMapRegion {
                    name: "kernel_stacks",
                    start: align_down(addr_ebss(), PAGE_SIZE),
                    end: align_up(addr_ekernel(), PAGE_SIZE),
                    permission: MapPermission::new(KERNEL_DATA_R, KERNEL_DATA_W, KERNEL_DATA_X),
                },
            ],
        }
    }

    pub fn regions(&self) -> &[KernelMapRegion] {
        &self.regions
    }

    pub fn validate(&self) {
        for region in self.regions() {
            crate::println!(
                "[kernel-builder-v37] region {} {:#x}..{:#x} R={} W={} X={}",
                region.name,
                region.start,
                region.end,
                region.permission.readable,
                region.permission.writable,
                region.permission.executable
            );

            assert!(region.start <= region.end);
            assert_eq!(region.start % PAGE_SIZE, 0);
            assert_eq!(region.end % PAGE_SIZE, 0);
        }

        let text = &self.regions[0];
        assert!(text.permission.readable);
        assert!(!text.permission.writable);
        assert!(text.permission.executable);

        let rodata = &self.regions[1];
        assert!(rodata.permission.readable);
        assert!(!rodata.permission.writable);
        assert!(!rodata.permission.executable);

        let data_bss = &self.regions[2];
        assert!(data_bss.permission.readable);
        assert!(data_bss.permission.writable);
        assert!(!data_bss.permission.executable);

        let stacks = &self.regions[3];
        assert!(stacks.permission.readable);
        assert!(stacks.permission.writable);
        assert!(!stacks.permission.executable);
    }
}

pub fn init() {
    crate::println!("[mm::kernel_builder] scaffold init v37");
}

pub fn test() {
    crate::println!("[kernel-builder-v37] dry-run begin");

    let builder = KernelAddressSpaceBuilder::new();
    builder.validate();

    crate::println!("[kernel-builder-v37] permissions ok");
    crate::println!("[kernel-builder-v37] dry-run passed");
}

fn align_down(value: usize, align: usize) -> usize {
    value & !(align - 1)
}

fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn ebss();
    fn ekernel();
}

fn addr_stext() -> usize {
    stext as *const () as usize
}

fn addr_etext() -> usize {
    etext as *const () as usize
}

fn addr_srodata() -> usize {
    srodata as *const () as usize
}

fn addr_erodata() -> usize {
    erodata as *const () as usize
}

fn addr_sdata() -> usize {
    sdata as *const () as usize
}

fn addr_ebss() -> usize {
    ebss as *const () as usize
}

fn addr_ekernel() -> usize {
    ekernel as *const () as usize
}
