# Project 061 – Hello Actix-Web App

## What I Built

A HTTP web server using the Actix-Web framework. This simple app responds with "Hello, Rust Web!" and introduces routing, handlers, and basic server setup.

## What I Learned

## Notes
###### Visit in Browser:
Open http://127.0.0.1:8080 – You’ll see:

```
👋 Hello, Rust Web!
```

###### Key Concepts:

`#[get("/")]`: Registers a handler for GET requests at /

`HttpResponse::Ok().body(...)`: Returns a plain response

`HttpServer::new()`: Starts the web server

##### Try these endpoints:

- http://localhost:8080

- http://localhost:8080/health

- http://localhost:8080/greet

- http://localhost:8080/api/greet?name=YourName

- http://localhost:8080/metrics
