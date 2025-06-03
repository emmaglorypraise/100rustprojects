# Project 005 – Word Counter

## What I Built
A Rust CLI tool that counts the number of words, lines and characters in a text file 

## What I Learned
```
use std::env;
```
This imports the env module, which lets you interact with the environment your program is running in.

This allows you access command-line arguments and environment variables.



```
use std::fs::File;
```
This imports the File struct from the file system module (std::fs).
It allows your program access to open, read, or write to files.

Example
```
let file = File::open("my_file.txt")?;
```

`File::open()` opens an existing file for reading.

You can also do `File::create()` to make a new file for writing.

```
use std::io::{self, Read};
```
This handles input/output and read file content as text or bytes



## Notes


    










