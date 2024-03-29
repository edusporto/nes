//! Module for the Bus used by a 6502 CPU.

use std::cell::RefCell;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::controller::{Controller, CTRL_ADDR_END, CTRL_ADDR_START};
use crate::system::ppu::{dma::Dma, Ppu, PPU_ADDR_END, PPU_ADDR_START};
use crate::system::ram::{Ram, RAM_ADDR_END, RAM_ADDR_START, RAM_MIRROR};

use super::ppu::dma::DMA_ADDR;

/// Contains the possible devices connected to the CPU.
#[derive(Clone, Debug)]
pub struct Bus {
    /// The console's Picture Processing Unit
    pub ppu: Ppu,
    /// Random Access Memory, 2 kb size with mirrorring up to 8 kb
    pub ram: Ram,

    pub controllers: [Controller; 2],
    controller_state: [Controller; 2],

    pub(crate) cartridge: Option<Rc<RefCell<Cartridge>>>,

    /// Allows the PPU to have direct access to memory
    pub(crate) dma: Dma,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ppu: Ppu::new(),
            ram: Ram::default(),

            controllers: [Controller::empty(); 2],
            controller_state: [Controller::empty(); 2],

            cartridge: None,
            dma: Dma::new(),
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        let rc = Rc::new(RefCell::new(cartridge));
        self.ppu.insert_cartridge(rc.clone());
        self.cartridge = Some(rc);
    }

    pub fn reset(&mut self) {
        self.ppu.reset();
        self.dma = Dma::default();
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

            if let Some(_mapped_data) = cart.cpu_map_write(addr, data) {
                return;
            }
        }

        match addr {
            RAM_ADDR_START..=RAM_ADDR_END => {
                // mirrors every 2kb (0x07FF)
                self.ram.write_mirrored(addr, data, RAM_MIRROR);
            }
            PPU_ADDR_START..=PPU_ADDR_END => {
                // mirrors `addr` into 8 entries
                self.ppu.cpu_write(addr & 0x07, data)
            }
            DMA_ADDR..=DMA_ADDR => {
                self.dma.page = data;
                self.dma.addr = 0x00;
                self.dma.transfer = true;
            }
            CTRL_ADDR_START..=CTRL_ADDR_END => {
                let which = addr as usize & 0x1;
                self.controller_state[which] = self.controllers[which];
            }
            _ => {} // _ => panic!("invalid address used to write to RAM: {:#4X}", addr), // TODO: should panic?
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let cart = self
            .cartridge
            .as_ref()
            .expect("No cartridge inserted!")
            .borrow();

        if let Some(mapped_data) = cart.cpu_map_read(addr) {
            return mapped_data;
        }

        match addr {
            RAM_ADDR_START..=RAM_ADDR_END => {
                // mirrors every 2kb
                self.ram.read_mirrored(addr, RAM_MIRROR)
            }
            PPU_ADDR_START..=PPU_ADDR_END => {
                // `Bus::read` is mutable because of this part
                // & 0x07 mirrors into 8 entries:
                self.ppu.cpu_read(addr & 0x07)
            }
            CTRL_ADDR_START..=CTRL_ADDR_END => {
                let which = addr as usize & 0x1;
                let data = u8::from(self.controller_state[which].bits() & 0x80 > 0);
                self.controller_state[which] =
                    Controller::from_bits_truncate(self.controller_state[which].bits() << 1);
                data
            }
            // TODO: find out if this should panic or not
            _ => 0,
            // _ => panic!("invalid address used to read from RAM, {:#4X}", addr),
        }
    }

    pub fn treat_dma_transfer(&mut self, clock_counter: u32) -> bool {
        if self.dma.transfer {
            if self.dma.dummy {
                // waiting to synchronise the CPU to the DMA
                if clock_counter % 2 == 1 {
                    // synchronise!
                    self.dma.dummy = false;
                }
            } else if clock_counter % 2 == 0 {
                // on even cycles, read data from the CPU address space
                self.dma.data = self.read((self.dma.page as u16) << 8 | self.dma.addr as u16);
            } else {
                // on odd cycles, write data to the PPU's OAM
                self.ppu.oam.set_byte(self.dma.addr, self.dma.data);
                self.dma.addr = self.dma.addr.wrapping_add(1);

                if self.dma.addr == 0x00 {
                    // the address wrapped back to 0, which means the transfer is over
                    self.dma.transfer = false;
                    self.dma.dummy = true;
                }
            }
            true
        } else {
            false
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
