# Project 095 – JSON Merge Tool (Conflict-Aware Merger)


## What I Built
A Rust CLI tool that merges two JSON files, handling nested structures and flagging conflicting keys. This is great for configuration managers, data overlays, or patching systems.


## What I Learned
- Conflict resolution: patch overwrites base

## Notes
### Example JSON Inputs
```
base.json

{
  "name": "App",
  "version": "1.0",
  "features": {
    "logging": true,
    "cache": true
  }
}

patch.json

{
  "version": "2.0",
  "features": {
    "cache": false,
    "metrics": true
  }
}
```

### Sample Output: merged.json
```
{
  "name": "App",
  "version": "2.0",
  "features": {
    "logging": true,
    "cache": false,
    "metrics": true
  }
}
```
### Run the Merger
```
cargo run -- base.json patch.json merged.json
```
