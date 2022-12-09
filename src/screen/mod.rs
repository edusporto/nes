//! Screen for the NES.
//!
//! Every pixel drawn by the PPU will be written to the screen.

pub mod pixel;

use itertools::Itertools;

use pixel::*;

#[derive(Debug, Clone)]
pub struct Screen<const WIDTH: usize, const HEIGHT: usize> {
    pixels: [[Pixel; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Screen<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Screen {
            pixels: [[Pixel::default(); WIDTH]; HEIGHT],
        }
    }

    pub fn enumerate(&self) -> impl Iterator<Item = ((usize, usize), &Pixel)> {
        (0..WIDTH)
            .cartesian_product(0..HEIGHT)
            .zip(self.pixels.iter().flatten())
    }

    pub fn flatten(&self) -> impl Iterator<Item = &Pixel> {
        self.pixels.iter().flatten()
    }

    pub fn pixels(&self) -> &[[Pixel; WIDTH]; HEIGHT] {
        &self.pixels
    }

    pub fn mut_pixels(&mut self) -> &mut [[Pixel; WIDTH]; HEIGHT] {
        &mut self.pixels
    }

    pub fn get_pixel(&self, (x, y): (usize, usize)) -> Pixel {
        if (0..HEIGHT).contains(&x) && (0..WIDTH).contains(&y) {
            self.pixels[x][y]
        } else {
            Pixel::default()
        }
    }

    pub fn set_pixel(&mut self, (x, y): (usize, usize), pixel: Pixel) {
        if (0..HEIGHT).contains(&x) && (0..WIDTH).contains(&y) {
            self.pixels[x][y] = pixel;
        }
    }
}

impl<const W: usize, const H: usize> Default for Screen<W, H> {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test() {
    let mut x: Screen<10, 10> = Screen::new();
    x.set_pixel((11, 11), Pixel::new(255, 255, 255));
    dbg!(x);
}
