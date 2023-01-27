pub mod native;
pub mod wasm;

use std::future::Future;
use std::sync::Arc;

use winit::window::Window;

/// Represents a target architecture for the binary,
/// specifying special behaviour for each arch.
///
/// The trait's functions expect the implementer to call
/// the `#[cfg(target_arch = "_")]` directive.
pub trait TargetArch {
    /// Prepares the environment for things like logging.
    fn prepare_env();
    /// Calls the `run` function, which deals with window management.
    /// Should return `None` if called on the wrong architecture.
    fn start_run<F: Future<Output = ()> + 'static>(fut: F) -> Option<F::Output>;
    /// Prepares structures related to windowing.
    fn prepare_window(window: &Arc<Window>);
}
