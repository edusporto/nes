//! Screen for the NES.
//!
//! Every pixel drawn by the PPU will be written to the screen.

pub mod pixel;

use itertools::Itertools;

use pixel::*;

pub const NES_WIDTH: usize = 256;
pub const NES_HEIGHT: usize = 240;

pub type NesScreen = Screen<NES_WIDTH, NES_HEIGHT>;

/// Represents a doubly buffered screen.
///
/// Write operations are done to the work buffer, read operations
/// are done from the draw buffer.
#[derive(Debug, Clone)]
pub struct Screen<const WIDTH: usize, const HEIGHT: usize> {
    /// Needs to be a box to avoid stack overflow
    buffer1: Box<[[Pixel; WIDTH]; HEIGHT]>,
    /// Needs to be a box to avoid stack overflow
    buffer2: Box<[[Pixel; WIDTH]; HEIGHT]>,
    work: WhichBuffer,
}

#[derive(Debug, Copy, Clone)]
enum WhichBuffer {
    One,
    Two,
}

impl<const WIDTH: usize, const HEIGHT: usize> Screen<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Screen {
            buffer1: Box::new([[Pixel::default(); WIDTH]; HEIGHT]),
            buffer2: Box::new([[Pixel::default(); WIDTH]; HEIGHT]),
            work: WhichBuffer::One,
        }
    }

    pub fn enumerate(&self) -> impl Iterator<Item = ((usize, usize), &Pixel)> {
        (0..WIDTH)
            .cartesian_product(0..HEIGHT)
            .zip(self.draw_buffer().iter().flatten())
    }

    pub fn flatten(&self) -> impl Iterator<Item = &Pixel> {
        self.draw_buffer().iter().flatten()
    }

    pub fn get_pixel(&self, (x, y): (usize, usize)) -> Pixel {
        if (0..HEIGHT).contains(&x) && (0..WIDTH).contains(&y) {
            self.draw_buffer()[x][y]
        } else {
            Pixel::default()
        }
    }

    pub fn set_pixel(&mut self, (x, y): (usize, usize), pixel: Pixel) {
        if (0..HEIGHT).contains(&x) && (0..WIDTH).contains(&y) {
            self.work_buffer_mut()[x][y] = pixel;
        }
    }

    pub fn draw_buffer(&self) -> &[[Pixel; WIDTH]; HEIGHT] {
        match self.work {
            WhichBuffer::One => &self.buffer2,
            WhichBuffer::Two => &self.buffer1,
        }
    }

    pub fn switch_buffer(&mut self) {
        self.work = match self.work {
            WhichBuffer::One => WhichBuffer::Two,
            WhichBuffer::Two => WhichBuffer::One,
        }
    }

    fn work_buffer_mut(&mut self) -> &mut [[Pixel; WIDTH]; HEIGHT] {
        match self.work {
            WhichBuffer::One => &mut self.buffer1,
            WhichBuffer::Two => &mut self.buffer2,
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
