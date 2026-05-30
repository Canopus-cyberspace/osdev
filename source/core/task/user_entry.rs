use crate::core::mm::{UserAddressSpace, UserEntryAddress};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserRegisterImage {
    arg0: usize,
    arg1: usize,
}

impl UserRegisterImage {
    pub const fn new(arg0: usize, arg1: usize) -> Self {
        Self { arg0, arg1 }
    }

    pub const fn arg0(self) -> usize {
        self.arg0
    }

    pub const fn arg1(self) -> usize {
        self.arg1
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UserEntrySpec {
    entry: UserEntryAddress,
    stack_pointer: usize,
    registers: UserRegisterImage,
}

impl UserEntrySpec {
    pub const fn new(
        entry: UserEntryAddress,
        stack_pointer: usize,
        registers: UserRegisterImage,
    ) -> Self {
        Self {
            entry,
            stack_pointer,
            registers,
        }
    }

    pub const fn entry(self) -> UserEntryAddress {
        self.entry
    }

    pub const fn entry_pc(self) -> usize {
        self.entry.value()
    }

    pub const fn stack_pointer(self) -> usize {
        self.stack_pointer
    }

    pub const fn registers(self) -> UserRegisterImage {
        self.registers
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PendingUserEntry {
    address_space: UserAddressSpace,
    registers: UserRegisterImage,
}

impl PendingUserEntry {
    pub const fn new(address_space: UserAddressSpace, registers: UserRegisterImage) -> Self {
        Self {
            address_space,
            registers,
        }
    }

    pub const fn address_space(self) -> UserAddressSpace {
        self.address_space
    }

    pub const fn registers(self) -> UserRegisterImage {
        self.registers
    }
}
