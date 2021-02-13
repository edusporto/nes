//! Module representing a 6502 CPU.
//!
//! The CPU has the following external connections:
//! * Address (16-bit)
//! * Read/Write Data (8-bit)
//! * Read/Write flag
//! * Clock
//!
//! The CPU will be connected to a Bus by the address lines
//! and data lines. More devices will be connected to the Bus,
//! such as the Picture Processing Unit.
//!
//! For now, the only other device connected to the CPU will
//! be the RAM.
//!
//! Our CPU has three registers:
//! * A: Accumulator (8-bit)
//! * X (8-bit)
//! * Y (8-bit)
//! * STKP: Stack pointer (8-bit)
//! * PC: Program counter (16-bit)
//! * STATUS: Status flags (8-bit)
//!
//! The instructions performed by the CPU can have different
//! sizes: they can be 1 byte, 2 bytes or 3 bytes. This means
//! that some instructions will be executed in several clocks.
//!
//! With each instruction, we will have to deal with
//! * Function
//! * Address mode
//! * Cycles

mod flags;
mod instructions;
mod addressing;

use flags::CpuFlags;

/// Defines a CPU and its registers
pub struct Cpu {
    pub a: u8,            // Accumulator register
    pub x: u8,            // X register
    pub y: u8,            // Y register
    pub stkp: u8,         // Stack pointer
    pub pc: u16,          // Program counter
    pub status: CpuFlags, // STATUS register

    cycles: u8, // Contains the amount of cycles
                // remaining by the current function.
                // When it reaches 0, the next
                // instruction will execute.
    
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            stkp: 0,
            pc: 0,
            status: CpuFlags::empty(),

            cycles: 0,
        }
    }
}
