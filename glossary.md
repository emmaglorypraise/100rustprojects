# Glossary

## Definations

- `Rust`:
- `Cargo`:
- `Prelude`: By default, Rust has set of items defined in the standard library that it brings into the scope of every program, e.g, the io library. If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a `use` statement.
- `String`: is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
- `Reference(&)`:   gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
- `Crate`: is a collection of Rust source code files. A binary crate is executable.
- `Crates.io`: is where people in the Rust ecosystem post their open source Rust projects for others to use.
- `Shadowing`: ets us reuse a variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example. This feature is often used when you want to convert a value from one type to another type.
- `Panicking`: is a term used in Rust when when a program exits with an error.
- `Destructuring`:  is breaking a compound type into different parts, e.g, breaking a tuple(tup) into three parts(x, y, z).

## Rust commands

- `clear`: to clear terminal
- `pwd`: check folder directory
- `open .`: open folder in directory from the terminal
- `rustc --version`: to check rust compiler version installed in pc
- `cargo --version`: to check the version of the cargo build tool.
- `rustup update`: update rust version from terminal
- `cargo update`: is used to update a crate, the command update will ignore the `Cargo.lock` file and figure out all the latest versions that fit your specifications in `Cargo.toml`.
- `cargo new <folder_name>`: create new rust project using cargo
- `cargo run`: run a cargo project
- `cargo build`: build a cargo project, optimized for fast compilation 
- `cargo check`: check the project for errors without building
- `cargo test`: run test cases if any
- `cargo doc`: generates documentation for project
- `cargo doc --open`: will build documentation provided by all your dependencies locally and open it in your browser
- `cargo build --release`: builds a release version of project, optimized for performance(production release)


## VS Code Extensions

- [Rust Syntax](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax) by Dusty Pomerleau
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) by rust-lang.org
- [Rust Flash Snippets](https://marketplace.visualstudio.com/items?itemName=lorenzopirro.rust-flash-snippets) by lorenzo pirro



## 🔗 Resources
- [Rust Book](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/meta/playground.html)
- https://quickref.me/rust.html
- https://cheats.rs/
