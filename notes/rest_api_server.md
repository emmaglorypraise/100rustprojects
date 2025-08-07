# Project 062 – REST API Server with Actix-Web

## What I Built
A simple RESTful API using Actix-Web that handles CRUD-like HTTP requests for a Book resource. Covering routing, JSON serialization, and handling dynamic data via endpoints.

## What I Learned


## Notes
### Run the Server
```
cargo run
```
### Test with curl or Postman:
1. GET all books

```
curl http://127.0.0.1:8080/books
```
2. POST new book

```
curl -X POST -H "Content-Type: application/json" \
-d '{"id":1,"title":"Rust in Action","author":"Tim McNamara"}' \
http://127.0.0.1:8080/books
```









