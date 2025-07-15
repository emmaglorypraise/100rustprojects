# Project 014 – File Reader CLI

## What I Built
A Rust CLI tool in Rust that reads a file and prints its contents. Add optional features like showing line numbers and searching for keywords within the file.

## What I Learned



## Notes
### How to Run:

Read full file:
```
cargo run sample.txt
```

Read with line numbers:

```
cargo run sample.txt --lines
```

Search for keyword:

```
cargo run sample.txt --search Rust
```

Search with line numbers:

```
cargo run sample.txt --lines --search Rust
```


Here's a simple and useful sample.txt file content you can use to test all features of this CLI tool

```
Rust is a systems programming language.
It is fast, memory-efficient, and safe.
You can build CLI tools with Rust.
This line has nothing to do with programming.
Learning Rust can be fun and challenging.
Let's talk about performance.
Rustaceans love fearless concurrency.
Search and filtering are important CLI features.
```
