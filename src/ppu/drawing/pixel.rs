//! Color information for the NES' PPU.

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel { r: 0, g: 0, b: 0 }
    }
}

pub static ALL_COLORS: [Pixel; 64] = [
    Pixel::new(84, 84, 84),
    Pixel::new(0, 30, 116),
    Pixel::new(8, 16, 144),
    Pixel::new(48, 0, 136),
    Pixel::new(68, 0, 100),
    Pixel::new(92, 0, 48),
    Pixel::new(84, 4, 0),
    Pixel::new(60, 24, 0),
    Pixel::new(32, 42, 0),
    Pixel::new(8, 58, 0),
    Pixel::new(0, 64, 0),
    Pixel::new(0, 60, 0),
    Pixel::new(0, 50, 60),
    Pixel::new(0, 0, 0),
    Pixel::new(0, 0, 0),
    Pixel::new(0, 0, 0),
    Pixel::new(152, 150, 152),
    Pixel::new(8, 76, 196),
    Pixel::new(48, 50, 236),
    Pixel::new(92, 30, 228),
    Pixel::new(136, 20, 176),
    Pixel::new(160, 20, 100),
    Pixel::new(152, 34, 32),
    Pixel::new(120, 60, 0),
    Pixel::new(84, 90, 0),
    Pixel::new(40, 114, 0),
    Pixel::new(8, 124, 0),
    Pixel::new(0, 118, 40),
    Pixel::new(0, 102, 120),
    Pixel::new(0, 0, 0),
    Pixel::new(0, 0, 0),
    Pixel::new(0, 0, 0),
    Pixel::new(236, 238, 236),
    Pixel::new(76, 154, 236),
    Pixel::new(120, 124, 236),
    Pixel::new(176, 98, 236),
    Pixel::new(228, 84, 236),
    Pixel::new(236, 88, 180),
    Pixel::new(236, 106, 100),
    Pixel::new(212, 136, 32),
    Pixel::new(160, 170, 0),
    Pixel::new(116, 196, 0),
    Pixel::new(76, 208, 32),
    Pixel::new(56, 204, 108),
    Pixel::new(56, 180, 204),
    Pixel::new(60, 60, 60),
    Pixel::new(0, 0, 0),
    Pixel::new(0, 0, 0),
    Pixel::new(236, 238, 236),
    Pixel::new(168, 204, 236),
    Pixel::new(188, 188, 236),
    Pixel::new(212, 178, 236),
    Pixel::new(236, 174, 236),
    Pixel::new(236, 174, 212),
    Pixel::new(236, 180, 176),
    Pixel::new(228, 196, 144),
    Pixel::new(204, 210, 120),
    Pixel::new(180, 222, 120),
    Pixel::new(168, 226, 144),
    Pixel::new(152, 226, 180),
    Pixel::new(160, 214, 228),
    Pixel::new(160, 162, 160),
    Pixel::new(0, 0, 0),
    Pixel::new(0, 0, 0),
];
