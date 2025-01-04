pub mod definitions;
mod implementations;

use super::{addressing::AddrMode, Cpu};
use definitions::LOOKUP_TABLE;

#[derive(Clone, Copy)]
pub struct Instruction {
    pub _name: &'static str,
    pub _opcode: u8,
    pub cycles: u8,
    pub addrmode: AddrMode,
    pub execute: fn(cpu: &mut Cpu) -> u8,
}

impl Instruction {
    pub fn lookup(opcode: u8) -> &'static Instruction {
        // It is impossible for the `opcode` index to be out
        // of bounds, since its value ranges from 0 to 255
        // and `LOOKUP_TABLE` has length 256.
        LOOKUP_TABLE[opcode as usize]
    }
}
