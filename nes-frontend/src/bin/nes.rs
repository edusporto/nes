use std::error::Error;
use std::rc::Rc;

use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use nes_core::cartridge::Cartridge;
use nes_core::controller::Controller;
use nes_core::{Nes, SCREEN_HEIGHT, SCREEN_WIDTH};

use nes_frontend::arch::{native::NativeArch, wasm::WasmArch, TargetArch};

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
    let cart = include_bytes!("../../../games/nestest.nes");

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("NES")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("WindowBuilder error")
    };

    let window = Rc::new(window);

    WasmArch::prepare_window(&window);
    NativeArch::prepare_window(&window);

    let mut input = WinitInputHelper::new();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
        Pixels::new_async(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)
            .await
            .expect("Pixels error")
    };

    // let mut game = Game::start(&file_name).expect(&format!("Couldn't load game in {}", file_name));
    let mut game = Game::start_from_raw(cart).expect(&format!("Couldn't load game"));

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            game.draw(pixels.get_frame_mut());
            if let Err(err) = pixels.render() {
                error!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            game.update_controllers(&input);

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            game.update();
            window.request_redraw();
        }
    });
}

struct Game(Nes);

impl Game {
    fn start(file_name: &str) -> Result<Game, Box<dyn Error>> {
        let nes = Nes::new(Cartridge::from_file(file_name)?);
        Ok(Game(nes))
    }

    fn start_from_raw(rom: &[u8]) -> Result<Game, Box<dyn Error>> {
        let nes = Nes::new(Cartridge::from_bytes(rom)?);
        Ok(Game(nes))
    }

    fn draw(&self, frame: &mut [u8]) {
        frame
            .chunks_exact_mut(4)
            .zip(self.0.screen().flatten())
            .for_each(|(pixel_frame, pixel)| {
                pixel_frame.copy_from_slice(&[pixel.r, pixel.g, pixel.b, 0xFF]);
            });
    }

    fn update(&mut self) {
        self.0.next_frame();
    }

    fn update_controllers(&mut self, input: &WinitInputHelper) {
        let [controller1, controller2] = self.0.mut_controllers();
        *controller1 = Controller::empty();
        *controller2 = Controller::empty();

        // TODO: do this better somehow
        if input.key_held(VirtualKeyCode::Up) {
            controller1.set(Controller::UP, true)
        };
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
