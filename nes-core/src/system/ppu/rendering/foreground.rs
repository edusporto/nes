//! Defines structures for rendering the foreground.

use crate::system::ppu::oam::OamEntry;

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct ForegroundData {
    pub sprite_scanline: [OamEntry; 8],
    pub sprite_count: u8,
    pub sprite_shifter_pattern_low: [u8; 8],
    pub sprite_shifter_pattern_high: [u8; 8],

    pub sprite_zero_hit_possible: bool,
    pub sprite_zero_being_rendered: bool,
}
