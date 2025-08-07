# Project 063 – User Authentication API (Actix-Web)     

## What I Built
A basic user authentication API with login and signup endpoints using in-memory storage. While practicing password hashing, POST data handling, and secure API logic, a key step for secure web applications.

## What I Learned

- `bcrypt` for password hashing & verification

- `Mutex<HashMap>` for safe shared user state

## Notes


### Run the Server
```
cargo run

```
### Test It with curl:
1. Sign up:

```
curl -X POST -H "Content-Type: application/json" \
-d '{"username":"alice", "password":"secret123"}' \
http://127.0.0.1:8080/signup
```
2. Log in:

```
curl -X POST -H "Content-Type: application/json" \
-d '{"username":"alice", "password":"secret123"}' \
http://127.0.0.1:8080/login
```






