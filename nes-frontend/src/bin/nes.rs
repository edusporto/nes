use std::sync::{Arc, RwLock};

use instant::{Duration, Instant};
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use nes_core::screen::{NES_HEIGHT, NES_WIDTH};
use nes_frontend::arch;
use nes_frontend::game::Game;

const NES_SIZE: LogicalSize<u32> = LogicalSize::new(NES_WIDTH as u32, NES_HEIGHT as u32);

const FPS: u64 = 60;
const FRAME_TIME: Duration = Duration::from_micros(1_000_000 / FPS);

#[derive(Debug, Copy, Clone)]
enum CustomEvents {
    RequestRedraw,
}

fn main() {
    arch::prepare_env();
    async_std::task::block_on(run());
}

// fn main() {
//     WasmArch::prepare_env();
//     WasmArch::start_run(run());

//     NativeArch::prepare_env();
//     NativeArch::start_run(run());
// }

async fn run() {
    // TODO: do this better somehow

    // let file_name = std::env::args()
    //     .nth(1)
    //     .expect("Missing the file name to the desired ROM as argument.");
    // let file_name = "games/Super Mario Bros.nes";
    let cart = include_bytes!("../../../games/Super Mario Bros.nes");

    let event_loop = EventLoopBuilder::<CustomEvents>::with_user_event().build();
    let window = WindowBuilder::new()
        .with_title("NES")
        .with_inner_size(NES_SIZE)
        .with_min_inner_size(NES_SIZE)
        .build(&event_loop)
        .expect("WindowBuilder error");

    let window = Arc::new(window);

    arch::prepare_window(&window);

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
        let proxy = event_loop.create_proxy();
        // std::thread::spawn(move || {
        async_std::task::spawn_local(async move {
            let mut game = Game::start_from_bytes(cart).expect("Couldn't load game");
            let mut time = Instant::now();
            loop {
                if time.elapsed() < FRAME_TIME {
                    // spin_sleep::sleep(FRAME_TIME - time.elapsed());
                    // tokio::time::sleep(FRAME_TIME - time.elapsed()).await;
                    // async_std::task::sleep(FRAME_TIME - time.elapsed()).await
                    arch::sleep(FRAME_TIME - time.elapsed()).await;
                }

                while time.elapsed() < FRAME_TIME {}

                time = Instant::now();
                game.update_controllers(&input.read().unwrap());
                game.update();
                game.draw(pixels.write().unwrap().get_frame_mut());
                proxy.send_event(CustomEvents::RequestRedraw).unwrap();
            }
        });
    }

    let mut fps_avg = MovingAvg::new(5);
    let mut time = Instant::now();

    event_loop.run(move |event, _event_loop, control_flow| {
        control_flow.set_wait();

        if let Event::UserEvent(CustomEvents::RequestRedraw) = event {
            window.request_redraw();
        }

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // game.lock().unwrap().draw(pixels.get_frame_mut());
            if let Err(err) = pixels.read().unwrap().render() {
                error!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }

            fps_avg.add(1.0 / time.elapsed().as_secs_f64());
            time = Instant::now();
            window.set_title(&format!("NES (FPS: {:.1})", fps_avg.avg()));
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
                if let Err(err) = pixels
                    .write()
                    .unwrap()
                    .resize_surface(size.width, size.height)
                {
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

struct MovingAvg {
    window: usize,
    data: std::collections::VecDeque<f64>,
}

impl MovingAvg {
    fn new(window: usize) -> Self {
        MovingAvg {
            window,
            data: Default::default(),
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
