//! Module for the Picture Processing Unit.

mod oam;
mod registers;
mod rendering;

use std::cell::RefCell;
use std::rc::Rc;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::cartridge::{Cartridge, CartridgeMirror};
use crate::ram::{AFTER_RAM_END, RAM_END, RAM_START};
use crate::screen::{pixel, Screen};
use oam::*;
use registers::*;

pub const PPU_ADDR_START: u16 = 0x2000;
pub const PPU_ADDR_END: u16 = 0x3FFF;

#[derive(FromPrimitive)]
pub enum PPUReadWriteAddr {
    ControlFlag = 0,
    MaskFlag = 1,
    StatusFlag = 2,
    OAMAddressFlag = 3,
    OAMDataFlag = 4,
    ScrollFlag = 5,
    PPUAddressFlag = 6,
    PPUDataFlag = 7,
}

#[derive(Clone, Debug)]
pub struct Ppu {
    screen: Screen<256, 240>,
    frame_complete: bool,

    name_table: [[u8; 1024]; 2],
    pattern_table: [[u8; 4096]; 2],
    palette_table: [u8; 32],

    cartridge: Option<Rc<RefCell<Cartridge>>>,

    /// Non-maskable interrupt; allows the PPU to send
    /// interrupts to the CPU
    nmi: bool,
    cycle: i16,
    scanline: i16,

    oam: Oam,
    oam_addr: u8,

    status: registers::StatusReg,
    mask: registers::MaskReg,
    control: registers::ControlReg,

    address_latch: u8,
    ppu_data_buffer: u8,

    /// VRAM address, used to index the VRAM
    vram_addr: RamAddrData,
    /// Temporary VRAM address, used for computations
    tram_addr: RamAddrData,
    /// Horizontal pixel offset. Vertical pixel offset
    /// is defined within `vram_addr`.
    fine_x: u8,

    /// Background data, used for rendering the background
    bg: rendering::BackgroundData,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            screen: Screen::default(),
            frame_complete: false,
            name_table: [[0; 1024]; 2],
            pattern_table: [[0; 4096]; 2],
            palette_table: [0; 32],
            cartridge: None,
            nmi: false,
            cycle: 0,
            scanline: 0,
            oam: Oam::default(),
            oam_addr: 0,
            status: StatusReg::empty(),
            mask: MaskReg::empty(),
            control: ControlReg::empty(),
            address_latch: 0,
            ppu_data_buffer: 0,
            vram_addr: RamAddrData(0),
            tram_addr: RamAddrData(0),
            fine_x: 0,
            bg: rendering::BackgroundData::default(),
        }
    }

    pub fn reset(&mut self) {
        self.fine_x = 0;
        self.address_latch = 0;
        self.ppu_data_buffer = 0;
        self.scanline = 0;
        self.cycle = 0;
        self.bg = rendering::BackgroundData::default();
        self.status = StatusReg::empty();
        self.control = ControlReg::empty();
        self.vram_addr = RamAddrData(0);
        self.tram_addr = RamAddrData(0);
    }

    pub fn screen(&self) -> &Screen<256, 240> {
        &self.screen
    }

    pub fn screen_ready(&mut self) -> bool {
        if self.frame_complete {
            self.frame_complete = false;
            true
        } else {
            false
        }
    }

    pub fn clock(&mut self) {
        self.frame_complete = false;

        match self.scanline {
            // rendering portion
            (-1..=239) => {
                if self.scanline == 0 && self.cycle == 0 {
                    // "odd frame" cycle skip
                    self.cycle = 1;
                }

                if self.scanline == -1 && self.cycle == 1 {
                    // start of new frame
                    self.status.set(StatusReg::VERTICAL_BLANK, false)
                }

                if (2..=257).contains(&self.cycle) || (321..=337).contains(&self.cycle) {
                    self.update_shifters();

                    // 8 cycle loop for background rendering
                    match (self.cycle - 1) % 8 {
                        0 => {
                            self.load_backgrond_shifters();

                            // fetch next background tile ID
                            self.bg.next_tile_id =
                                self.ppu_read(0x2000 | (self.vram_addr.0 & 0x0FFF));
                        }
                        2 => {
                            self.bg.next_tile_attrib = self.ppu_read(
                                0x23C0
                                    | ((self.vram_addr.nametable_y() as u16) << 11)
                                    | ((self.vram_addr.nametable_x() as u16) << 10)
                                    | ((self.vram_addr.coarse_y() >> 2) << 3)
                                    | (self.vram_addr.coarse_x() >> 2),
                            );

                            if self.vram_addr.coarse_y() & 0x02 != 0 {
                                // `>>=` is not the monadic bind :(
                                // x >>= y ≣ x = x >> y
                                self.bg.next_tile_attrib >>= 4;
                            }

                            if self.vram_addr.coarse_x() & 0x02 != 0 {
                                self.bg.next_tile_attrib >>= 2;
                            }

                            self.bg.next_tile_attrib &= 0x03;
                        }
                        4 => {
                            self.bg.next_tile_lsb = self.ppu_read(
                                ((self.control.contains(ControlReg::PATTERN_BACKGROUND) as u16)
                                    << 12)
                                    + ((self.bg.next_tile_id as u16) << 4)
                                    + (self.vram_addr.fine_y()),
                            );
                        }
                        6 => {
                            self.bg.next_tile_msb = self.ppu_read(
                                ((self.control.contains(ControlReg::PATTERN_BACKGROUND) as u16)
                                    << 12)
                                    + ((self.bg.next_tile_id as u16) << 4)
                                    + (self.vram_addr.fine_y() + 8),
                            );
                        }
                        7 => {
                            self.increment_scroll_x();
                        }
                        _ => { /* do nothing */ }
                    };
                }

                if self.cycle == 256 {
                    self.increment_scroll_y();
                }

                if self.cycle == 257 {
                    self.load_backgrond_shifters();
                    self.transfer_address_x();
                }

                if self.cycle == 338 || self.cycle == 340 {
                    self.bg.next_tile_id = self.ppu_read(0x2000 | (self.vram_addr.0 & 0x0FFF));
                }

                if self.scanline == -1 && (280..=304).contains(&self.cycle) {
                    self.transfer_address_y();
                }
            }
            240 => { /* do nothing! */ }
            (241..=260) => {
                if self.scanline == 241 && self.cycle == 1 {
                    self.status.set(StatusReg::VERTICAL_BLANK, true);

                    // PPU has finished drawing, send interrupt signal to the CPU.
                    // This allows the CPU to process data without interfering
                    // with the PPU's drawing
                    if self.control.contains(ControlReg::ENABLE_NMI) {
                        self.nmi = true;
                    }
                }
            }
            _ => {}
        }

        let mut bg_pixel: u8 = 0; // 2 bit pixel index
        let mut bg_palette: u8 = 0; // 3 bit palette index

        // render the background
        if self.mask.contains(MaskReg::RENDER_BACKGROUND) {
            // handle pixel selection with smooth scrolling
            let bit_mux: u16 = 0x8000 >> self.fine_x;

            // get pixel index
            let p0_pixel: u8 = u8::from((self.bg.shifter_pattern_low & bit_mux) > 0);
            let p1_pixel: u8 = u8::from((self.bg.shifter_pattern_high & bit_mux) > 0);
            bg_pixel = (p1_pixel << 1) | p0_pixel;

            // get palette index
            let bg_pal0 = u8::from((self.bg.shifter_attrib_low & bit_mux) > 0);
            let bg_pal1 = u8::from((self.bg.shifter_attrib_high & bit_mux) > 0);
            bg_palette = (bg_pal1 << 1) | bg_pal0;
        }

        self.screen.set_pixel(
            (self.scanline as usize, (self.cycle - 1) as usize),
            self.color_from_palette(bg_palette, bg_pixel),
        );

        self.cycle += 1;
        if self.cycle >= 341 {
            self.cycle = 0;

            self.scanline += 1;
            if self.scanline >= 261 {
                self.scanline = -1;
                self.frame_complete = true;
            }
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Rc<RefCell<Cartridge>>) {
        self.cartridge = Some(cartridge)
    }

    pub fn interrupt_sent(&self) -> bool {
        self.nmi
    }

    pub fn interrupt_done(&mut self) {
        self.nmi = false;
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        use PPUReadWriteAddr::*;

        let addr = addr & 0x07; // mirrors on 8 entries (3 bits)

        match FromPrimitive::from_u16(addr) {
            Some(ControlFlag) => {
                self.control = ControlReg::from_bits_truncate(data);
                self.tram_addr
                    .set_nametable_x(self.control.contains(ControlReg::NAMETABLE_X));
                self.tram_addr
                    .set_nametable_y(self.control.contains(ControlReg::NAMETABLE_Y));
            }
            Some(MaskFlag) => {
                self.mask = MaskReg::from_bits_truncate(data);
            }
            Some(StatusFlag) => {}
            Some(OAMAddressFlag) => {
                self.oam_addr = data;
            }
            Some(OAMDataFlag) => {
                self.oam.set_byte(self.oam_addr, data);
            }
            Some(ScrollFlag) => match self.address_latch {
                0 => {
                    // write contains X offset
                    self.fine_x = data & 0x07; // mirrors on 8 entries (3 bits)
                    self.tram_addr.set_coarse_x(data as u16 >> 3); // TODO: test
                    self.address_latch = 1;
                }
                _ => {
                    // write contains Y offset
                    self.tram_addr.set_fine_y((data & 0x07) as u16); // mirrors on 8 entries
                    self.tram_addr.set_coarse_y(data as u16 >> 3); // TODO: test
                    self.address_latch = 0;
                }
            },
            Some(PPUAddressFlag) => match self.address_latch {
                // allows the PPU address bus to be accessed by the CPU
                0 => {
                    // latches high byte of address
                    self.tram_addr =
                        RamAddrData(((data as u16 & 0x3F) << 8) | (self.tram_addr.0 & 0x00FF));
                    self.address_latch = 1;
                }
                _ => {
                    // latches low byte of address
                    self.tram_addr = RamAddrData((self.tram_addr.0 & 0xFF00) | data as u16);
                    self.vram_addr = self.tram_addr;
                    self.address_latch = 0;
                }
            },
            Some(PPUDataFlag) => {
                self.ppu_write(self.vram_addr.0, data);

                // writes from PPU data increment the nametable
                // increments by 32 if on vertical mode,
                // increments by 1 if on horizontal mode
                self.vram_addr = RamAddrData(
                    self.vram_addr.0
                        + if self.control.contains(ControlReg::INCREMENT_MODE) {
                            32
                        } else {
                            1
                        },
                )
            }
            _ => {}
        }
    }

    pub fn cpu_read(&mut self, addr: u16) -> u8 {
        use PPUReadWriteAddr::*;

        // only 8 entries
        let addr = addr & 0x07;
        let mut data: u8 = 0;

        match FromPrimitive::from_u16(addr) {
            Some(ControlFlag) => {}
            Some(MaskFlag) => {}
            Some(StatusFlag) => {
                // resets some parts of the circuit,
                // bottom 5 bits of the status flag contains noise that may be
                // used by games
                data = (self.status.bits() & 0xE0) | (self.ppu_data_buffer & 0x1F);
                self.status.set(StatusReg::VERTICAL_BLANK, false);
                self.address_latch = 0;
            }
            Some(OAMAddressFlag) => {}
            Some(OAMDataFlag) => {
                data = self.oam.get_byte(self.oam_addr);
            }
            Some(ScrollFlag) => {}
            Some(PPUAddressFlag) => {}
            Some(PPUDataFlag) => {
                // reads the PPU data with 1 cycle of delay
                data = self.ppu_data_buffer;
                // prepares the buffer for the next cycle
                self.ppu_data_buffer = self.ppu_read(self.vram_addr.0);

                // if the address was in the palette range, don't delay
                if self.vram_addr.0 >= 0x3F00 {
                    data = self.ppu_data_buffer;
                }

                // reads from PPU data increment the nametable address
                // the same way it did on `self.cpu_write`
                self.vram_addr = RamAddrData(
                    self.vram_addr.0
                        + if self.control.contains(ControlReg::INCREMENT_MODE) {
                            32
                        } else {
                            1
                        },
                )
            }
            _ => {}
        }

        data
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) {
        let addr: u16 = addr & PPU_ADDR_END;

        let mut cart = self
            .cartridge
            .as_mut()
            .expect("No cartridge inserted!")
            .borrow_mut();
        let (mapped, _mapped_data) = cart.ppu_map_write(addr, data);

        if mapped {
            return;
        }

        match addr {
            (0..=RAM_END) => {
                self.pattern_table[(addr as usize & 0x1000) >> 12][addr as usize & 0x0FFF] = data;
            }
            (AFTER_RAM_END..=0x3EFF) => match cart.mirror {
                CartridgeMirror::Vertical => match addr & 0x0FFF {
                    (0x0000..=0x03FF) | (0x0800..=0x0BFF) => {
                        self.name_table[0][addr as usize & 0x03FF] = data;
                    }
                    (0x0400..=0x07FF) | (0x0C00..=0x0FFF) => {
                        self.name_table[1][addr as usize & 0x03FF] = data;
                    }
                    _ => { /* can't happen due to the mirror! */ }
                },
                CartridgeMirror::Horizontal => match addr & 0x0FFF {
                    (0x0000..=0x07FF) => {
                        self.name_table[0][addr as usize & 0x03FF] = data;
                    }
                    (0x0800..=0x0FFF) => {
                        self.name_table[1][addr as usize & 0x03FF] = data;
                    }
                    _ => { /* can't happen due to the mirror! */ }
                },
                _ => {}
            },
            (0x3F00..=0x3FFF) => {
                let addr = addr & 0x001F;
                let addr = match addr {
                    0x0010 => 0x0000,
                    0x0014 => 0x0004,
                    0x0018 => 0x0008,
                    0x001C => 0x000C,
                    _ => addr,
                };
                self.palette_table[addr as usize] = data;
            }
            _ => {}
        }
    }

    pub fn ppu_read(&self, addr: u16) -> u8 {
        let addr = addr & PPU_ADDR_END;
        let mut data: u8 = 0;

        let cart = self
            .cartridge
            .as_ref()
            .expect("No cartridge inserted!")
            .borrow();
        let (mapped, mapped_data) = cart.ppu_map_read(addr);

        if mapped {
            return mapped_data;
        }

        match addr {
            (RAM_START..=RAM_END) => {
                data = self.pattern_table[(addr as usize & 0x1000) >> 12][addr as usize & 0x0FFF];
            }
            (AFTER_RAM_END..=0x3EFF) => match cart.mirror {
                CartridgeMirror::Vertical => match addr & 0x0FFF {
                    (0x0000..=0x03FF) | (0x0800..=0x0BFF) => {
                        data = self.name_table[0][addr as usize & 0x03FF];
                    }
                    (0x0400..=0x07FF) | (0x0C00..=0x0FFF) => {
                        data = self.name_table[1][addr as usize & 0x03FF];
                    }
                    _ => { /* can't happen due to the mirror! */ }
                },
                CartridgeMirror::Horizontal => match addr & 0x0FFF {
                    (0x0000..=0x07FF) => {
                        data = self.name_table[0][addr as usize & 0x03FF];
                    }
                    (0x0800..=0x0FFF) => {
                        data = self.name_table[1][addr as usize & 0x03FF];
                    }
                    _ => { /* can't happen due to the mirror! */ }
                },
                _ => {}
            },
            (0x3F00..=0x3FFF) => {
                let addr = addr & 0x001F;
                let addr = match addr {
                    0x0010 => 0x0000,
                    0x0014 => 0x0004,
                    0x0018 => 0x0008,
                    0x001C => 0x000C,
                    _ => addr,
                };
                data = self.palette_table[addr as usize]
                    & if self.mask.contains(MaskReg::GRAYSCALE) {
                        0x30
                    } else {
                        0x3F
                    }
            }
            _ => {}
        };

        data
    }

    /// Returns a color from a palette and pixel index.
    pub fn color_from_palette(&self, palette_index: u8, pixel_index: u8) -> pixel::Pixel {
        // - 0x3F00 is the PPU offset where palettes are stored
        // - Each palette is 4 bytes
        // - Each pixel index if an integer from 0 to 3
        // - The mirror "& 0x3F" prevents indexing `ALL_COLORS` out of bounds
        let index = self.ppu_read(0x3F00 + (palette_index as u16 * 4) + pixel_index as u16) & 0x3F;
        pixel::ALL_COLORS[index as usize]
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
