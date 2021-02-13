//! Module for the RAM used by the 6502 CPU.

const MAX_MEM_SIZE: usize = std::u16::MAX as usize;

pub struct Ram {
    pub mem: [u8; MAX_MEM_SIZE],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            mem: [0; MAX_MEM_SIZE],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data
    }

    pub fn read(&self, addr: u16) -> u8{
        self.mem[addr as usize]
    }
}
