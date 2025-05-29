# Project 001 – Hello Rust CLI

## What I Built
A simple Rust CLI that prints "Hello, Rust!" to the console using `println!`.

## What I Learned
- How to set up Rust on your PC: You can install Rust using [rustup](https://www.rust-lang.org/tools/install), which is the recommended installer. It sets up everything you need including the compiler (rustc), the package manager (cargo), and more.
Alternatively, if you don’t want to install anything, you can try out Rust directly in your browser using [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024).

- How to initialize a Rust project using cargo new
cargo is Rust’s official package manager and build tool. When you run:
```
cargo new hello_rust
```
It creates a new folder with all the necessary files to start coding — including a basic Rust file (main.rs) and configuration file (Cargo.toml). 

- Compiling and running Rust projects using cargo run
To compile and run your code, you just need:
```
cargo run
```
This compiles the code and runs it in one go 

- Simple Rust commands

- The basic project structure in Rust


## Notes
Rust is a system programming language focused on safety, performance, and concurrency.
Rust is designed to give you the speed of C/C++ with the safety of modern languages like Python or JavaScript.
It eliminates entire categories of bugs, such as null pointer derefrencing and data races, by enforcing strict rules at compile-time

Why use Rust?
- Memory safety without garbage collection
Rust uses ownership and borrowing rules to manage memory without needing a garbage collector. This makes it fast and reliable.

- Concurrency without data races
Thanks to its ownership model, Rust prevents common bugs in concurrent programming like data races at compile time.

- High performance
Rust compiles to machine code, making it ideal for building blazing-fast apps and command-line tools.

- Great ecosystem
With tools like cargo, crates.io (Rust’s package registry), and a supportive community, Rust is very beginner-friendly for a systems language.












