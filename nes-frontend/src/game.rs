use std::error::Error;

use nes_core::cartridge::Cartridge;
use nes_core::controller::Controller;
use nes_core::Nes;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub struct Game(Nes);

#[allow(dead_code)]
impl Game {
    pub fn start(file_name: &str) -> Result<Game, Box<dyn Error>> {
        let nes = Nes::new(Cartridge::from_file(file_name)?);
        Ok(Game(nes))
    }

    pub fn start_from_bytes(rom: &[u8]) -> Result<Game, Box<dyn Error>> {
        let nes = Nes::new(Cartridge::from_bytes(rom)?);
        Ok(Game(nes))
    }

    pub fn draw(&self, frame: &mut [u8]) {
        frame
            .chunks_exact_mut(4)
            .zip(self.0.screen().flatten())
            .for_each(|(pixel_frame, pixel)| {
                pixel_frame.copy_from_slice(&[pixel.r, pixel.g, pixel.b, 0xFF]);
            });
    }

    pub fn update(&mut self) {
        self.0.next_frame();
    }

    pub fn update_controllers(&mut self, input: &WinitInputHelper) {
        let [controller1, controller2] = self.0.mut_controllers();
        *controller1 = Controller::empty();
        *controller2 = Controller::empty();

        // TODO: do this better somehow
        if input.key_held(VirtualKeyCode::Up) {
            controller1.set(Controller::UP, true);
        }
        if input.key_held(VirtualKeyCode::Right) {
            controller1.set(Controller::RIGHT, true);
        }
        if input.key_held(VirtualKeyCode::Down) {
            controller1.set(Controller::DOWN, true);
        }
        if input.key_held(VirtualKeyCode::Left) {
            controller1.set(Controller::LEFT, true);
        }
        if input.key_held(VirtualKeyCode::Z) {
            controller1.set(Controller::BUTTON_A, true);
        }
        if input.key_held(VirtualKeyCode::X) {
            controller1.set(Controller::BUTTON_B, true);
        }
        if input.key_held(VirtualKeyCode::Space) {
            controller1.set(Controller::START, true);
        }
        if input.key_held(VirtualKeyCode::Back) {
            controller1.set(Controller::SELECT, true);
        }
    }
}
