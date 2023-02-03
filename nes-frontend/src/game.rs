use std::error::Error;

use nes_core::cartridge::Cartridge;
use nes_core::controller::Controller;
use nes_core::Nes;
use pixels::Pixels;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub struct Game {
    nes: Nes,
    pub input: WinitInputHelper,
    pub pixels: Pixels,
}

#[allow(dead_code)]
impl Game {
    pub fn start(
        file_name: &str,
        input: WinitInputHelper,
        pixels: Pixels,
    ) -> Result<Game, Box<dyn Error>> {
        let nes = Nes::new(Cartridge::from_file(file_name)?);
        Ok(Game { nes, input, pixels })
    }

    pub fn start_from_bytes(
        rom: &[u8],
        input: WinitInputHelper,
        pixels: Pixels,
    ) -> Result<Game, Box<dyn Error>> {
        let nes = Nes::new(Cartridge::from_bytes(rom)?);
        Ok(Game { nes, input, pixels })
    }

    pub fn draw(&mut self) {
        self.pixels
            .get_frame_mut()
            .chunks_exact_mut(4)
            .zip(self.nes.screen().flatten())
            .for_each(|(pixel_frame, pixel)| {
                pixel_frame.copy_from_slice(&[pixel.r, pixel.g, pixel.b, 0xFF]);
            });
    }

    pub fn update(&mut self) {
        self.nes.next_frame();
        self.update_controllers();
    }

    pub fn update_controllers(&mut self) {
        let [controller1, controller2] = self.nes.mut_controllers();
        *controller1 = Controller::empty();
        *controller2 = Controller::empty();

        // TODO: do this better somehow
        if self.input.key_held(VirtualKeyCode::Up) {
            controller1.set(Controller::UP, true);
        }
        if self.input.key_held(VirtualKeyCode::Right) {
            controller1.set(Controller::RIGHT, true);
        }
        if self.input.key_held(VirtualKeyCode::Down) {
            controller1.set(Controller::DOWN, true);
        }
        if self.input.key_held(VirtualKeyCode::Left) {
            controller1.set(Controller::LEFT, true);
        }
        if self.input.key_held(VirtualKeyCode::Z) {
            controller1.set(Controller::BUTTON_A, true);
        }
        if self.input.key_held(VirtualKeyCode::X) {
            controller1.set(Controller::BUTTON_B, true);
        }
        if self.input.key_held(VirtualKeyCode::Space) {
            controller1.set(Controller::START, true);
        }
        if self.input.key_held(VirtualKeyCode::Back) {
            controller1.set(Controller::SELECT, true);
        }
    }
}
