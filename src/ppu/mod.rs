//! Module for the Picture Processing Unit.

use std::rc::Rc;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::cartridge::Cartridge;

pub const PPU_ADDR_START: u16 = 0x2000;
pub const PPU_ADDR_END: u16 = 0x3FFF;

#[derive(FromPrimitive)]
pub enum PPUReadWriteAddr {
    Control = 0,
    Mask = 1,
    Status = 2,
    OAMAddress = 3,
    OAMData = 4,
    Scroll = 5,
    PPUAddress = 6,
    PPUData = 7,
}

pub struct Ppu {
    name_table: [[u8; 1024]; 2],
    palette_table: [u8; 32],

    cartridge: Option<Rc<Cartridge>>,

    clock_counter: i16,
    scanline: i16,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            cartridge: None,
            name_table: [[0; 1024]; 2],
            palette_table: [0; 32],
            clock_counter: 0,
            scanline: 0,
        }
    }

    pub fn clock(&mut self) {
        self.clock_counter += 1;
        if self.clock_counter >= 341 {
            self.clock_counter = 0;

            self.scanline += 1;
            if self.scanline >= 261 {
                self.scanline = -1;
                // TODO: set `frame_complete` ?
            }
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Rc<Cartridge>) {
        self.cartridge = Some(cartridge)
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        use PPUReadWriteAddr::*;

        // only 8 entries
        let addr = addr & 0x07;

        match FromPrimitive::from_u16(addr) {
            Some(Control) => {}
            Some(Mask) => {}
            Some(Status) => {}
            Some(OAMAddress) => {}
            Some(OAMData) => {}
            Some(Scroll) => {}
            Some(PPUAddress) => {}
            Some(PPUData) => {}
            _ => {}
        }
    }

    pub fn cpu_read(&self, addr: u16) -> u8 {
        use PPUReadWriteAddr::*;

        // only 8 entries
        let addr = addr & 0x07;
        let data: u8 = 0;

        match FromPrimitive::from_u16(addr) {
            Some(Control) => {}
            Some(Mask) => {}
            Some(Status) => {}
            Some(OAMAddress) => {}
            Some(OAMData) => {}
            Some(Scroll) => {}
            Some(PPUAddress) => {}
            Some(PPUData) => {}
            _ => {}
        }

        data
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) {
        let addr: u16 = addr & PPU_ADDR_END;
    }

    pub fn ppu_read(&self, addr: u16) -> u8 {
        let data: u8 = 0;
        let addr = addr & PPU_ADDR_END;

        data
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
