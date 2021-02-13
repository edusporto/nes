//! Module for the flags used by the 6502 CPU.

use bitflags::bitflags;

bitflags! {
    pub struct CpuFlags: u8 {
        const C = 0b00000001; // Carry
        const Z = 0b00000010; // Zero
        const I = 0b00000100; // Disable interrupts
        const D = 0b00001000; // Decimal mode
        const B = 0b00010000; // Break
        const U = 0b00100000; // Unused
        const V = 0b01000000; // Overflow
        const N = 0b10000000; // Negative
    }
}
