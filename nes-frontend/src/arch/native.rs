#![cfg(not(target_arch = "wasm32"))]

use web_time::Duration;

pub fn prepare_env() {
    env_logger::init();
}

pub fn start_run<F: std::future::Future>(fut: F) -> F::Output {
    pollster::block_on(fut)
}

/// Returns the initial physical size of the window's surface.
pub fn prepare_window(
    window: &std::sync::Arc<winit::window::Window>,
) -> winit::dpi::PhysicalSize<u32> {
    window.inner_size()
}

pub fn sleep(duration: Duration) {
    spin_sleep::sleep(duration);
}

pub fn spawn<F: std::future::Future + 'static>(fut: F) {
    pollster::block_on(fut);
}
