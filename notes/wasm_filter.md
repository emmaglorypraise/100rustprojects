# Project 098 – WebAssembly Image Filter (Rust → WASM)

## What I Built
A a browser-ready image filter app by compiling Rust to WebAssembly (WASM). Applying a grayscale filter to an image using the image crate in Rust, then expose the function to JavaScript via wasm-bindgen.


## What I Learned
- `wasm-bindgen` bridges Rust and JS

- `image` crate applies transformations

- Image I/O via `Uint8Array <-> &[u8]`

- JS handles DOM/UI, Rust handles compute

## Notes
### Prerequisites
Install wasm-pack:
```
cargo install wasm-pack
```
### Create WASM Project
```
wasm-pack new wasm_filter 
cd wasm_filter
```
### Build the WASM module
```
wasm-pack build --target web
```
This outputs pkg/ with wasm and JS bindings.

### Serve the Site
Use a local static server like basic-http-server:
```
cargo install basic-http-server
basic-http-server .
```
Open http://localhost:4000 and test your image filter 🎨
