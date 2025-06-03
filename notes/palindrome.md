# Project 007 – Palindrome Checker

## What I Built
A Rust CLI tool that checks if a given string is a palindrome 

## What I Learned
Palindrome is a word, phrase, or sequence that reads the same backwards as forward ignoring spaces, punctuations and capitalization. Palindromes can be words, phrases, numbers, or names. Examples are madam, racecar, etc

## Notes
Difference between &str and String
In Rust, a `String` is an owned, mutable, heap-allocated string type that you can modify and grow as needed, commonly created with 
```
String::from("Hello, World!")
```
or 
```
"Hello".to_string()

```
While a `&str` is an immutable reference to a sequence of UTF-8 bytes, typically used for string literals like 
```
let greeting: &str = "Hello";
```
or as a borrowed view into a String. 
For example, you might write 
```
let owned: String = String::from("Hello, Rust!");
``` 
for an owned string, and 
```
let slice: &str = "Hello, world!"; 
```
for a string slice; you can also borrow a slice from a String with 
```
let slice: &str = &owned;
```










