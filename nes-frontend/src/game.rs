use fnv::FnvHashMap;
use nes_core::cartridge::{Cartridge, CartridgeError};
use nes_core::controller::Controller;
use nes_core::Nes;
use pixels::Pixels;
use tokio::sync::mpsc::Receiver;
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
    pub receiver: Receiver<GuiEvent>,
}

#[allow(dead_code)]
impl GameState {
    pub fn new(
        input: WinitInputHelper,
        pixels: Pixels,
        framework: Framework,
        receiver: Receiver<GuiEvent>,
    ) -> Self {
        GameState {
            nes: None,
            input,
            pixels,
            framework,
            receiver,
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
                .frame_mut()
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
        while let Ok(event) = self.receiver.try_recv() {
            match event {
                GuiEvent::ChangeRom(Some((name, cart))) => {
                    self.framework.gui.settings_window.selected_cart_name = Some(name);
                    self.start_from_cartridge(Some(cart));
                }
                GuiEvent::ChangeRom(None) => self.start_from_cartridge(None),
                GuiEvent::ToggleSettings => self.framework.gui.settings_window.toggle(),
                GuiEvent::CartridgeError(message) => {
                    self.framework.gui.settings_window.toggle();
                    self.framework.gui.error_window.show(&message);
                }
            }
        }
    }

    pub fn treat_input(&mut self) {
        // Update the scale factor
        if let Some(scale_factor) = self.input.scale_factor() {
            self.framework.scale_factor(scale_factor as f32);
        }

        // Resize the window
        if let Some(size) = self.input.window_resized() {
            self.pixels.resize_surface(size.width, size.height).ok();
            self.framework.resize(size.width, size.height);
        }

        // Load ROM
        if let Some(file_name) = self.input.dropped_file() {
            let contents = std::fs::read(file_name).ok();
            self.start_from_bytes(contents.as_deref()).ok();
            self.framework.gui.settings_window.open = false;
        }

        // Show settings menu
        if self.input.key_pressed(VirtualKeyCode::Escape) {
            self.framework.gui.settings_window.toggle();
        }

        // Reset game
        if self.input.key_pressed(VirtualKeyCode::F5) {
            self.restart();
        }
    }
}
