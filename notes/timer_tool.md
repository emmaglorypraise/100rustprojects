# Project 011 – Basic Timer Tool

## What I Built
A Rust CLI timer tool that allows users to set a time for a specified duration and notifies them when time is up.

## What I Learned
```
use std::{io, thread, time::Duration};
use std::io::Write;
use std::sync::mpsc;
use std::thread::sleep;
```

```
std::io
```

We need io to read user input (io::stdin().read_line(...)) and to flush the prompt to the console (io::stdout().flush().unwrap()).

```
std::thread
```

We spawn a background thread to listen for pause/resume commands. Anything involving thread::spawn(...) or channel communication needs this import.

```
std::time::Duration
```

We use Duration::from_secs(1) (and later from_millis(200)) to pause the main loop for a given interval. This makes the countdown tick once per second (or check for commands every 200 ms when paused).

```
std::io::Write
```

The Write trait provides flush(). We call io::stdout().flush().unwrap() to force the “Time Remaining…” prompt to appear immediately without waiting for a newline.

```
std::sync::mpsc
```

mpsc (multi-producer, single-consumer) channels let us send a small message ('p' or 'r') from the “listener” thread into the main timer loop. We call mpsc::channel() to create (tx, rx), where tx is the transmitter held by the spawned thread, and rx is the receiver used by the main loop.

```
std::thread::sleep
```

For convenience, we imported sleep directly so we can write sleep(Duration::from_secs(1)) instead of thread::sleep(...). This pauses whichever thread is running for that duration.
## Notes
Printing with a carriage returns `\r` so it overwrites the same line each second. `\n` writes on  a new line instead.

