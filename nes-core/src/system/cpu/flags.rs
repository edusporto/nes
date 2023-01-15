//! Module for the flags used by the 6502 CPU.

use bitflags::bitflags;

bitflags! {
    pub struct CpuFlags: u8 {
        /// Carry
        const C = 0b00000001;
        /// Zero
        const Z = 0b00000010;
        /// Disable interrupts
        const I = 0b00000100;
        /// Decimal mode
        const D = 0b00001000;
        /// Break
        const B = 0b00010000;
        /// Unused
        const U = 0b00100000;
        /// Overflow
        const V = 0b01000000;
        /// Negative
        const N = 0b10000000;
    }
}
