# Project 058 – Key-Value Store

## What I Built

A simple persistent key-value store that supports set, get, and delete commands via CLI. The data is stored in a JSON file, making it a lightweight alternative to Redis or LevelDB for local storage.

## What I Learned

## Notes
##### Sample Interaction:
```
> set language Rust
✅ Key set.
> get language
🔑 Value: Rust
> list
📦 Stored Keys:
- language
> delete language
🗑️ Key deleted (if it existed).
> exit

```