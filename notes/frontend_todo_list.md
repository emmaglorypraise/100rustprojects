# Project 072 – Frontend Todo List (Yew + REST API)

## What I Built
A Yew-based frontend that connects to a Actix-Web backend to fetch and display todos via HTTP. 

## What I Learned


## Notes
### How to Run the Application:
##### Prerequisites:

- Install `Trunk` (WASM web app bundler): `cargo install trunk`

- Install `wasm-bindgen`: `cargo install wasm-bindgen-cli`

##### Set up the WebSocket server:

- You'll need a WebSocket server running at `ws://localhost:9001`

- This could be the server from `Project 55`

##### Run the Yew frontend:
```
trunk serve --open
```
##### Development workflow:

The command will:

- Compile your Rust code to WebAssembly

- Bundle it with your HTML/CSS

- Start a development server

- Open your browser to `http://localhost:8080`

##### Testing:
- Open multiple browser tabs/windows to test the chat functionality

- Messages should appear in real-time across all connected clients











