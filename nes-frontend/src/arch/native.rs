#![cfg(not(target_arch = "wasm32"))]

pub fn prepare_env() {
    env_logger::init();
}

pub fn start_run<F: std::future::Future>(fut: F) -> F::Output {
    pollster::block_on(fut)
}

pub fn prepare_window(_window: &std::sync::Arc<winit::window::Window>) {}
