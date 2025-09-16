# Project 090 – Conway’s Game of Life (Terminal Grid Simulation)


## What I Built
A terminal-based simulation of Conway’s Game of Life, a cellular automaton that evolves based on simple rules. Learned 2D grid modeling, terminal rendering, and game loop logic in Rust.

## What I Learned


## Notes
### Run the App
```
cargo run
```

### Game Rules Recap:
- Live cell with 2 or 3 neighbors → lives.

- Dead cell with 3 neighbors → comes alive.

- All other cells → die or remain dead.

### Key Concepts:
- 2D arrays for grid logic

- Terminal rendering with ANSI escape codes

- `wrapping_add` to avoid panics on underflow

- Game loop + timing with thread::sleep

We now have a dynamic simulation running in terminal — useful for learning core game dev principles or experimenting with cellular automata.



