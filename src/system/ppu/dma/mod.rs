//! Module for the Direct Memory Access (DMA).

/// If the CPU receives a write to this address, the DMA is initiated.
pub const DMA_ADDR: u16 = 0x4014;

/// The Direct Memory Access (DMA) enables the PPU to directly access the RAM
#[derive(Clone, Copy, Debug)]
pub struct Dma {
    pub(crate) page: u8,
    pub(crate) addr: u8,
    pub(crate) data: u8,

    pub(crate) transfer: bool,
    pub(crate) dummy: bool,
}

impl Dma {
    pub fn new() -> Self {
        Dma {
            page: 0,
            addr: 0,
            data: 0,

            transfer: false,
            dummy: true,
        }
    }
}

impl Default for Dma {
    fn default() -> Self {
        Self::new()
    }
}
