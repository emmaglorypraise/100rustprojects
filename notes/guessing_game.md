# Project 004 – Guessing Game CLI

## What I Built
An interactive Number Guessing Game in Rust CLI that uses random number generation, user input, and control flow to guide the player.

## What I Learned
- How to import dependencies: After adding dependencies to the cargo.toml file, run:
```
cargo build
```
This will download the dependecies into your project
- Difference between an infinite loop and for loop. Instead of an infinite loop, I used `for attempt in 1..=max_attempts` to count attempts. This automatically stops after the allowed number.

## Notes
I added 3 new features to the guessing game
- Replay option
- Difficulty levels(1-20, 1-50, 1-100)
- Max number of tries for each levels











