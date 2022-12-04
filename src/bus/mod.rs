//! Module for the Bus used by a 6502 CPU.

use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::ppu::{Ppu, PPU_ADDR_END, PPU_ADDR_START};
use crate::ram::{Ram, RAM_END, RAM_MIRROR, RAM_START};

/// Contains the possible devices connected to the CPU.
#[derive(Clone, Debug)]
pub struct Bus {
    /// The console's Picture Processing Unit
    pub ppu: Ppu,
    /// Random Access Memory, 2 kb size with mirrorring up to 8 kb
    pub ram: Ram,

    cartridge: Option<Rc<Cartridge>>,

    clock_counter: u32,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ppu: Ppu::new(),
            ram: Ram::default(),

            cartridge: None,

            clock_counter: 0,
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        let rc = Rc::new(cartridge);
        self.ppu.insert_cartridge(rc.clone());
        self.cartridge = Some(rc);
    }

    pub fn reset(&mut self) {
        self.ppu.reset();
        self.clock_counter = 0;
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM_START..=RAM_END => {
                // mirrors every 2kb (0x07FF)
                self.ram.write_mirrored(addr, data, RAM_MIRROR);
            }
            PPU_ADDR_START..=PPU_ADDR_END => {
                // mirrors into 8 entries
                self.ppu.cpu_write(addr, data)
            }
            _ => panic!("invalid address used to write to RAM"),
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END => {
                // mirrors every 2kb
                self.ram.read_mirrored(addr, RAM_MIRROR)
            }
            PPU_ADDR_START..=PPU_ADDR_END => {
                // mirrors into 8 entries;
                // `Bus::read` is mutable because of this
                self.ppu.cpu_read(addr)
            }
            // TODO: find out if this should panic or not
            _ => 0,
            // _ => panic!("invalid address used to read from RAM, {:#4X}", addr),
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
