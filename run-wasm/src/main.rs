/// Allows the workspace to be compiled to WebAssembly (Wasm).
///
/// To compile and run the frontend into Wasm, run:
/// ```bash
/// cargo run-wasm --release --package nes-frontend
/// ```
///
/// To only build the Wasm frontend, run:
/// ```bash
/// cargo run-wasm --release --build-only --package nes-frontend
/// ```
/// The build files will be stored on `./target/wasm-examples/minimal-web/`.
fn main() {
    cargo_run_wasm::run_wasm_with_css("body { margin: 0px; }");
}
