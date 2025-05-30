# Project 003 – Simple Calculator

## What I Built
A Rust CLI calculator that supports basic arithmetic operations like addition, substraction, multiplication and deletion.

## What I Learned
```
io::stdout().flush().unwrap();
```
That call grabs the program’s standard‐output stream, requests that any text still sitting in its internal buffer be written out to the terminal immediately (rather than waiting for a newline or the buffer to fill), and then unwrap() will panic if for some reason the flush fails. In practice you use io::stdout().flush().unwrap() right after a print! that doesn’t include a newline so that your prompt appears on the screen before your code blocks waiting for input.

to use:
```
use std::io::{self, Write};

```
## Notes


    










