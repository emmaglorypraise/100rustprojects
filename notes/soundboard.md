# Project 097 – CLI Soundboard App (with rodio Audio Playback)

## What I Built
A fun CLI tool that plays sound clips using number-based selection. Using the rodio crate for cross-platform audio playback, and practice file I/O, audio streaming, and clean CLI menus.


## What I Learned
- r`odio::Decoder` to stream audio from file

- `Sink` to control playback

- `Buffered I/O` with `BufReader` for file decoding

- CLI selection logic with HashMap

## Notes
### Expected Folder Structure
```
sounds/
├── clap.mp3
├── ding.mp3
├── sneeze.mp3

You can use .wav, .ogg, or .mp3 formats.

```
### Run the App
```
cargo run
And select a sound to hear it play 🎧
```
