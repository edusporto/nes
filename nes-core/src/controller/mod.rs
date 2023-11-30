//! Module for the console's controllers.

use bitflags::bitflags;

pub const CTRL_ADDR_START: u16 = 0x4016;
pub const CTRL_ADDR_END: u16 = 0x4017;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Controller: u8 {
        const    RIGHT = 0b0000_0001;
        const     LEFT = 0b0000_0010;
        const     DOWN = 0b0000_0100;
        const       UP = 0b0000_1000;
        const    START = 0b0001_0000;
        const   SELECT = 0b0010_0000;
        const BUTTON_A = 0b0100_0000;
        const BUTTON_B = 0b1000_0000;
    }
}
