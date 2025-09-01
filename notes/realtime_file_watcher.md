# Project 081 – Real-Time File Watcher

## What I Built

A tool that watches a file or directory and prints a message every time it’s modified. Using the notify crate to set up real-time filesystem monitoring, which is great for auto-reloaders, compilers, or backup triggers.

## What I Learned
Added these features:
- Trigger a shell command or reload task
- Send desktop notifications (notify-rust)

## Notes

### How to Run the Application:

1. Create a file in root directory named `some_file.txt`

2. Run the CLI:

```
cargo run -- ./some_file.txt

```

3. Edit the file in another terminal or text editor, and watch the updates roll in:

```
👁️ Watching for changes in: ./some_file.txt
📝 File modified: ["some_file.txt"]
```
