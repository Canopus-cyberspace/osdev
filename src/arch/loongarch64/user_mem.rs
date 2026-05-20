use crate::real_elf;

extern "C" {
    fn loongarch64_user_region_start();
    fn loongarch64_user_region_end();
}

pub(crate) fn user_range_valid(ptr: usize, len: usize) -> bool {
    let start = loongarch64_user_region_start as *const () as usize;
    let end = loongarch64_user_region_end as *const () as usize;
    match ptr.checked_add(len) {
        Some(limit) => (ptr >= start && limit <= end) || real_elf::user_range_valid(ptr, len),
        None => false,
    }
}

pub(crate) fn real_user_range_valid(ptr: usize, len: usize) -> bool {
    real_elf::user_range_valid(ptr, len)
}

pub(crate) fn copy_to_user(ptr: usize, src: &[u8]) -> Result<(), &'static str> {
    real_elf::copy_to_user(ptr, src)
}

#[allow(dead_code)]
pub(crate) fn copy_from_user(ptr: usize, dst: &mut [u8]) -> Result<(), &'static str> {
    real_elf::copy_from_user(ptr, dst)
}

pub(crate) fn read_user_cstr(ptr: usize, out: &mut [u8]) -> Result<usize, &'static str> {
    real_elf::read_user_cstr(ptr, out)
}

pub(crate) fn write_user_usize_pair(
    ptr: usize,
    first: usize,
    second: usize,
) -> Result<(), &'static str> {
    real_elf::write_user_usize_pair(ptr, first, second)
}
