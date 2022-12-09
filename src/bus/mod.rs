//! Module for the Bus used by a 6502 CPU.

use std::cell::RefCell;
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

    cartridge: Option<Rc<RefCell<Cartridge>>>,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ppu: Ppu::new(),
            ram: Ram::default(),

            cartridge: None,
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        let rc = Rc::new(RefCell::new(cartridge));
        self.ppu.insert_cartridge(rc.clone());
        self.cartridge = Some(rc);
    }

    pub fn reset(&mut self) {
        self.ppu.reset();
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        {
            // scope for `cart`, allows the mutable borrow to end before
            // the ppu also tries to borrow `cart`

            let mut cart = self
                .cartridge
                .as_ref()
                .expect("No cartridge inserted!")
                .borrow_mut();
            let (mapped, _mapped_data) = cart.cpu_map_write(addr, data);

            if mapped {
                return;
            }
        }

        match addr {
            RAM_START..=RAM_END => {
                // mirrors every 2kb (0x07FF)
                self.ram.write_mirrored(addr, data, RAM_MIRROR);
            }
            PPU_ADDR_START..=PPU_ADDR_END => {
                // mirrors `addr` into 8 entries
                self.ppu.cpu_write(addr & 0x07, data)
            }
            _ => {}
            // _ => panic!("invalid address used to write to RAM: {:#4X}", addr), // TODO: should panic? 
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let cart = self
            .cartridge
            .as_ref()
            .expect("No cartridge inserted!")
            .borrow();
        let (mapped, mapped_data) = cart.cpu_map_read(addr);

        if mapped {
            return mapped_data;
        }

        match addr {
            RAM_START..=RAM_END => {
                // mirrors every 2kb
                self.ram.read_mirrored(addr, RAM_MIRROR)
            }
            PPU_ADDR_START..=PPU_ADDR_END => {
                // `Bus::read` is mutable because of this part
                // & 0x07 mirrors into 8 entries:
                self.ppu.cpu_read(addr & 0x07)
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
