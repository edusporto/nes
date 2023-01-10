pub(crate) mod background;
pub(crate) mod foreground;

use crate::system::ppu::{registers::MaskReg, Ppu};

impl Ppu {
    pub fn increment_scroll_x(&mut self) {
        if self.mask.contains(MaskReg::RENDER_BACKGROUND)
            || self.mask.contains(MaskReg::RENDER_SPRITES)
        {
            if self.vram_addr.coarse_x() == 31 {
                self.vram_addr.set_coarse_x(0);
                self.vram_addr
                    .set_nametable_x(!self.vram_addr.nametable_x());
            } else {
                self.vram_addr.set_coarse_x(self.vram_addr.coarse_x() + 1);
            }
        }
    }

    pub fn increment_scroll_y(&mut self) {
        if self.mask.contains(MaskReg::RENDER_BACKGROUND)
            || self.mask.contains(MaskReg::RENDER_SPRITES)
        {
            if self.vram_addr.fine_y() < 7 {
                self.vram_addr.set_fine_y(self.vram_addr.fine_y() + 1);
                return;
            }

            self.vram_addr.set_fine_y(0);

            if self.vram_addr.coarse_y() == 29 {
                self.vram_addr.set_coarse_y(0);
                self.vram_addr
                    .set_nametable_y(!self.vram_addr.nametable_y());
            } else if self.vram_addr.coarse_y() == 31 {
                self.vram_addr.set_coarse_y(0);
            } else {
                self.vram_addr.set_coarse_y(self.vram_addr.coarse_y() + 1);
            }
        }
    }

    pub fn transfer_address_x(&mut self) {
        if self.mask.contains(MaskReg::RENDER_BACKGROUND)
            || self.mask.contains(MaskReg::RENDER_SPRITES)
        {
            self.vram_addr.set_nametable_x(self.tram_addr.nametable_x());
            self.vram_addr.set_coarse_x(self.tram_addr.coarse_x());
        }
    }

    pub fn transfer_address_y(&mut self) {
        if self.mask.contains(MaskReg::RENDER_BACKGROUND)
            || self.mask.contains(MaskReg::RENDER_SPRITES)
        {
            self.vram_addr.set_fine_y(self.tram_addr.fine_y());
            self.vram_addr.set_nametable_y(self.tram_addr.nametable_y());
            self.vram_addr.set_coarse_y(self.tram_addr.coarse_y());
        }
    }

    pub fn load_backgrond_shifters(&mut self) {
        self.bg.shifter_pattern_low =
            (self.bg.shifter_pattern_low & 0xFF00) | self.bg.next_tile_lsb as u16;

        self.bg.shifter_pattern_high =
            (self.bg.shifter_pattern_high & 0xFF00) | self.bg.next_tile_msb as u16;

        self.bg.shifter_attrib_low = (self.bg.shifter_attrib_low & 0xFF00)
            | if self.bg.next_tile_attrib & 0b01 != 0 {
                0xFF
            } else {
                0x00
            };

        self.bg.shifter_attrib_high = (self.bg.shifter_attrib_high & 0xFF00)
            | if self.bg.next_tile_attrib & 0b10 != 0 {
                0xFF
            } else {
                0x00
            };
    }

    pub fn update_shifters(&mut self) {
        if self.mask.contains(MaskReg::RENDER_BACKGROUND) {
            self.bg.shifter_pattern_low <<= 1;
            self.bg.shifter_pattern_high <<= 1;

            self.bg.shifter_attrib_low <<= 1;
            self.bg.shifter_attrib_high <<= 1;
        }

        if self.mask.contains(MaskReg::RENDER_SPRITES) && self.cycle >= 1 && self.cycle < 258 {
            for i in 0..self.fg.sprite_count as usize {
                if self.fg.sprite_scanline[i].x > 0 {
                    self.fg.sprite_scanline[i].x = self.fg.sprite_scanline[i].x.wrapping_sub(1);
                } else {
                    self.fg.sprite_shifter_pattern_low[i] <<= 1;
                    self.fg.sprite_shifter_pattern_high[i] <<= 1;
                }
            }
        }
    }
}
