# Project 064 – CRUD API with Actix-Web    

## What I Built
A fully functional CRUD API for a Post resource using Actix-Web, storing data in-memory tohandle GET, POST, PUT, DELETE requests, core operations in any RESTful service.

## What I Learned


## Notes

### Run the Server
```
cargo run
```

### Test It with curl:
1. Create a post:
```
curl -X POST -H "Content-Type: application/json" \
-d '{"id": 1, "title": "Hello", "content": "Actix is awesome"}' \
http://127.0.0.1:8080/posts
```

2. List posts:
```
curl http://127.0.0.1:8080/posts
```

3. Update a post:
```
curl -X PUT -H "Content-Type: application/json" \
-d '{"id": 1, "title": "Updated", "content": "Edited content"}' \
http://127.0.0.1:8080/posts/1
```

4. Delete a post:
```
curl -X DELETE http://127.0.0.1:8080/posts/1
```