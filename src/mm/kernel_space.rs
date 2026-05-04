#[derive(Clone, Copy, Debug)]
pub struct KernelMapPermission {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

impl KernelMapPermission {
    pub const fn new(readable: bool, writable: bool, executable: bool) -> Self {
        Self {
            readable,
            writable,
            executable,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct KernelRegion {
    pub name: &'static str,
    pub start: usize,
    pub end: usize,
    pub perm: KernelMapPermission,
}

pub fn init() {
    crate::println!("[mm::kernel_space] scaffold init v35e");
}

pub fn test() {
    test_kernel_space_dry_run();
}

pub fn test_kernel_space_dry_run() {
    crate::println!("[kernel-space-v35e] dry-run begin");

    let text = KernelRegion {
        name: "text",
        start: symbol_stext(),
        end: symbol_etext(),
        perm: KernelMapPermission::new(true, false, true),
    };

    let rodata = KernelRegion {
        name: "rodata",
        start: symbol_srodata(),
        end: symbol_erodata(),
        perm: KernelMapPermission::new(true, false, false),
    };

    let data_bss = KernelRegion {
        name: "data_bss",
        start: symbol_sdata(),
        end: symbol_ebss(),
        perm: KernelMapPermission::new(true, true, false),
    };

    print_region(text);
    print_region(rodata);
    print_region(data_bss);

    assert!(text.start < text.end);
    assert!(rodata.start <= rodata.end);
    assert!(data_bss.start <= data_bss.end);

    assert!(text.perm.readable);
    assert!(!text.perm.writable);
    assert!(text.perm.executable);
    crate::println!("[kernel-space-v35e] text flags ok");

    assert!(rodata.perm.readable);
    assert!(!rodata.perm.writable);
    assert!(!rodata.perm.executable);
    crate::println!("[kernel-space-v35e] rodata flags ok");

    assert!(data_bss.perm.readable);
    assert!(data_bss.perm.writable);
    assert!(!data_bss.perm.executable);
    crate::println!("[kernel-space-v35e] data/bss flags ok");

    crate::println!("[kernel-space-v35e] dry-run passed");
}

fn print_region(region: KernelRegion) {
    crate::println!(
        "[kernel-space-v35e] region {} {:#x}..{:#x} R={} W={} X={}",
        region.name,
        region.start,
        region.end,
        region.perm.readable,
        region.perm.writable,
        region.perm.executable,
    );
}

fn symbol_stext() -> usize {
    stext as *const () as usize
}

fn symbol_etext() -> usize {
    etext as *const () as usize
}

fn symbol_srodata() -> usize {
    srodata as *const () as usize
}

fn symbol_erodata() -> usize {
    erodata as *const () as usize
}

fn symbol_sdata() -> usize {
    sdata as *const () as usize
}

fn symbol_ebss() -> usize {
    ebss as *const () as usize
}

unsafe extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn ebss();
}
