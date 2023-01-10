//! Defines structures for rendering the background.

/// Used to render the background.
#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct BackgroundData {
    pub next_tile_id: u8,
    pub next_tile_attrib: u8,
    pub next_tile_lsb: u8,
    pub next_tile_msb: u8,
    pub shifter_pattern_low: u16,
    pub shifter_pattern_high: u16,
    pub shifter_attrib_low: u16,
    pub shifter_attrib_high: u16,
}
