pub(crate) mod bus;
pub(crate) mod cpu;
pub(crate) mod mapper;
pub(crate) mod ppu;
pub(crate) mod ram;

use crate::cartridge::Cartridge;
use crate::controller::Controller;
use crate::screen::NesScreen;
use cpu::Cpu;

#[derive(Clone, Debug, Default)]
pub(crate) struct System {
    cpu: Cpu,

    /// Used by the `self.system_clock` function
    /// to have the PPU clock faster than the CPU.
    clock_counter: u32,
}

impl System {
    pub fn new(cartridge: Cartridge) -> Self {
        let mut system = Self {
            cpu: Cpu::new(),
            clock_counter: 0,
        };
        system.cpu.bus.insert_cartridge(cartridge);
        system.reset();
        system
    }

    pub fn screen(&self) -> &NesScreen {
        self.cpu.bus.ppu.screen()
    }

    pub fn next_frame(&mut self) -> &NesScreen {
        while !self.cpu.bus.ppu.screen_ready() {
            self.clock();
        }

        self.screen()
    }

    pub fn controllers(&self) -> &[Controller; 2] {
        &self.cpu.bus.controllers
    }

    pub fn mut_controllers(&mut self) -> &mut [Controller; 2] {
        &mut self.cpu.bus.controllers
    }

    /// **System clock cycle**
    ///
    /// Executes a clock cycle for all parts of the console's internal system,
    /// namely, the CPU and PPU.
    pub fn clock(&mut self) {
        self.cpu.bus.ppu.clock();

        if self.clock_counter % 3 == 0 {
            // it may be time to clock the CPU, depending on the status of the DMA
            // let dma = &mut self.cpu.bus.dma;

            let dma_treated = self.cpu.bus.treat_dma_transfer(self.clock_counter);
            if !dma_treated {
                // the DMA isn't transferring data, the CPU is allowed to clock
                self.cpu.clock();
            }
        }

        if self.cpu.bus.ppu.interrupt_sent() {
            self.cpu.bus.ppu.interrupt_done();
            self.cpu.nmi();
        }

        // TODO: Test if wrapping_add breaks anything
        self.clock_counter = self.clock_counter.wrapping_add(1);
    }

    /// **System reset**
    ///
    /// Resets the CPU and the PPU.
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.cpu.bus.reset();
        self.clock_counter = 0;
    }
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cpu.bus.cartridge {
            Some(cart) => match cart.try_borrow() {
                Ok(cart) => write!(f, "NES online with cartridge \"{}\"", cart),
                Err(_) => write!(f, "NES online, cartridge being used"),
            },
            None => write!(f, "NES offline with no cartridge"),
        }
    }
}
