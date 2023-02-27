#![cfg(target_arch = "wasm32")]

use std::future::Future;
use std::sync::Arc;

use instant::Duration;
use winit::dpi::LogicalSize;
use winit::window::Window;

pub fn prepare_env() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).expect("error initializing logger");
}

pub fn start_run<F: Future<Output = ()> + 'static>(fut: F) -> F::Output {
    wasm_bindgen_futures::spawn_local(fut)
}

pub fn prepare_window(window: &Arc<Window>) {
    use wasm_bindgen::JsCast;
    use winit::platform::web::WindowExtWebSys;

    // Retrieve current width and height dimensions of browser client window
    let get_window_size = || {
        let client_window = web_sys::window().unwrap();
        LogicalSize::new(
            client_window.inner_width().unwrap().as_f64().unwrap(),
            client_window.inner_height().unwrap().as_f64().unwrap(),
        )
    };

    let window = Arc::clone(&window);

    let canvas = window.canvas();
    // Prevent bottom padding (without this, the window is bigger than the canvas)
    canvas.style().set_property("vertical-align", "bottom").ok();

    // Initialize winit window with current dimensions of browser client
    window.set_inner_size(get_window_size());

    let client_window = web_sys::window().unwrap();

    // Attach winit canvas to body element
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body| body.append_child(&web_sys::Element::from(canvas)).ok())
        .expect("couldn't append canvas to document body");

    // Listen for resize event on browser client. Adjust winit window dimensions
    // on event trigger
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
        let size = get_window_size();
        window.set_inner_size(size)
    }) as Box<dyn FnMut(_)>);
    client_window
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}

// I would love for this to work, but it's still not quite there.
//
// I have tested this on Windows and on Linux, and have gotten two different but
// equally bad results.
//
// The first result was that it did work, but while the thread was sleeping, no
// events would register. This meant that some button presses would be ignored
// by the program.
// The second result was that nothing happened. My theory is that, in this case,
// the spawned future would run in the background. If I can get this behaviour to
// be reproduced reliably, it could be used to start a separate process that would
// deal uniquely with the game loop.
#[allow(dead_code, unused_variables)]
pub fn sleep(duration: Duration) {
    // wasm_bindgen_futures::spawn_local(async move {
    //     wasm_timer::Delay::new(duration)
    //         .await
    //         .expect("couldn't sleep");
    // });
}

pub fn spawn<F: std::future::Future<Output = ()> + 'static>(fut: F) {
    wasm_bindgen_futures::spawn_local(fut)
}
