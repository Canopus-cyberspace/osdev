#[derive(Clone, Copy)]
pub struct FatalConsole {
    write: fn(&[u8]) -> usize,
}

impl FatalConsole {
    pub const fn new(write: fn(&[u8]) -> usize) -> Self {
        Self { write }
    }

    pub const fn writer(self) -> fn(&[u8]) -> usize {
        self.write
    }

    pub fn write(self, bytes: &[u8]) -> usize {
        (self.write)(bytes)
    }
}
