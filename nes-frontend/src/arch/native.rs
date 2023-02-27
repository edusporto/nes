#![cfg(not(target_arch = "wasm32"))]

use instant::Duration;

pub fn prepare_env() {
    env_logger::init();
}

pub fn start_run<F: std::future::Future>(fut: F) -> F::Output {
    pollster::block_on(fut)
}

pub fn prepare_window(_window: &std::sync::Arc<winit::window::Window>) {}

pub fn sleep(duration: Duration) {
    spin_sleep::sleep(duration);
}

pub fn spawn<F: std::future::Future + 'static>(fut: F) {
    pollster::block_on(fut);
}
