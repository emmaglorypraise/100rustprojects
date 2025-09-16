# Project 094 – Dependency Graph Visualizer (with petgraph)


## What I Built
A Rust CLI tool that models a graph of module or crate dependencies, then outputs a .dot file for visualization in tools like Graphviz. You’ll use the petgraph crate to build and walk graphs.


## What I Learned
- DiGraph creates a directed graph

- Dot formatter creates .dot syntax

- Nodes = modules, Edges = dependencies

## Notes
### Example Use Case
You’ll simulate dependencies between modules:
```
main.rs depends on utils.rs and config.rs  
utils.rs depends on log.rs
```

### Run the App
```
cargo run
```
Then render the graph with:
```
dot -Tpng dep_graph.dot -o graph.png

open graph.png
```
