#![windows_subsystem = "windows"]

use std::sync::Arc;

use instant::{Duration, Instant};
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

use nes_core::screen::{NES_HEIGHT, NES_WIDTH};
use nes_frontend::arch;
use nes_frontend::fps::FpsCounter;
use nes_frontend::framework::Framework;
use nes_frontend::game::GameState;
use nes_frontend::gui::GuiEvent;

const NES_SIZE: LogicalSize<u32> = LogicalSize::new(NES_WIDTH as u32, NES_HEIGHT as u32);
const SCALED_SIZE: LogicalSize<u32> = LogicalSize::new(NES_SIZE.width * 3, NES_SIZE.height * 3);
const FPS: u32 = 60;
const FRAME_TIME: Duration = Duration::from_micros(1_000_000 / FPS as u64);

fn main() {
    arch::prepare_env();
    arch::start_run(run());
}

async fn run() {
    let (window, event_loop, input, pixels, framework) = build_window().await;

    let game = GameState::new(input, pixels, framework);
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
            for event in g.game.framework.gui.take_game_events() {
                match event {
                    GuiEvent::ChangeRom(cart) => {
                        g.game.start_from_cartridge(cart);
                    }
                    GuiEvent::ToggleSettings => g.game.framework.gui.toggle_settings(),
                }
            }

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

            g.game.framework.prepare(&g.window);
            let render_result = g
                .game
                .pixels
                .render_with(|encoder, render_target, context| {
                    context.scaling_renderer.render(encoder, render_target);
                    g.game.framework.render(encoder, render_target, context);
                    Ok(())
                });

            if let Err(err) = render_result {
                error!("Could not render the screen. Error: {err}");
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

                // Update the scale factor
                if let Some(scale_factor) = g.game.input.scale_factor() {
                    g.game.framework.scale_factor(scale_factor as f32);
                }

                // Resize the window
                if let Some(size) = g.game.input.window_resized() {
                    g.game.pixels.resize_surface(size.width, size.height).ok();
                    g.game.framework.resize(size.width, size.height);
                }

                // Show settings menu
                if g.game.input.key_pressed(VirtualKeyCode::Escape) {
                    g.game.framework.gui.toggle_settings();
                }

                if g.game.input.key_pressed(VirtualKeyCode::F5) {
                    g.game.restart();
                }
            }

            if let winit::event::Event::WindowEvent { event, .. } = event {
                g.game.framework.handle_event(event);
            }
        },
    );
}

async fn build_window() -> (
    Arc<Window>,
    EventLoop<()>,
    WinitInputHelper,
    Pixels,
    Framework,
) {
    let event_loop = EventLoopBuilder::<()>::with_user_event().build();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("NES")
            .with_inner_size(SCALED_SIZE)
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

    let framework = Framework::new(
        &event_loop,
        window.inner_size().width,
        window.inner_size().height,
        window.scale_factor() as f32 * 1.2,
        &pixels,
    );

    (window, event_loop, input, pixels, framework)
}
