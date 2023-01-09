//! Module for the register structs used by the PPU.

use bitfield::bitfield;
use bitflags::bitflags;

bitflags! {
    pub struct StatusReg: u8 {
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_ZERO_HIT = 0b0100_0000;
        const VERTICAL_BLANK  = 0b1000_0000;
    }

    pub struct MaskReg: u8 {
        const GRAYSCALE              = 0b0000_0001;
        const RENDER_BACKGROUND_LEFT = 0b0000_0010;
        const RENDER_SPRITES_LEFT    = 0b0000_0100;
        const RENDER_BACKGROUND      = 0b0000_1000;
        const RENDER_SPRITES         = 0b0001_0000;
        const ENHANCE_RED            = 0b0010_0000;
        const ENHANCE_GREEN          = 0b0100_0000;
        const ENHANCE_BLUE           = 0b1000_0000;
    }

    pub struct ControlReg: u8 {
        const NAMETABLE_X        = 0b0000_0001;
        const NAMETABLE_Y        = 0b0000_0010;
        const INCREMENT_MODE     = 0b0000_0100;
        const PATTERN_SPRITE     = 0b0000_1000;
        const PATTERN_BACKGROUND = 0b0001_0000;
        const SPRITE_SIZE        = 0b0010_0000;
        const SLAVE_MODE         = 0b0100_0000;
        const ENABLE_NMI         = 0b1000_0000;
    }
}

bitfield! {
    #[derive(Copy, Clone)]
    /// Based on the "Loopy register" found in the code
    /// by OneLoneCoder, originally by Loopy from the
    /// NES dev community
    pub struct RamAddrData(u16);
    impl Debug;
    pub coarse_x, set_coarse_x: 4, 0;
    pub coarse_y, set_coarse_y: 9, 5;
    pub nametable_x, set_nametable_x: 10;
    pub nametable_y, set_nametable_y: 11;
    /// Vertical pixel offset, bits 12 to 14
    pub fine_y, set_fine_y: 14, 12;
}

#[test]
/// The defined bitfield can easily panic if the
/// memory layout is wrong. This simple test catches
/// simple problems in the definition.
fn test_bitfield_panic() {
    let x = RamAddrData(0);
    dbg!(x);
}
