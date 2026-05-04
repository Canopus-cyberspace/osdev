//! User memory copy helpers.
//!
//! v31 keeps Sv39 disabled, so user virtual addresses are currently direct
//! addresses in the kernel address space.  The important design point is that
//! syscalls no longer dereference user pointers directly; they go through this
//! module.  When Sv39 is enabled later, only this layer should need to change.

use core::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UserCopyError {
    NullPointer,
    LengthOverflow,
}

impl fmt::Display for UserCopyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NullPointer => write!(f, "null user pointer"),
            Self::LengthOverflow => write!(f, "user buffer length overflow"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct UserBuffer {
    ptr: usize,
    len: usize,
}

impl UserBuffer {
    pub fn new(ptr: usize, len: usize) -> Result<Self, UserCopyError> {
        if len > 0 && ptr == 0 {
            return Err(UserCopyError::NullPointer);
        }

        ptr.checked_add(len).ok_or(UserCopyError::LengthOverflow)?;

        Ok(Self { ptr, len })
    }

    pub const fn ptr(&self) -> usize {
        self.ptr
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn copy_to_kernel(&self, dst: &mut [u8]) -> Result<usize, UserCopyError> {
        let copy_len = core::cmp::min(self.len, dst.len());
        copy_from_user(self.ptr, &mut dst[..copy_len])?;
        Ok(copy_len)
    }
}

pub fn copy_from_user(src_user: usize, dst_kernel: &mut [u8]) -> Result<(), UserCopyError> {
    if dst_kernel.is_empty() {
        return Ok(());
    }

    if src_user == 0 {
        return Err(UserCopyError::NullPointer);
    }

    src_user
        .checked_add(dst_kernel.len())
        .ok_or(UserCopyError::LengthOverflow)?;

    unsafe {
        let src = core::slice::from_raw_parts(src_user as *const u8, dst_kernel.len());
        dst_kernel.copy_from_slice(src);
    }

    Ok(())
}

pub fn copy_to_user(src_kernel: &[u8], dst_user: usize) -> Result<(), UserCopyError> {
    if src_kernel.is_empty() {
        return Ok(());
    }

    if dst_user == 0 {
        return Err(UserCopyError::NullPointer);
    }

    dst_user
        .checked_add(src_kernel.len())
        .ok_or(UserCopyError::LengthOverflow)?;

    unsafe {
        let dst = core::slice::from_raw_parts_mut(dst_user as *mut u8, src_kernel.len());
        dst.copy_from_slice(src_kernel);
    }

    Ok(())
}

pub fn test_direct_user_copy() {
    let src = b"usercopy-v31";
    let mut dst = [0u8; 12];

    copy_from_user(src.as_ptr() as usize, &mut dst).expect("copy_from_user failed");

    assert_eq!(&dst, src);
    crate::println!("[mm::user_buffer] direct user copy test passed");
}
