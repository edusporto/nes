use std::error::Error;

use nes::cartridge::Cartridge;
use nes::screen::Screen;
use nes::{Nes, SCREEN_HEIGHT, SCREEN_WIDTH};

use minifb::{Key, Scale, Window, WindowOptions};

fn game() -> Result<Nes, Box<dyn Error>> {
    let nes = Nes::new(Cartridge::from_file("games/Micro Mages.nes")?);
    Ok(nes)
}

fn update_buffer(buffer: &mut [u32], screen: &Screen<256, 240>) {
    screen.flatten().enumerate().for_each(|(i, pixel)| {
        let val: u32 = ((pixel.r as u32) << 16) | ((pixel.g as u32) << 8) | (pixel.b as u32);
        buffer[i] = val;
    });
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new(
        "NES emulator",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            scale: Scale::FitScreen,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut nes = game()?;

    let mut time = std::time::SystemTime::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // update_buffer(&mut buffer, nes.screen());
        update_buffer(&mut buffer, nes.next_frame());

        window.set_title(&format!(
            "NES (FPS: {:.0})",
            1.0 / time.elapsed().unwrap().as_secs_f64()
        ));
        time = std::time::SystemTime::now();

        window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }

    Ok(())
}
