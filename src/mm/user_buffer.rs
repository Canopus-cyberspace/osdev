pub const USER_COPY_CHUNK_SIZE: usize = 64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UserCopyError {
    NullPointer,
    InvalidRange,
    BufferTooSmall,
}

impl UserCopyError {
    pub fn as_errno(self) -> isize {
        match self {
            Self::NullPointer => -14,
            Self::InvalidRange => -14,
            Self::BufferTooSmall => -22,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UserBuffer {
    ptr: usize,
    len: usize,
}

impl UserBuffer {
    pub fn new(ptr: usize, len: usize) -> Result<Self, UserCopyError> {
        if len == 0 {
            return Ok(Self { ptr, len });
        }

        if ptr == 0 {
            return Err(UserCopyError::NullPointer);
        }

        let buffer = Self { ptr, len };
        buffer.checked_range()?;
        Ok(buffer)
    }

    pub fn ptr(&self) -> usize {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn checked_range(&self) -> Result<(), UserCopyError> {
        if self.len == 0 {
            return Ok(());
        }

        self.ptr
            .checked_add(self.len - 1)
            .map(|_| ())
            .ok_or(UserCopyError::InvalidRange)
    }
}

pub fn copy_from_user(src: usize, len: usize, dst: &mut [u8]) -> Result<usize, UserCopyError> {
    let ubuf = UserBuffer::new(src, len)?;

    if len > dst.len() {
        return Err(UserCopyError::BufferTooSmall);
    }

    if ubuf.is_empty() {
        return Ok(0);
    }

    unsafe {
        core::ptr::copy_nonoverlapping(ubuf.ptr() as *const u8, dst.as_mut_ptr(), ubuf.len());
    }

    Ok(ubuf.len())
}

pub fn copy_to_user(dst: usize, src: &[u8]) -> Result<usize, UserCopyError> {
    let ubuf = UserBuffer::new(dst, src.len())?;

    if ubuf.is_empty() {
        return Ok(0);
    }

    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr(), ubuf.ptr() as *mut u8, ubuf.len());
    }

    Ok(ubuf.len())
}

pub fn copy_cstr_from_user(src: usize, dst: &mut [u8]) -> Result<usize, UserCopyError> {
    if src == 0 {
        return Err(UserCopyError::NullPointer);
    }
    if dst.is_empty() {
        return Err(UserCopyError::BufferTooSmall);
    }

    let mut len = 0usize;
    while len < dst.len() {
        let ch = unsafe { core::ptr::read_volatile((src + len) as *const u8) };
        if ch == 0 {
            return Ok(len);
        }
        dst[len] = ch;
        len += 1;
    }

    Err(UserCopyError::BufferTooSmall)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UserIovec {
    pub base: usize,
    pub len: usize,
}

impl UserIovec {
    pub const fn empty() -> Self {
        Self { base: 0, len: 0 }
    }
}

pub fn read_iovec_array(src: usize, iovcnt: usize, dst: &mut [UserIovec]) -> Result<usize, UserCopyError> {
    if iovcnt > dst.len() {
        return Err(UserCopyError::BufferTooSmall);
    }
    if iovcnt == 0 {
        return Ok(0);
    }
    UserBuffer::new(src, iovcnt * 16)?;

    let mut i = 0usize;
    while i < iovcnt {
        let entry = src + i * 16;
        let base = unsafe { core::ptr::read_volatile(entry as *const usize) };
        let len = unsafe { core::ptr::read_volatile((entry + 8) as *const usize) };
        UserBuffer::new(base, len)?;
        dst[i] = UserIovec { base, len };
        i += 1;
    }

    Ok(iovcnt)
}

pub fn test_direct_user_copy() {
    crate::println!("[mm::user_buffer] user-copy-v32e direct test begin");

    let src = b"user-copy-v32e";
    let mut dst = [0u8; 64];

    let copied = copy_from_user(src.as_ptr() as usize, src.len(), &mut dst).unwrap();
    assert_eq!(copied, src.len());
    assert_eq!(&src[..], &dst[..src.len()]);

    let mut out = [0u8; 64];
    let copied_back = copy_to_user(out.as_mut_ptr() as usize, &src[..]).unwrap();
    assert_eq!(copied_back, src.len());
    assert_eq!(&src[..], &out[..src.len()]);

    let ubuf = UserBuffer::new(src.as_ptr() as usize, src.len()).unwrap();
    let mut via_ubuf = [0u8; 64];
    let copied_via_ubuf = copy_from_user(ubuf.ptr(), ubuf.len(), &mut via_ubuf).unwrap();
    assert_eq!(copied_via_ubuf, src.len());
    assert_eq!(&src[..], &via_ubuf[..src.len()]);

    let empty = UserBuffer::new(0, 0).unwrap();
    assert!(empty.is_empty());
    assert_eq!(copy_from_user(0, 0, &mut dst).unwrap(), 0);
    assert_eq!(copy_to_user(0, &[]).unwrap(), 0);
    assert_eq!(UserBuffer::new(0, 1).err(), Some(UserCopyError::NullPointer));

    crate::println!("[mm::user_buffer] user-copy-v32e direct test passed");
}
