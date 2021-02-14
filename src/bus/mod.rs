//! Module for the Bus used by a 6502 CPU.

use crate::ram::Ram;

// The following constants will be expanded in the future.
// Their purpose is to define the ranges of address that
// each device connected to the Bus represents.

/// Start of
const ADDR_START: u16 = 0x0000;
const ADDR_END: u16 = 0xFFFF;

/// Contains the possible devices on the CPU.
pub struct Bus {
    pub ram: Ram,
}

impl Bus {
    pub fn new() -> Bus {
        Bus { ram: Ram::new() }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            ADDR_START..=ADDR_END => self.ram.write(addr, data),
            // _ => panic!("invalid address used to write to CPU"),
            // ^ unreachable pattern
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            ADDR_START..=ADDR_END => self.ram.read(addr),
            // _ => panic!("invalid address used to read from CPU"),
            // ^ unreachable pattern
        }
    }
}
