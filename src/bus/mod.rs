//! Module for the Bus used by a 6502 CPU.

use crate::ram::Ram;

// The following constants will be expanded in the future.
// Their purpose is to define the ranges of address that
// each device connected to the Bus represents.

/// Start of RAM
const RAM_START: u16 = 0x0000;
/// End of RAM
///
/// The NES RAM is only 2 KB but can be addressed
/// up until 0x1FFF (8 KB) due to mirrorring.
///
/// This means that each memory position of the RAM
/// can be accessed by 4 different addresses.
const RAM_END: u16 = 0x1FFF;
/// RAM size (2 KB)
pub const RAM_SIZE: usize = 1024 * 2;

/// Contains the possible devices on the CPU.
pub struct Bus {
    pub ram: Ram,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: Ram::new(RAM_SIZE),
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM_START..=RAM_END => {
                let real_addr = addr & RAM_END;
                self.ram.write(real_addr, data);
            },
            _ => panic!("invalid address used to write to RAM"),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END => {
                let real_addr = addr & RAM_END; 
                self.ram.read(real_addr)
            },
            _ => panic!("invalid address used to read from RAM"),
        }
    }
}
