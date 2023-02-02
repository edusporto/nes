#![cfg(not(target_arch = "wasm32"))]

pub fn prepare_env() {
    env_logger::init();
}

pub fn start_run<F: std::future::Future>(fut: F) -> Option<F::Output> {
    return Some(pollster::block_on(fut));
}

pub fn prepare_window(_window: &std::sync::Arc<winit::window::Window>) {}

pub async fn sleep(duration: instant::Duration) {
    // spin_sleep::sleep(duration);
    async_std::task::sleep(duration).await
}
