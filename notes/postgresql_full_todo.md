# Project 068 – Todo App with Actix-Web + PostgreSQL (Full CRUD)  

## What I Built
A complete full-featured Todo application backed by PostgreSQL using Actix-Web and SQLx. This project ties together everything from database modeling, routing, error handling, to CRUD operations—all structured for scalable web development.

This app is structured  as:

- Organized endpoints

- Middleware-ready

- Documentation-friendly responses

#### Full support for:

- GET /todos (List)

- POST /todos (Create)

- GET /todos/{id} (View one)

- PUT /todos/{id} (Update)

- DELETE /todos/{id} (Delete)

## What I Learned

#### Step 1: Set Up PostgreSQL Database
bash
```
# Start PostgreSQL (method depends on your OS)
# On macOS with Homebrew:
brew services start postgresql

# On Ubuntu:
sudo service postgresql start

# On Windows: Start PostgreSQL from Services
bash
```
#### Access PostgreSQL 
```
psql postgres
# or
psql -U $(whoami) postgres
# or
sudo -u postgres psql
```
Once in psql, run:
```
sql
-- Create database
CREATE DATABASE todo_db;

-- Connect to the database
\c todo_db;

-- Create the todos table
CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);

-- Exit psql
\q
```
#### Step 2: Create Environment File
Create a .env file in your project root:
```
bash
touch .env
Edit .env with your database URL:

text
DATABASE_URL=postgres://username:password@localhost/todo_db
```
Important: Replace username:password with your actual PostgreSQL credentials. If you don't have a password, use:
```
text
DATABASE_URL=postgres://username@localhost/todo_db
```

#### Step 3: Build and Run the Application
```
# Build the project
cargo build

# Run the application
cargo run
You should see:

text
✅ Todo API running at http://127.0.0.1:8080
```
Step 7: Test the API
Open a new terminal and test each endpoint:
```
# Health check
curl http://localhost:8080/health

# Add a todo
curl -X POST -H "Content-Type: application/json" \
-d '{"title": "Learn Actix Web with PostgreSQL"}' http://localhost:8080/todos

# Get all todos
curl http://localhost:8080/todos

# Get specific todo
curl http://localhost:8080/todos/1
 
# Update todo
curl -X PUT -H "Content-Type: application/json" \
-d '{"completed": true}' http://localhost:8080/todos/1
 
# Delete todo
curl -X DELETE http://localhost:8080/todos/1
```

## Notes