#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserCopyError {
    AddressOverflow,
    InvalidUserRange,
    NotMapped,
    PermissionDenied,
    Unsupported,
}

pub trait UserMemoryReader {
    fn read_user(&self, address: usize, out: &mut [u8]) -> Result<(), UserCopyError>;
}

pub trait UserMemoryWriter {
    fn write_user(&self, address: usize, input: &[u8]) -> Result<(), UserCopyError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserMapError {
    AddressOverflow,
    AlreadyMapped,
    FrameExhausted,
    InvalidRange,
    NotReady,
    PermissionDenied,
    Unsupported,
}

pub trait UserMemoryMapper {
    fn map_zeroed_user_pages(&self, start: usize, byte_len: usize) -> Result<(), UserMapError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NoUserMemory;

impl UserMemoryReader for NoUserMemory {
    fn read_user(&self, _address: usize, _out: &mut [u8]) -> Result<(), UserCopyError> {
        Err(UserCopyError::Unsupported)
    }
}

impl UserMemoryWriter for NoUserMemory {
    fn write_user(&self, _address: usize, _input: &[u8]) -> Result<(), UserCopyError> {
        Err(UserCopyError::Unsupported)
    }
}

impl UserMemoryMapper for NoUserMemory {
    fn map_zeroed_user_pages(&self, _start: usize, _byte_len: usize) -> Result<(), UserMapError> {
        Err(UserMapError::Unsupported)
    }
}

pub fn copy_from_user<R: UserMemoryReader>(
    reader: &R,
    address: usize,
    out: &mut [u8],
) -> Result<(), UserCopyError> {
    reader.read_user(address, out)
}

pub fn copy_to_user<W: UserMemoryWriter>(
    writer: &W,
    address: usize,
    input: &[u8],
) -> Result<(), UserCopyError> {
    writer.write_user(address, input)
}

pub fn map_zeroed_user_pages<M: UserMemoryMapper>(
    mapper: &M,
    start: usize,
    byte_len: usize,
) -> Result<(), UserMapError> {
    mapper.map_zeroed_user_pages(start, byte_len)
}
