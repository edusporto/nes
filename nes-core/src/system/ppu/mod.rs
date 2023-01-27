//! Module for the Picture Processing Unit.

pub(crate) mod dma;
mod oam;
mod registers;
mod rendering;

use std::cell::RefCell;
use std::rc::Rc;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::cartridge::{Cartridge, CartridgeMirror};
use crate::screen::{pixel, NesScreen};
use crate::system::ram::{AFTER_RAM_END, RAM_ADDR_END, RAM_ADDR_START};

use oam::*;
use registers::*;
use rendering::background::BackgroundData;
use rendering::foreground::ForegroundData;

pub const PPU_ADDR_START: u16 = 0x2000;
pub const PPU_ADDR_END: u16 = 0x3FFF;

#[derive(FromPrimitive)]
pub enum PPUReadWriteFlags {
    Control = 0,
    Mask = 1,
    Status = 2,
    OAMAddress = 3,
    OAMData = 4,
    Scroll = 5,
    PPUAddress = 6,
    PPUData = 7,
}

#[derive(Clone, Debug)]
pub struct Ppu {
    screen: NesScreen,
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

    pub(crate) oam: Oam,
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
    bg: BackgroundData,

    /// Foreground data, used for rendering the foreground
    fg: ForegroundData,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            screen: NesScreen::default(),
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
            bg: BackgroundData::default(),
            fg: ForegroundData::default(),
        }
    }

    pub fn reset(&mut self) {
        self.fine_x = 0;
        self.address_latch = 0;
        self.ppu_data_buffer = 0;
        self.scanline = 0;
        self.cycle = 0;
        self.bg = BackgroundData::default();
        self.fg = ForegroundData::default();
        self.status = StatusReg::empty();
        self.mask = MaskReg::empty();
        self.control = ControlReg::empty();
        self.vram_addr = RamAddrData(0);
        self.tram_addr = RamAddrData(0);
    }

    pub fn screen(&self) -> &NesScreen {
        &self.screen
    }

    pub fn screen_ready(&mut self) -> bool {
        if self.frame_complete {
            self.frame_complete = false;
            self.screen.switch_buffer();
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
                    self.status.set(StatusReg::VERTICAL_BLANK, false);
                    self.status.set(StatusReg::SPRITE_OVERFLOW, false);
                    self.status.set(StatusReg::SPRITE_ZERO_HIT, false);

                    self.fg.sprite_shifter_pattern_low = [0; 8];
                    self.fg.sprite_shifter_pattern_high = [0; 8];
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

                // This point on (up until the end of the scope) deals with foreground rendering.
                // In this implementation, foreground rendering is not cycle-accurate to the NES.
                if self.cycle == 257 && self.scanline >= 0 {
                    // This part represents the end of the visible part of a scanline.
                    // We will now determine which sprites will be visible on the next line and
                    // preload information into the foreground buffers.

                    // Clear the sprite memory
                    self.fg.sprite_scanline = [OamEntry::from([0xFF, 0xFF, 0xFF, 0xFF]); 8];
                    self.fg.sprite_count = 0;
                    self.fg.sprite_zero_hit_possible = false;

                    // Clear residual information in sprite pattern shifters
                    self.fg.sprite_shifter_pattern_low = [0; 8];
                    self.fg.sprite_shifter_pattern_high = [0; 8];

                    for oam_index in 0..64 {
                        if self.fg.sprite_count >= 9 {
                            break;
                        }

                        // Cast to signed integers
                        let diff: i16 = self.scanline - self.oam.get_entry(oam_index).y as i16;

                        let sprite_size_cmp = if self.control.contains(ControlReg::SPRITE_SIZE) {
                            16
                        } else {
                            8
                        };

                        if diff >= 0 && diff < sprite_size_cmp {
                            // Ccanline at least as high as the sprite and resides in the sprite vertically
                            if self.fg.sprite_count < 8 {
                                if oam_index == 0 {
                                    // May trigger a sprite zero hit
                                    self.fg.sprite_zero_hit_possible = true;
                                }

                                self.fg.sprite_scanline[self.fg.sprite_count as usize] =
                                    self.oam.get_entry(oam_index);
                                self.fg.sprite_count += 1;
                            }
                        }
                    }

                    self.status
                        .set(StatusReg::SPRITE_OVERFLOW, self.fg.sprite_count > 8);

                    // By this point, the `self.fg.sprite_scanline` array has up to 8 visible sprites
                    // for the next scanline, which are ranked by priority.
                }

                if self.cycle == 340 {
                    // At the end of the scanline, extract the row patterns of each sprite

                    for i in 0..self.fg.sprite_count as usize {
                        let sprite = self.fg.sprite_scanline[i];

                        let mut sprite_pattern_bits_low: u8;
                        let mut sprite_pattern_bits_high: u8;
                        let sprite_pattern_addr_low: u16;
                        // let sprite_pattern_addr_high: u16; // defined later

                        if !self.control.contains(ControlReg::SPRITE_SIZE) {
                            // 8x8 sprite mode

                            if sprite.attribute & 0x80 == 0 {
                                // Sprite is not flipped vertically
                                sprite_pattern_addr_low =
                                    (u16::from(self.control.contains(ControlReg::PATTERN_SPRITE))
                                        << 12)
                                        | ((sprite.tile_id as u16) << 4)
                                        | (self.scanline - sprite.y as i16) as u16;
                            } else {
                                // Sprite is flipped vertically
                                sprite_pattern_addr_low =
                                    (u16::from(self.control.contains(ControlReg::PATTERN_SPRITE))
                                        << 12)
                                        | ((sprite.tile_id as u16) << 4)
                                        | (7 - (self.scanline - sprite.y as i16) as u16);
                            }
                        } else {
                            // 8x16 sprite mode
                            if sprite.attribute & 0x80 == 0 {
                                // Sprite is not flipped vertically
                                if self.scanline - (sprite.y as i16) < 8 {
                                    // Reading top half tile
                                    sprite_pattern_addr_low = ((sprite.tile_id as u16 & 0x01)
                                        << 12)
                                        | ((sprite.tile_id as u16 & 0xFE) << 4)
                                        | ((self.scanline - sprite.y as i16) as u16 & 0x07);
                                } else {
                                    // Reading bottom half tile
                                    sprite_pattern_addr_low = ((sprite.tile_id as u16 & 0x01)
                                        << 12)
                                        | (((sprite.tile_id as u16 & 0xFE) + 1) << 4)
                                        | ((self.scanline - sprite.y as i16) as u16 & 0x07);
                                }
                            } else {
                                // Sprite is flipped vertically
                                if self.scanline - (sprite.y as i16) < 8 {
                                    // Reading top half tile
                                    sprite_pattern_addr_low = ((sprite.tile_id as u16 & 0x01)
                                        << 12)
                                        | (((sprite.tile_id as u16 & 0xFE) + 1) << 4)
                                        | ((7 - (self.scanline - sprite.y as i16) as u16) & 0x07);
                                } else {
                                    // Reading bottom half tile
                                    sprite_pattern_addr_low = ((sprite.tile_id as u16 & 0x01)
                                        << 12)
                                        | ((sprite.tile_id as u16 & 0xFE) << 4)
                                        | ((7 - (self.scanline - sprite.y as i16) as u16) & 0x07);
                                }
                            }
                        }

                        let sprite_pattern_addr_high: u16 = sprite_pattern_addr_low + 8;

                        sprite_pattern_bits_low = self.ppu_read(sprite_pattern_addr_low);
                        sprite_pattern_bits_high = self.ppu_read(sprite_pattern_addr_high);

                        if sprite.attribute & 0x40 != 0 {
                            // sprite is flipped horizontally, we need to flip the pattern bytes
                            // `flip_byte` from https://stackoverflow.com/a/2602885
                            let flip_byte = |mut byte: u8| {
                                byte = (byte & 0xF0) >> 4 | (byte & 0x0F) << 4;
                                byte = (byte & 0xCC) >> 2 | (byte & 0x33) << 2;
                                byte = (byte & 0xAA) >> 1 | (byte & 0x55) << 1;
                                byte
                            };

                            sprite_pattern_bits_low = flip_byte(sprite_pattern_bits_low);
                            sprite_pattern_bits_high = flip_byte(sprite_pattern_bits_high);
                        }

                        self.fg.sprite_shifter_pattern_low[i] = sprite_pattern_bits_low;
                        self.fg.sprite_shifter_pattern_high[i] = sprite_pattern_bits_high;
                    }
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
        // end background

        // render the foreground
        let mut fg_pixel: u8 = 0;
        let mut fg_palette: u8 = 0;
        let mut fg_priority: u8 = 0;

        if self.mask.contains(MaskReg::RENDER_SPRITES) {
            self.fg.sprite_zero_being_rendered = false;

            for i in 0..self.fg.sprite_count as usize {
                let sprite = self.fg.sprite_scanline[i];

                if sprite.x == 0 {
                    // scanline collided with sprite, shifters take over
                    // fine x does not apply to sprites

                    // determine pixel value
                    let fg_pixel_low: u8 =
                        u8::from((self.fg.sprite_shifter_pattern_low[i] & 0x80) > 0);
                    let fg_pixel_high: u8 =
                        u8::from((self.fg.sprite_shifter_pattern_high[i] & 0x80) > 0);
                    fg_pixel = (fg_pixel_high << 1) | fg_pixel_low;

                    // extract the palette from bottom two bits
                    fg_palette = (sprite.attribute & 0x03) + 0x04;
                    fg_priority = u8::from((sprite.attribute & 0x20) == 0);

                    // if the pixel is not transparent, render it.
                    // don't bother with sprite order, since earlier sprites
                    // will have higher priority
                    if fg_pixel != 0 {
                        if i == 0 {
                            self.fg.sprite_zero_being_rendered = true;
                        }
                        break;
                    }
                }
            }
        }
        // end foreground

        // Now, we need to combine the background and foreground pixels.
        let mut pixel: u8 = 0;
        let mut palette: u8 = 0;

        use std::cmp::Ordering;
        match (bg_pixel.cmp(&0), fg_pixel.cmp(&0)) {
            (Ordering::Equal, Ordering::Equal) => {
                // Both pixels are transparent
                pixel = 0;
                palette = 0;
            }
            (Ordering::Equal, Ordering::Greater) => {
                // Background pixel is transparent
                pixel = fg_pixel;
                palette = fg_palette;
            }
            (Ordering::Greater, Ordering::Equal) => {
                // Foreground pixel is visible
                pixel = bg_pixel;
                palette = bg_palette;
            }
            (Ordering::Greater, Ordering::Greater) => {
                // Both pixels are visible
                if fg_priority != 0 {
                    pixel = fg_pixel;
                    palette = fg_palette;
                } else {
                    pixel = bg_pixel;
                    palette = bg_palette;
                }

                if self.fg.sprite_zero_hit_possible && self.fg.sprite_zero_being_rendered {
                    // Sprite zero collides between background and foreground,
                    // both must be enabled
                    if self
                        .mask
                        .contains(MaskReg::RENDER_BACKGROUND & MaskReg::RENDER_SPRITES)
                    {
                        if !self.mask.contains(
                            MaskReg::RENDER_BACKGROUND_LEFT | MaskReg::RENDER_SPRITES_LEFT,
                        ) {
                            if self.cycle >= 9 && self.cycle < 258 {
                                self.status.set(StatusReg::SPRITE_ZERO_HIT, true);
                            }
                        } else if self.cycle >= 1 && self.cycle < 258 {
                            self.status.set(StatusReg::SPRITE_ZERO_HIT, true);
                        }

                        // This is probably not the right way to do this, but it is how it
                        // was done on the reference emulator by Javidx9
                        // if !u8::from(!self.mask.contains(
                        //     MaskReg::RENDER_BACKGROUND_LEFT | MaskReg::RENDER_SPRITES_LEFT),
                        // ) != 0 {
                        //     if self.cycle >= 9 && self.cycle < 258 {
                        //         self.status.set(StatusReg::SPRITE_ZERO_HIT, true);
                        //     }
                        // } else if self.cycle >= 1 && self.cycle < 258 {
                        //     self.status.set(StatusReg::SPRITE_ZERO_HIT, true);
                        // }
                    }
                }
            }
            _ => { /* other arms are impossible, since fg_pixel and bg_pixel are unsigned */ }
        }

        // Finally draw the pixel!
        self.screen.set_pixel(
            (self.scanline as usize, (self.cycle - 1) as usize),
            self.color_from_palette(palette, pixel),
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
        use PPUReadWriteFlags::*;

        let addr = addr & 0x07; // mirrors on 8 entries (3 bits)

        match FromPrimitive::from_u16(addr) {
            Some(Control) => {
                self.control = ControlReg::from_bits_truncate(data);
                self.tram_addr
                    .set_nametable_x(self.control.contains(ControlReg::NAMETABLE_X));
                self.tram_addr
                    .set_nametable_y(self.control.contains(ControlReg::NAMETABLE_Y));
            }
            Some(Mask) => {
                self.mask = MaskReg::from_bits_truncate(data);
            }
            Some(Status) => {}
            Some(OAMAddress) => {
                self.oam_addr = data;
            }
            Some(OAMData) => {
                self.oam.set_byte(self.oam_addr, data);
            }
            Some(Scroll) => match self.address_latch {
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
            Some(PPUAddress) => match self.address_latch {
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
            Some(PPUData) => {
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
        use PPUReadWriteFlags::*;

        // only 8 entries
        let addr = addr & 0x07;
        let mut data: u8 = 0;

        match FromPrimitive::from_u16(addr) {
            Some(Control) => {}
            Some(Mask) => {}
            Some(Status) => {
                // resets some parts of the circuit,
                // bottom 5 bits of the status flag contains noise that may be
                // used by games
                data = (self.status.bits() & 0xE0) | (self.ppu_data_buffer & 0x1F);
                self.status.set(StatusReg::VERTICAL_BLANK, false);
                self.address_latch = 0;
            }
            Some(OAMAddress) => {}
            Some(OAMData) => {
                data = self.oam.get_byte(self.oam_addr);
            }
            Some(Scroll) => {}
            Some(PPUAddress) => {}
            Some(PPUData) => {
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

        if let Some(_mapped_data) = cart.ppu_map_write(addr, data) {
            return;
        }

        match addr {
            (0..=RAM_ADDR_END) => {
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

        if let Some(mapped_data) = cart.ppu_map_read(addr) {
            return mapped_data;
        }

        match addr {
            (RAM_ADDR_START..=RAM_ADDR_END) => {
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
