//! Defines structures for rendering the background and foreground.

/// Used to render the background.
#[derive(Copy, Clone, Debug, Default)]
pub struct BackgroundData {
    pub next_tile_id: u8,
    pub next_tile_attrib: u8,
    pub next_tile_lsb: u8,
    pub next_tile_msb: u8,
    pub shifter_pattern_low: u8,
    pub shifter_pattern_high: u8,
    pub shifter_attrib_low: u8,
    pub shifter_attrib_high: u8,
}
