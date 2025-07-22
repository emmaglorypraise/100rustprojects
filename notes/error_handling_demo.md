# Project 018 – Error Handling Demo

## What I Built
A Rust CLI tool that explores Rust’s error handling mechanisms using Result, Option, and match. I created a small calculator that demonstrates graceful handling of invalid inputs using unwrap, expect, ?, and custom error messages.

## What I Learned

1. Result Type:

- Functions return either Ok(value) for success or Err(error) for failure

- The ? operator automatically handles errors

2. Pattern Matching (match):

- Checks which menu option you chose

- Handles both success and error cases cleanly

3. Type Conversion:

- .parse::<f64>() tries to convert text to a decimal number

#### ParseFloatError

`use std::num::ParseFloatError;` in Rust brings the ParseFloatError type into scope so you can use it in your code. 

It's a special error type that Rust provides when string-to-number parsing fails. When you try to convert a string (like "3.14") to a floating-point number (like f64) using .parse(), there's a chance it might fail if the string isn't a valid number.

##### Why Do We Need It?
1. Error Handling: When you write "hello".parse::<f64>(), this will fail because "hello" isn't a number. Rust needs a way to tell you about this failure.

2. Type Safety: Instead of crashing, Rust returns a Result type that contains either:

- Ok(value) if successful

- Err(ParseFloatError) if failed

Example
```
fn parse_two_numbers() -> Result<(f64, f64), ParseFloatError> {
    let a = input("Enter first number: ").parse::<f64>()?;
    let b = input("Enter second number: ").parse::<f64>()?;
    Ok((a, b))
}
```

## Notes

In Rust, println! and eprintln! are both macros used to print text to the console, but they write to different output streams. 

The main difference lies in which stream they use. println! writes to the standard output stream (stdout), which is normally used for a program's regular, expected output. When you want to display results, information, or normal program output that users or other programs might want to process, println! is the appropriate choice. For example, in a calculator program, you'd use println! to display calculation results because that's the primary purpose of the program.

On the other hand, eprintln! writes to the standard error stream (stderr), which is designed for error messages, warnings, and diagnostic information. This separation is particularly useful because it allows users and other programs to distinguish between normal output and error messages. In the calculator example, when displaying error messages like invalid input or division by zero, eprintln! would be used because these aren't part of the normal program output but rather exceptional conditions.

This separation becomes especially important when redirecting output. In Unix-like systems, you can redirect stdout to a file while stderr continues to display in the terminal. For instance, if you run ./program > output.txt, all println! output goes to the file, while eprintln! messages still appear in the terminal. This helps ensure that error messages remain visible even when normal output is being captured or processed by another program. It's a good practice that makes your programs more flexible and user-friendly in real-world usage scenarios.
    










