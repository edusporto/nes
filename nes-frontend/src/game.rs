use fnv::FnvHashMap;
use nes_core::cartridge::{Cartridge, CartridgeError};
use nes_core::controller::Controller;
use nes_core::Nes;
use pixels::Pixels;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::framework::Framework;
use crate::gui::GuiEvent;

pub struct GameState {
    nes: Option<Nes>,
    pub input: WinitInputHelper,
    pub input_map: FnvHashMap<VirtualKeyCode, Controller>,
    pub pixels: Pixels,
    pub framework: Framework,
}

#[allow(dead_code)]
impl GameState {
    pub fn new(input: WinitInputHelper, pixels: Pixels, framework: Framework) -> Self {
        GameState {
            nes: None,
            input,
            pixels,
            framework,
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

    pub fn start_from_file(&mut self, file_name: Option<&str>) -> Result<(), CartridgeError> {
        self.nes = match file_name {
            Some(file_name) => Some(Nes::new(Cartridge::from_file(file_name)?)),
            None => None,
        };
        Ok(())
    }

    pub fn start_from_bytes(&mut self, bytes: Option<&[u8]>) -> Result<(), CartridgeError> {
        self.nes = match bytes {
            Some(bytes) => Some(Nes::new(Cartridge::from_bytes(bytes)?)),
            None => None,
        };
        Ok(())
    }

    pub fn start_from_cartridge(&mut self, cart: Option<Cartridge>) {
        self.nes = cart.map(Nes::new);
    }

    pub fn restart(&mut self) {
        self.nes.as_mut().map(Nes::system_reset);
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
        self.treat_gui_events();
        self.update_controllers();
        if let Some(nes) = self.nes.as_mut() {
            nes.next_frame();
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

    pub fn treat_gui_events(&mut self) {
        for event in self.framework.gui.take_events() {
            match event {
                GuiEvent::ChangeRom(cart) => {
                    self.start_from_cartridge(cart);
                }
                GuiEvent::ToggleSettings => self.framework.gui.settings_window.toggle(),
            }
        }
    }
}
