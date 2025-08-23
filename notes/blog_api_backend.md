# Project 068 – Blog API Backend (Posts, Comments, Categories) 

## What I Built
A full-featured Blog API backend using Actix-Web and PostgreSQL. Where I implement endpoints for posts, comments, and categories, creating relationships and learned about joined data, foreign keys, and nested querying.


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
-- Create database
CREATE DATABASE blog_db;

-- Connect to the database
\c blog_db;

-- Create the blog table
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);
 
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    category_id INTEGER REFERENCES categories(id)
);
 
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    post_id INTEGER REFERENCES posts(id),
    author TEXT NOT NULL,
    content TEXT NOT NULL
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
DATABASE_URL=postgres://blog_user:your_password@localhost/blog_db

```
Important: Replace username:password with your actual PostgreSQL credentials. If you don't have a password, use:
```
text
DATABASE_URL=postgres://username@localhost/blog_db
```

#### Step 3: Build and Run the Application
```
# Build the project
cargo build

# Run the application
cargo run

You should see:

📝 Blog API running at http://localhost:8080

```
Step 7: Test the API
Open a new terminal and test each endpoint:
```
#Create a category:
curl -X POST -H "Content-Type: application/json" \
-d '{"name":"Rust"}' http://localhost:8080/categories

#Create a post:
curl -X POST -H "Content-Type: application/json" \
-d '{"title":"Actix Rocks","body":"It's blazing fast.","category_id":1}' \
http://localhost:8080/posts

#Add a comment:
curl -X POST -H "Content-Type: application/json" \
-d '{"post_id":1,"author":"Alice","content":"Love it!"}' \
http://localhost:8080/comments
```

## Notes