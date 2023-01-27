use super::TargetArch;

pub struct NativeArch;

impl TargetArch for NativeArch {
    fn prepare_env() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            env_logger::init();
        }
    }

    #[allow(unused_variables, unreachable_code)]
    fn start_run<F: std::future::Future>(fut: F) -> Option<F::Output> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            return Some(pollster::block_on(fut));
        }

        None
    }

    #[allow(unused_variables, unreachable_code)]
    fn prepare_window(window: &std::sync::Arc<winit::window::Window>) {}
}
