# Project 065 – (JSON Web Token) JWT Authentication in Actix-Web

## What I Built
A secure JWT-based authentication in Actix-Web. Created login endpoints that return a JSON Web Token, and a protected route that requires a valid token in the request header. This is essential for stateless API security.

## What I Learned
### Key Concepts:
- `jsonwebtoken` for secure, signed tokens

- `Authorization headers` for bearer tokens

- `Token claims` include user identity and expiration

## Notes

### Run the App
cargo run

### Test with curl:
1. Login and get token:
```
curl -X POST -H "Content-Type: application/json" \
-d '{"username":"admin", "password":"password"}' \
http://127.0.0.1:8080/login
```

2. Use token to access protected route:
```
curl -H "Authorization: Bearer <TOKEN>" \
http://127.0.0.1:8080/protected
```










