//! Module for the Bus used by a 6502 CPU.

use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::ppu::{Ppu, PPU_ADDR_END, PPU_ADDR_START};
use crate::ram::{RAM_END, RAM_START};

// The following constants will be expanded in the future.
// Their purpose is to define the ranges of address that
// each device connected to the Bus represents.

/// Contains the possible devices connected to the CPU.
pub struct Bus {
    pub cpu: Cpu,
    pub ppu: Ppu,

    pub cartridge: Option<Rc<Cartridge>>,

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

    pub fn insert_cartridge(&mut self, cartridge: Rc<Cartridge>) {
        self.ppu.insert_cartridge(cartridge.clone());
        self.cartridge = Some(cartridge);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.clock_counter = 0;
    }

    pub fn clock(&mut self) {
        self.ppu.clock();

        if self.clock_counter % 3 == 0 {
            self.cpu.clock();
        }

        // TODO: Test if wrapping_add breaks anything
        self.clock_counter = self.clock_counter.wrapping_add(1);
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM_START..=RAM_END => {
                // mirrors every 2kb (0x07FF)
                self.cpu.write(addr, data)
                // let real_addr = addr & 0x07FF;
                // self.ram.write(real_addr, data);
            }
            PPU_ADDR_START..=PPU_ADDR_END => {
                // mirrors into 8 entries
                self.ppu.cpu_write(addr, data)
            }
            _ => panic!("invalid address used to write to RAM"),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END => {
                // mirrors every 2kb
                self.cpu.read(addr)
            }
            PPU_ADDR_START..=PPU_ADDR_END => {
                // mirrors into 8 entries
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
