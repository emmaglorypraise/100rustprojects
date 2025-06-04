# Project 008 – Fibonacci Sequence Generator 

## What I Built
A Rust CLI tool to generate the Fibonacci sequence up to a user-defined number of terms.

## What I Learned
 The Fibonacci sequence is a series of numbers where each successive number is equal to the sum of the two numbers that precede it.
 The mathematical representation is:
 F(n) = F(n-1) + F(n-2)
 E.g. 0,1,1,2,3,5,8,13,21,34

## Notes
This program begins by prompting the user to enter how many terms of the Fibonacci sequence they wish to generate. It validates that the input is a positive integer, then computes exactly that many Fibonacci numbers, storing them in a vector. Once the full sequence is built, the code separates each Fibonacci term into two groups, evens and odds, by checking whether each number is divisible by two. Finally, it prints out the original Fibonacci list along with the two filtered lists (even and odd), giving the user a clear view of how the sequence breaks down by parity.

Under the hood, the program uses a helper function to read and parse a single line of input as a u32. If parsing fails or the number is zero, it exits with an error message. The generate_fibonacci function then allocates a vector of the requested size and fills it by pushing 0 and 1 for the first two terms, followed by iteratively summing the two previous values. After generating the sequence, split_even_odd iterates over that vector, pushing each value into either an “evens” or “odds” vector based on its remainder when divided by two. This clear separation of concerns—input handling, sequence generation, and parity filtering, keeps the code organized and easy to follow.