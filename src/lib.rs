pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod mapper;
pub mod ppu;
pub mod ram;
pub mod screen;

use crate::bus::Bus;
use crate::cartridge::Cartridge;
use crate::screen::Screen;

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;

#[derive(Clone, Debug, Default)]
pub struct Nes {
    bus: Bus,
}

impl Nes {
    pub fn new() -> Self {
        Self { bus: Bus::new() }
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.bus.insert_cartridge(cartridge);
    }

    pub fn screen(&self) -> &Screen<256, 240> {
        self.bus.ppu.screen()
    }

    pub fn get_frame(&mut self) -> Option<&Screen<256, 240>> {
        self.bus.ppu.get_frame()
    }

    pub fn clock(&mut self) {
        self.bus.clock();
    }
}
