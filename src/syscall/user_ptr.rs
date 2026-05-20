#![allow(dead_code, unused_imports)]

pub use crate::mm::user_buffer::{
    copy_cstr_from_user, copy_from_user, copy_to_user, read_iovec_array, UserBuffer,
    UserCopyError, UserIovec, USER_COPY_CHUNK_SIZE,
};

pub fn read_user_usize(user_ptr: usize) -> Result<usize, UserCopyError> {
    if user_ptr == 0 {
        return Err(UserCopyError::NullPointer);
    }

    Ok(unsafe { core::ptr::read_volatile(user_ptr as *const usize) })
}
