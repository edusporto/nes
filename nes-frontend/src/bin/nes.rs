#![windows_subsystem = "windows"]

use std::sync::Arc;

use instant::{Duration, Instant};
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

use nes_core::screen::{NES_HEIGHT, NES_WIDTH};
use nes_frontend::arch;
use nes_frontend::fps::FpsCounter;
use nes_frontend::game::GameState;

const NES_SIZE: LogicalSize<u32> = LogicalSize::new(NES_WIDTH as u32, NES_HEIGHT as u32);
const FPS: u32 = 60;
const FRAME_TIME: Duration = Duration::from_micros(1_000_000 / FPS as u64);

fn main() {
    arch::prepare_env();
    arch::start_run(run());
}

async fn run() {
    let (window, event_loop, input, pixels) = build_window().await;

    let game = GameState::new(input, pixels);
    let mut fps = FpsCounter::new(10);
    let mut time = Instant::now();

    game_loop::game_loop(
        event_loop,
        window,
        game,
        FPS,
        0.1,
        |g| {
            // Update function
            g.game.update();
            g.game.draw();
            g.window.request_redraw();
        },
        move |g| {
            // Render function
            if time.elapsed() < FRAME_TIME {
                arch::sleep(FRAME_TIME.saturating_sub(time.elapsed()));
            }

            time = Instant::now();

            if let Err(err) = g.game.pixels.render() {
                error!("pixels.render() failed: {err}");
                g.exit();
            }

            fps.update();
            g.window.set_title(&format!("NES (FPS: {:.1})", fps.avg()));
        },
        move |g, event| {
            // Window handler
            if g.game.input.update(event) {
                // Close events
                if g.game.input.quit() {
                    g.exit();
                }

                // Resize the window
                if let Some(size) = g.game.input.window_resized() {
                    g.game.pixels.resize_surface(size.width, size.height).ok();
                }
            }
        },
    );
}

async fn build_window() -> (Arc<Window>, EventLoop<()>, WinitInputHelper, Pixels) {
    let event_loop = EventLoopBuilder::<()>::with_user_event().build();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("NES")
            .with_inner_size(NES_SIZE)
            .with_min_inner_size(NES_SIZE)
            .build(&event_loop)
            .expect("WindowBuilder error"),
    );
    let input = WinitInputHelper::new();

    arch::prepare_window(&window);

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
        Pixels::new_async(NES_WIDTH as u32, NES_HEIGHT as u32, surface_texture)
            .await
            .expect("Pixels error")
    };

    (window, event_loop, input, pixels)
}
