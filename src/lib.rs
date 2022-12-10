pub mod bus;
pub mod cartridge;
pub mod controller;
pub mod cpu;
pub mod mapper;
pub mod ppu;
pub mod ram;
pub mod screen;

use controller::Controller;

use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::screen::Screen;

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;

#[derive(Clone, Debug, Default)]
pub struct Nes {
    cpu: Cpu,
}

impl Nes {
    pub fn new(cartridge: Cartridge) -> Self {
        let mut nes = Self { cpu: Cpu::new() };
        nes.cpu.bus.insert_cartridge(cartridge);
        nes.cpu.system_reset();
        nes
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.cpu.bus.insert_cartridge(cartridge);
    }

    pub fn screen(&self) -> &Screen<256, 240> {
        self.cpu.bus.ppu.screen()
    }

    pub fn next_frame(&mut self) -> &Screen<256, 240> {
        while !self.cpu.bus.ppu.screen_ready() {
            self.system_clock();
        }

        self.screen()
    }

    pub fn controllers(&self) -> &[Controller; 2] {
        &self.cpu.bus.controllers
    }

    pub fn mut_controllers(&mut self) -> &mut [Controller; 2] {
        &mut self.cpu.bus.controllers
    }

    pub fn system_clock(&mut self) {
        self.cpu.system_clock();
    }

    pub fn system_reset(&mut self) {
        self.cpu.system_reset();
    }
}
