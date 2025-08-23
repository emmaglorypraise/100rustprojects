# Project 073 – Weather Dashboard (Yew + External API)

## What I Built
A simple weather dashboard in Yew that fetches real-time data from a public weather API (OpenWeatherMap). This project teaches API integration, conditional rendering, and handling user input in Yew.

## What I Learned


## Notes
### How to Run the Application:
##### Prerequisites:

- Install `Trunk` (WASM web app bundler): `cargo install trunk`

- Install `wasm-bindgen`: `cargo install wasm-bindgen-cli`

##### Get the OpenWeatherMap API key(free signup):
- Go to https://openweathermap.org/api

- Copy your API key (replace "YOUR_API_KEY" below)

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











