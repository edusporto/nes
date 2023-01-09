pub mod cartridge;
pub mod controller;
pub mod screen;

pub(crate) mod system;

use crate::cartridge::Cartridge;
use crate::controller::Controller;
use crate::screen::Screen;
use system::System;

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;

#[derive(Clone, Debug, Default)]
pub struct Nes {
    system: System,
}

impl Nes {
    pub fn new(cartridge: Cartridge) -> Self {
        Nes {
            system: System::new(cartridge),
        }
    }

    pub fn screen(&self) -> &Screen<256, 240> {
        self.system.screen()
    }

    pub fn next_frame(&mut self) -> &Screen<256, 240> {
        self.system.next_frame()
    }

    pub fn controllers(&self) -> &[Controller; 2] {
        &self.system.controllers()
    }

    pub fn mut_controllers(&mut self) -> &mut [Controller; 2] {
        self.system.mut_controllers()
    }

    pub fn system_clock(&mut self) {
        self.system.clock();
    }

    pub fn system_reset(&mut self) {
        self.system.reset();
    }
}

impl std::fmt::Display for Nes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.system)
    }
}
