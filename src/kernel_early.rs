pub struct ArchInfo {
    pub name: &'static str,
}

pub fn common_kernel_init(arch: ArchInfo, write_str: fn(&str)) {
    write_str("[kernel] common init entered\n");
    write_str("[kernel] arch=");
    write_str(arch.name);
    write_str("\n");
}
