//! Implementation of certain Mappers.
//!
//! If you are a user of this library, feel free to
//! add more mappers.

use crate::mapper::Mapper;
use crate::ram::RAM_END;

#[derive(Debug)]
pub struct Mapper0 {
    program_banks: u8,
    character_banks: u8,
}

impl Mapper0 {
    pub fn new(program_banks: u8, character_banks: u8) -> Self {
        Mapper0 {
            program_banks,
            character_banks,
        }
    }
}

impl Mapper for Mapper0 {
    fn cpu_map_read(&self, addr: u16) -> (bool, u32) {
        let mirror = if self.program_banks > 1 {
            // 32 KB ROM
            0x7FFF
        } else {
            // 16 KB
            0x3FFF
        };

        match addr {
            0x8000..=0xFFFF => (true, addr as u32 & mirror),
            _ => (false, addr as u32),
        }
    }

    fn cpu_map_write(&self, addr: u16) -> (bool, u32) {
        self.cpu_map_read(addr)
    }

    fn ppu_map_read(&self, addr: u16) -> (bool, u32) {
        match addr {
            (0x0000..=RAM_END) => (true, addr as u32),
            _ => (false, addr as u32),
        }
    }

    fn ppu_map_write(&self, addr: u16) -> (bool, u32) {
        match addr {
            (0x0000..=RAM_END) if self.character_banks == 0 => (true, addr as u32),
            _ => (false, addr as u32),
        }
    }
}
