//! Module for the RAM used by the 6502 CPU.

/// Random Access Memory
pub struct Ram {
    pub mem: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Ram {
        Ram {
            mem: vec![0; size],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }
}
