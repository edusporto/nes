//! Module for the Bus used by a 6502 CPU.

use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::ppu::{Ppu, PPU_ADDR_END, PPU_ADDR_START};
use crate::ram::{RAM_END, RAM_START};

/// Contains the possible devices connected to the CPU.
#[derive(Clone, Debug)]
pub struct Bus {
    pub cpu: Cpu,
    pub ppu: Ppu,

    cartridge: Option<Rc<Cartridge>>,

    clock_counter: u32,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            cpu: Cpu::new(),
            ppu: Ppu::new(),

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
        self.cpu.reset();
        self.ppu.reset();
        self.clock_counter = 0;
    }

    pub fn clock(&mut self) {
        self.ppu.clock();

        if self.clock_counter % 3 == 0 {
            self.cpu.clock();
        }

        if self.ppu.interrupt_sent() {
            self.ppu.interrupt_done();
            self.cpu.nmi();
        }

        // TODO: Test if wrapping_add breaks anything
        self.clock_counter = self.clock_counter.wrapping_add(1);
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM_START..=RAM_END => {
                // mirrors every 2kb (0x07FF)
                self.cpu.write(addr, data)
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
                self.cpu.read(addr)
            }
            PPU_ADDR_START..=PPU_ADDR_END => {
                // mirrors into 8 entries;
                // `Bus::read` is mutable because of this
                self.ppu.cpu_read(addr)
            }
            _ => panic!("invalid address used to read from RAM"),
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
