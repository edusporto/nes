use std::sync::Arc;

use instant::Instant;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoopBuilder;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use nes_core::screen::{NES_HEIGHT, NES_WIDTH};
use nes_frontend::arch;
use nes_frontend::game::Game;

const NES_SIZE: LogicalSize<u32> = LogicalSize::new(NES_WIDTH as u32, NES_HEIGHT as u32);

const FPS: u32 = 60;
// const FRAME_TIME: Duration = Duration::from_micros(1_000_000 / FPS);

fn main() {
    arch::prepare_env();
    arch::start_run(run());
}

async fn run() {
    // TODO: do this better somehow

    // let file_name = std::env::args()
    //     .nth(1)
    //     .expect("Missing the file name to the desired ROM as argument.");
    // let file_name = "games/Super Mario Bros.nes";
    let cart = include_bytes!("../../../games/Super Mario Bros.nes");

    let event_loop = EventLoopBuilder::<()>::with_user_event().build();
    let window = WindowBuilder::new()
        .with_title("NES")
        .with_inner_size(NES_SIZE)
        .with_min_inner_size(NES_SIZE)
        .build(&event_loop)
        .expect("WindowBuilder error");

    // let window = Arc::new(window);

    // arch::prepare_window(&window);

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        let get_window_size = || {
            let client_window = web_sys::window().unwrap();
            LogicalSize::new(
                client_window.inner_width().unwrap().as_f64().unwrap(),
                client_window.inner_height().unwrap().as_f64().unwrap(),
            )
        };

        window.set_inner_size(get_window_size());

        let client_window = web_sys::window().unwrap();

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
    }

    let input = WinitInputHelper::new();
    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new_async(NES_WIDTH as u32, NES_HEIGHT as u32, surface_texture)
            .await
            .expect("Pixels error")
    };

    let game = Game::start_from_bytes(cart, input, pixels).expect("Couldn't load game");

    let mut fps_avg = MovingAvg::new(5);
    let mut fps_time = Instant::now();

    game_loop::game_loop(
        event_loop,
        window,
        game,
        FPS,
        0.1,
        |g| {
            g.game.update();
            g.game.draw();
        },
        move |g| {
            if let Err(err) = g.game.pixels.render() {
                error!("pixels.render() failed: {err}");
                g.exit();
            }

            fps_avg.add(1.0 / fps_time.elapsed().as_secs_f64());
            g.window
                .set_title(&format!("NES (FPS: {:.1})", fps_avg.avg()));
            fps_time = Instant::now();
        },
        move |g, event| {
            if g.game.input.update(&event) {
                // Close events
                if g.game.input.key_pressed(VirtualKeyCode::Escape) || g.game.input.quit() {
                    g.exit();
                }

                // Resize the window
                if let Some(size) = g.game.input.window_resized() {
                    g.game.pixels.resize_surface(size.width, size.height).ok();
                }
            }
        },
    );

    // event_loop.run(move |event, _event_loop, control_flow| {
    //     // control_flow.set_wait();
    //     // control_flow.set_poll();

    //     match event {
    //         Event::RedrawRequested(_) => {
    //             if let Err(err) = pixels.render() {
    //                 error!("pixels.render() failed: {err}");
    //                 *control_flow = ControlFlow::Exit;
    //                 return;
    //             }

    //             fps_avg.add(1.0 / fps_time.elapsed().as_secs_f64());
    //             window.set_title(&format!("NES (FPS: {:.1})", fps_avg.avg()));
    //             fps_time = Instant::now();
    //         }
    //         _ => {}
    //     }

    //     if input.update(&event) {
    //         // Close events
    //         if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
    //             *control_flow = ControlFlow::Exit;
    //             return;
    //         }

    //         // Resize the window
    //         if let Some(size) = input.window_resized() {
    //             if let Err(err) = pixels.resize_surface(size.width, size.height) {
    //                 error!("pixels.resize_surface() failed: {err}");
    //                 *control_flow = ControlFlow::Exit;
    //                 return;
    //             }
    //         }

    //         if time.elapsed() > FRAME_TIME {
    //             time = Instant::now();
    //             game.update_controllers();
    //             game.update();
    //             game.draw();
    //             window.request_redraw();
    //         }
    //     }
    // });
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
