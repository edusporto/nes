use std::error::Error;

use nes::cartridge::Cartridge;
use nes::{Nes, SCREEN_HEIGHT, SCREEN_WIDTH};

use minifb::{Key, Window, WindowOptions};

fn game() -> Result<Nes, Box<dyn Error>> {
    let mut nes = Nes::new();
    nes.insert_cartridge(Cartridge::from_file("games/Donkey Kong.nes")?);
    Ok(nes)
}

fn update_buffer(buffer: &mut [u32], nes: &mut Nes) -> bool {
    if let Some(screen) = nes.get_frame() {
        screen.flatten().enumerate().for_each(|(i, pixel)| {
            let val: u32 =
                ((pixel.r as u32) << 16) | ((pixel.g as u32) << 8) | ((pixel.b as u32) << 0);
            buffer[i] = val;
        });
        true
    } else {
        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut nes = game()?;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if update_buffer(&mut buffer, &mut nes) {
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window
                .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
                .unwrap();
        }

        nes.clock();
    }

    Ok(())
}
