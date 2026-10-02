#![windows_subsystem = "windows"]

use std::sync::Arc;

// use web_time::{Duration, Instant};
use log::error;
use nes_frontend::gui::GuiEvent;
use pixels::wgpu::TextureFormat;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use tokio::sync::mpsc::Receiver;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use nes_core::screen::{NES_HEIGHT, NES_WIDTH};
use nes_frontend::arch;
// use nes_frontend::fps::FpsCounter;
use nes_frontend::framework::Framework;
use nes_frontend::game::GameState;

const NES_SIZE: LogicalSize<u32> = LogicalSize::new(NES_WIDTH as u32, NES_HEIGHT as u32);
const SCALED_SIZE: LogicalSize<u32> = LogicalSize::new(NES_SIZE.width * 3, NES_SIZE.height * 3);
const FPS: u32 = 60;
/// Zoom applied to the GUI on top of the window's native scale factor.
const GUI_ZOOM: f32 = 1.2;
/// Non-sRGB surface format for the window. There's no single format available
/// everywhere: Metal has no `Rgba8*` surfaces, while WebGL has no `Bgra*` ones.
/// `Bgra8Unorm` is offered by Metal, DX12, Vulkan and GL on desktop, and
/// `Rgba8Unorm` by both WebGPU and WebGL.
#[cfg(not(target_arch = "wasm32"))]
const SURFACE_FORMAT: TextureFormat = TextureFormat::Bgra8Unorm;
#[cfg(target_arch = "wasm32")]
const SURFACE_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;
// const FRAME_TIME: Duration = Duration::from_micros(1_000_000 / FPS as u64);

fn main() {
    arch::prepare_env();
    arch::start_run(run());
}

async fn run() {
    let (window, event_loop, input, pixels, framework, receiver) = build_window().await;

    let game = GameState::new(input, pixels, framework, receiver);
    // let mut fps = FpsCounter::new(10);
    // let mut time = Instant::now();

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
            // if time.elapsed() < FRAME_TIME {
                // TODO: Test if still necessary on current `game_loop` version
                // arch::sleep(FRAME_TIME.saturating_sub(time.elapsed()));
            // }

            // time = Instant::now();

            g.game.framework.prepare(g.window.as_ref());
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

            // fps.update();
            // g.window.set_title(&format!("NES (FPS: {:.1})", fps.avg()));
        },
        move |g, event| {
            // Window handler
            match event {
                Event::NewEvents(_) => g.game.input.step(),
                Event::WindowEvent { event, .. } => {
                    g.game.input.process_window_event(event);
                    // GUI event
                    g.game.framework.handle_event(&g.window, event);
                }
                Event::DeviceEvent { event, .. } => g.game.input.process_device_event(event),
                Event::AboutToWait => {
                    g.game.input.end_step();

                    // Close events
                    if g.game.input.close_requested() || g.game.input.destroyed() {
                        g.exit();
                    }

                    g.game.treat_input();
                }
                _ => {}
            }
        },
    )
    .unwrap();
}

async fn build_window() -> (
    Arc<Window>,
    EventLoop<()>,
    WinitInputHelper,
    Pixels<'static>,
    Framework,
    Receiver<GuiEvent>,
) {
    let event_loop = EventLoop::new().expect("EventLoop error");
    let window_attributes = Window::default_attributes()
        .with_title("NES")
        .with_inner_size(SCALED_SIZE)
        .with_min_inner_size(NES_SIZE);
    // `game_loop` still drives winit through the closure-based `EventLoop::run`
    // API, so the window has to be created before the event loop starts.
    #[allow(deprecated)]
    let window = Arc::new(
        event_loop
            .create_window(window_attributes)
            .expect("Window creation error"),
    );
    let input = WinitInputHelper::new();

    let window_size = arch::prepare_window(&window);

    let pixels = {
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.clone());
        // egui expects a non-sRGB framebuffer (it blends in gamma space), and
        // WebGPU canvases don't offer sRGB formats at all. The backing texture
        // must then also be non-sRGB, so the (already sRGB-encoded) NES colors
        // pass through unchanged instead of being decoded to linear and darkened.
        PixelsBuilder::new(NES_WIDTH as u32, NES_HEIGHT as u32, surface_texture)
            .surface_texture_format(SURFACE_FORMAT)
            .texture_format(TextureFormat::Rgba8Unorm)
            .build_async()
            .await
            .expect("Pixels error")
    };

    let (sender, receiver) = tokio::sync::mpsc::channel(50);

    let framework = Framework::new(
        &window,
        window_size.width,
        window_size.height,
        GUI_ZOOM,
        &pixels,
        sender,
    );

    (window, event_loop, input, pixels, framework, receiver)
}
