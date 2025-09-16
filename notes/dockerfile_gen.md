# Project 099 – WebAssembly Image Filter (Rust → WASM)

## What I Built
A CLI tool that detects Rust project layout (Cargo.toml, src/main.rs) and generates a multi-stage Dockerfile to build and ship the app. It’s perfect for CI/CD and containerized deployment. A Rust-native Dockerfile generator, ideal for automating deployment and shipping microservices!


## What I Learned

## Notes
The tool has a professional command-line interface with comprehensive options, making it suitable for use in various development and CI/CD environments.


### Run the App
# Basic usage 
cargo run

# Generate a Dockerfile with specific options
cargo run -- --rust-version nightly --base-image alpine --output Dockerfile.alpine

# Generate a Dockerfile without cache optimization
cargo run -- --no-cache

# Generate a Dockerfile quietly with custom target directory
cargo run -- --target-dir /tmp/target --quiet

# Show help
cargo run -- --help
```
