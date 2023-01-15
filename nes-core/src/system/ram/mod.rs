//! Module for the RAM used by the 6502 CPU.

/// Start of RAM
pub const RAM_ADDR_START: u16 = 0x0000;

/// End of RAM
///
/// The NES RAM is only 2 KB but can be addressed
/// up until 0x1FFF (8 KB) due to mirrorring.
///
/// This means that each memory position of the RAM
/// can be accessed by 4 different addresses.
pub const RAM_ADDR_END: u16 = 0x1FFF;

/// One byte after the end of RAM.
///
/// Helps code that depends on range pattern matching.
pub const AFTER_RAM_END: u16 = 0x2000;

/// RAM size (2 KB)
pub const RAM_SIZE: usize = 1024 * 2;

pub const RAM_MIRROR: u16 = 0x07FF;

/// Random Access Memory
#[derive(Clone, Debug)]
pub struct Ram {
    mem: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Ram {
        Ram { mem: [0; RAM_SIZE] }
    }

    #[allow(dead_code)]
    pub fn write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data
    }

    #[allow(dead_code)]
    pub fn read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    /// Writes to the RAM with a mirror.
    ///
    /// The NES' RAM is only 2 kb, but addressable up to 8 kb.
    /// The mirror allows writing to addresses higher than the RAM's
    /// size, while not allowing overflow.
    pub fn write_mirrored(&mut self, addr: u16, data: u8, mirror: u16) {
        self.mem[(addr & mirror) as usize] = data
    }

    /// Reads from the RAM with a mirror.
    ///
    /// The NES' RAM is only 2 kb, but addressable up to 8 kb.
    /// The mirror allows reading from addresses higher than the RAM's
    /// size, while not allowing overflow.
    pub fn read_mirrored(&self, addr: u16, mirror: u16) -> u8 {
        self.mem[(addr & mirror) as usize]
    }
}

impl Default for Ram {
    fn default() -> Self {
        Ram::new()
    }
}
