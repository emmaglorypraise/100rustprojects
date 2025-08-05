# Project 055 – WebSocket Echo Server

## What I Built

A basic WebSocket echo server that handles real-time communication with clients. This project applies bidirectional async messaging, using the tokio and tokio-tungstenite crates to support modern WebSocket connections.

## What I Learned

## Notes
###### Test with a WebSocket Client
You can use:

- websocat:

```
npm install -g wscat  # Install if you don't have it
websocat ws://127.0.0.1:9001.   # In another tab
```