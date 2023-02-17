# nes

Multi-platform NES emulator built with Rust.

# Running

This project is made with Rust. To install the toolchain necessary to build this
project, check out https://rustup.rs/.

Once the toolchain is installed, you can compile the project to run natively in
your operating system or run in the browser with WebAssembly.

## Native

Run the following command:

```
cargo run --release
```

## Browser (WebAssembly)

To build the project targeting the web, you first need to install the `wasm32`
toolchain:

```
rustup target install wasm32-unknown-unknown
```

After installing the toolchain, you can build the project and run it locally
with the following command:

```
cargo run-wasm --release --bin nes
```

This will start a local web server at `http://localhost:8000/`. The resulting
build files will be located at `target/wasm-examples/nes`.

# About

This implementation of an NES emulator is heavily inspired and guided by
OneLoneCoder's implementation, which can be found at
https://github.com/OneLoneCoder/olcNES.
