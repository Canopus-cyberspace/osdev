use core::sync::atomic::{AtomicUsize, Ordering};

const STDOUT_FD: usize = 1;
const STDERR_FD: usize = 2;

type OutputWriter = fn(&[u8]) -> usize;

static USER_OUTPUT_WRITER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserOutputError {
    SinkMissing,
    UnsupportedFd,
}

pub fn install_user_output_writer(writer: OutputWriter) {
    USER_OUTPUT_WRITER.store(writer as usize, Ordering::Release);
}

pub fn write_user_fd(fd: usize, bytes: &[u8]) -> Result<usize, UserOutputError> {
    if fd != STDOUT_FD && fd != STDERR_FD {
        return Err(UserOutputError::UnsupportedFd);
    }

    let raw = USER_OUTPUT_WRITER.load(Ordering::Acquire);
    if raw == 0 {
        return Err(UserOutputError::SinkMissing);
    }

    let writer: OutputWriter = unsafe { core::mem::transmute(raw) };
    Ok(writer(bytes))
}
