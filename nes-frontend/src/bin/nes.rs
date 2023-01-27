use std::error::Error;
use std::sync::{Arc, RwLock};

use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use nes_core::cartridge::Cartridge;
use nes_core::controller::Controller;
use nes_core::screen::{NES_HEIGHT, NES_WIDTH};
use nes_core::Nes;

use nes_frontend::arch::{native::NativeArch, wasm::WasmArch, TargetArch};

const NES_SIZE: LogicalSize<u32> = LogicalSize::new(NES_WIDTH as u32, NES_HEIGHT as u32);

fn main() {
    // TODO: find if there is a way to run these functions for all
    // implementations of the trait

    WasmArch::prepare_env();
    WasmArch::start_run(run());

    NativeArch::prepare_env();
    NativeArch::start_run(run());
}

async fn run() {
    // TODO: do this better somehow

    // let file_name = std::env::args()
    //     .nth(1)
    //     .expect("Missing the file name to the desired ROM as argument.");
    // let file_name = "games/Super Mario Bros.nes";
    let cart = include_bytes!("../../../games/Super Mario Bros.nes");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("NES")
        .with_inner_size(NES_SIZE)
        .with_min_inner_size(NES_SIZE)
        .build(&event_loop)
        .expect("WindowBuilder error");

    let window = Arc::new(window);

    WasmArch::prepare_window(&window);
    NativeArch::prepare_window(&window);

    let input = Arc::new(RwLock::new(WinitInputHelper::new()));
    let pixels = Arc::new(RwLock::new({
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
        Pixels::new_async(NES_WIDTH as u32, NES_HEIGHT as u32, surface_texture)
            .await
            .expect("Pixels error")
    }));

    // let mut game = Game::start(&file_name).expect(&format!("Couldn't load game in {}", file_name));
    // let game = Game::start_from_bytes(cart).expect("Couldn't load game");

    {
        let input = Arc::clone(&input);
        let pixels = Arc::clone(&pixels);
        let window = Arc::clone(&window);
        std::thread::spawn(move || {
            let mut game = Game::start_from_bytes(cart).expect("Couldn't load game");
            loop {
                {
                    game.update_controllers(&input.read().unwrap());
                    game.update();
                    game.draw(pixels.write().unwrap().get_frame_mut());
                    window.request_redraw();

                }
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        });
    }

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // game.lock().unwrap().draw(pixels.get_frame_mut());
            if let Err(err) = pixels.read().unwrap().render() {
                error!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        let mut input = input.write().unwrap();
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // game.write().unwrap().update_controllers(&input);

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.write().unwrap().resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            // game.update();
            // window.request_redraw();
        }
    });
}

struct Game(Nes);

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
