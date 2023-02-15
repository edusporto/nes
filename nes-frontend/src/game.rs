use fnv::FnvHashMap;
use nes_core::cartridge::{Cartridge, CartridgeError};
use nes_core::controller::Controller;
use nes_core::Nes;
use pixels::Pixels;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub struct GameState {
    nes: Option<Nes>,
    pub input: WinitInputHelper,
    pub input_map: FnvHashMap<VirtualKeyCode, Controller>,
    pub pixels: Pixels,
}

#[allow(dead_code)]
impl GameState {
    pub fn new(input: WinitInputHelper, pixels: Pixels) -> Self {
        GameState {
            nes: None,
            input,
            pixels,
            input_map: [
                (VirtualKeyCode::Up, Controller::UP),
                (VirtualKeyCode::Right, Controller::RIGHT),
                (VirtualKeyCode::Down, Controller::DOWN),
                (VirtualKeyCode::Left, Controller::LEFT),
                (VirtualKeyCode::Z, Controller::BUTTON_A),
                (VirtualKeyCode::X, Controller::BUTTON_B),
                (VirtualKeyCode::Space, Controller::START),
                (VirtualKeyCode::Back, Controller::SELECT),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    pub fn start_from_file(&mut self, file_name: &str) -> Result<(), CartridgeError> {
        self.nes = Some(Nes::new(Cartridge::from_file(file_name)?));
        Ok(())
    }

    pub fn start_from_bytes(&mut self, bytes: &[u8]) -> Result<(), CartridgeError> {
        self.nes = Some(Nes::new(Cartridge::from_bytes(bytes)?));
        Ok(())
    }

    pub fn draw(&mut self) {
        if let Some(nes) = self.nes.as_ref() {
            self.pixels
                .get_frame_mut()
                .chunks_exact_mut(4)
                .zip(nes.screen().flatten())
                .for_each(|(pixel_frame, pixel)| {
                    pixel_frame.copy_from_slice(&[pixel.r, pixel.g, pixel.b, 0xFF]);
                });
        }
    }

    pub fn update(&mut self) {
        if let Some(nes) = self.nes.as_mut() {
            nes.next_frame();
            self.update_controllers();
        }
    }

    pub fn update_controllers(&mut self) {
        let [controller1, controller2] = match self.nes.as_mut() {
            Some(nes) => nes.mut_controllers(),
            None => return,
        };

        *controller1 = Controller::empty();
        *controller2 = Controller::empty();

        for (&key, &button) in &self.input_map {
            if self.input.key_held(key) {
                controller1.set(button, true);
            }
        }
    }
}
