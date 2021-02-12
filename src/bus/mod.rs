//! Module for the Bus used by a 6502 CPU.

pub struct Bus {}

impl Bus {
    pub fn write(addr: u16, data: u8) {}

    pub fn read(addr: u16, read_only: bool) {}
}
