#[cfg(not(target_arch = "wasm32"))]
#[path = "native.rs"]
mod specific;

#[cfg(target_arch = "wasm32")]
#[path = "wasm.rs"]
mod specific;

pub use self::specific::*;
