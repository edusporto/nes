use std::future::Future;
use std::rc::Rc;

#[allow(unused_imports)]
use winit::dpi::LogicalSize;
use winit::window::Window;

use super::TargetArch;

pub struct WasmArch;

impl TargetArch for WasmArch {
    fn prepare_env() {
        #[cfg(target_arch = "wasm32")]
        {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Trace).expect("error initializing logger");
        }
    }

    #[allow(unused_variables, unreachable_code)]
    fn start_run<F: Future<Output = ()> + 'static>(fut: F) -> Option<F::Output> {
        #[cfg(target_arch = "wasm32")]
        {
            return Some(wasm_bindgen_futures::spawn_local(fut));
        }

        None
    }

    #[allow(unused_variables, unreachable_code)]
    fn prepare_window(window: &Rc<Window>) {
        #[cfg(target_arch = "wasm32")]
        {
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

            let window = Rc::clone(&window);

            // Initialize winit window with current dimensions of browser client
            window.set_inner_size(get_window_size());

            let client_window = web_sys::window().unwrap();

            // Attach winit canvas to body element
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");

            // Listen for resize event on browser client. Adjust winit window dimensions
            // on event trigger
            let closure =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
                    let size = get_window_size();
                    window.set_inner_size(size)
                }) as Box<dyn FnMut(_)>);
            client_window
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    }
}
