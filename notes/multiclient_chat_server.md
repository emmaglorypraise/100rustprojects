# Project 032 – Multi-Client Chat Server

## What I Built
A lightweight Rust CLI tool that extends the TCP server to support multiple clients chatting together. This project introduces shared state across threads, broadcasting messages, and synchronizing sockets using Arc<Mutex<>>.

## What I Learned
This code implements a multi-client chat server using TCP in Rust, where a server listens for incoming client connections, handles messages, and broadcasts them to all other connected clients. The server runs on a specific IP address and port (127.0.0.1:7878). It uses a TcpListener to accept client connections and spawns a new thread for each client using thread::spawn. Each client connection is represented by a TcpStream, which allows data to be sent and received over a network. The server stores these connections in a HashMap, where each key is the client's address (as a String), and the value is the corresponding TcpStream. To handle multiple clients simultaneously, the server uses Arc (Atomic Reference Counting) and Mutex to ensure safe, concurrent access to the HashMap shared between threads.

In each thread, the server reads messages from the connected client using a BufReader. When a message is received, it formats the message with the client's address and broadcasts it to all other connected clients. If a client fails to receive a message (e.g., due to disconnection), it is removed from the list of active clients. This is achieved by locking the Mutex to safely modify the shared HashMap of clients. The server prints the disconnection message once a client disconnects, and it removes the client from the list of active connections. This implementation demonstrates core client-server concepts, where the server manages multiple clients, listens for connections, and relays messages between clients in a real-time chat environment.

## Notes
A multi-client chat server is a server application designed to facilitate real-time communication among multiple connected clients. This type of server acts as a central hub, receiving messages from individual clients and then distributing those messages to other designated clients, enabling a group chat environment or private messaging capabilities.

### Test Using Netcat 
```
nc 127.0.0.1 7878
```

### Test chatting:

Type messages in one terminal, they should appear in all other connected terminals

Try disconnecting clients to see the cleanup in action


### Testing Scenarios to Verify:
##### Basic messaging:

Connect Client A and Client B

Send message from A - verify B receives it

Send message from B - verify A receives it

##### Multiple clients:

Connect 3+ clients

Verify messages broadcast to all

##### Disconnection handling:

Connect several clients

Disconnect one abruptly (close terminal)

Verify server removes it from list

Verify other clients can still communicate

###### Concurrency testing:

Connect many clients (10+) simultaneously

Verify no messages are lost

##### Error cases:

Try connecting to wrong port

Send very long messages

Send binary data



