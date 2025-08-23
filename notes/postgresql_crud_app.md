# Project 067 – PostgreSQL CRUD App with SQLx + Actix-Web  

## What I Built
An PostgreSQL database into a Actix-Web server using SQLx, enabling scalable, production-grade data storage and implementing CRUD operations for a Todo app.

## What I Learned


## Notes
##### Prerequisites:
###### For .env File:
Create a .env file with your database URL:
```
DATABASE_URL=postgres://postgres:password@localhost/todo_db
```
Make sure the database todo_db exists and PostgreSQL is running.

###### Database Schema:
Run this SQL to initialize the table:

```CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);
```

### How to Run the Application:

##### Run the App
```
 cargo run
 ```
##### Sample Commands:
```
# Add a todo
curl -X POST -H "Content-Type: application/json" \
-d '{"title": "Learn Actix + SQLx"}' http://localhost:8080/todos
 
# List todos
curl http://localhost:8080/todos
 
# Update todo
curl -X PUT -H "Content-Type: application/json" \
-d '{"completed": true}' http://localhost:8080/todos/1
 
# Delete todo
curl -X DELETE http://localhost:8080/todos/1
```