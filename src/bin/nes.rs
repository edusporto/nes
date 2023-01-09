use std::collections::VecDeque;
use std::error::Error;

use nes::cartridge::Cartridge;
use nes::controller::Controller;
use nes::screen::Screen;
use nes::{Nes, SCREEN_HEIGHT, SCREEN_WIDTH};

use minifb::{Key, Scale, Window, WindowOptions};

fn game(file_name: &str) -> Result<Nes, Box<dyn Error>> {
    let nes = Nes::new(Cartridge::from_file(file_name)?);
    Ok(nes)
}

fn update_buffer(buffer: &mut [u32], screen: &Screen<256, 240>) {
    screen.flatten().enumerate().for_each(|(i, pixel)| {
        let val: u32 = ((pixel.r as u32) << 16) | ((pixel.g as u32) << 8) | (pixel.b as u32);
        buffer[i] = val;
    });
}

fn update_controllers(window: &mut Window, nes: &mut Nes) {
    let [controller1, controller2] = nes.mut_controllers();
    *controller1 = Controller::empty();
    *controller2 = Controller::empty();

    window.get_keys().iter().for_each(|&key| match key {
        Key::Up => controller1.set(Controller::UP, true),
        Key::Down => controller1.set(Controller::DOWN, true),
        Key::Right => controller1.set(Controller::RIGHT, true),
        Key::Left => controller1.set(Controller::LEFT, true),
        Key::Z => controller1.set(Controller::BUTTON_A, true),
        Key::X => controller1.set(Controller::BUTTON_B, true),
        Key::Space => controller1.set(Controller::START, true),
        Key::Backspace => controller1.set(Controller::SELECT, true),
        _ => {}
    });
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = std::env::args()
        .nth(1)
        .ok_or("Missing the file name to the desired ROM as argument.")?;

    let mut nes = game(&file_name)?;

    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new(
        "NES emulator",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16666)));

    let mut fps_avg = MovingAvg::new(30);
    let mut time = std::time::SystemTime::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update_controllers(&mut window, &mut nes);
        update_buffer(&mut buffer, nes.next_frame());

        let fps = 1.0 / time.elapsed().unwrap().as_secs_f64();
        fps_avg.add(fps);
        time = std::time::SystemTime::now();

        window.set_title(&format!("NES (FPS: {:.1})", fps_avg.avg()));

        window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }

    Ok(())
}

struct MovingAvg {
    window: usize,
    data: VecDeque<f64>,
}

impl MovingAvg {
    fn new(window: usize) -> Self {
        MovingAvg {
            window,
            data: VecDeque::new(),
        }
    }

    fn add(&mut self, value: f64) {
        if self.data.len() == self.window {
            self.data.pop_front();
        }

        self.data.push_back(value);
    }

    fn avg(&self) -> f64 {
        self.data.iter().sum::<f64>() / self.data.len() as f64
    }
}
