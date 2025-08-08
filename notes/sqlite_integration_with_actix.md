# Project 066 – SQLite Integration with Actix-Web

## What I Built
An SQLite database into your Actix-Web server using SQLx, enabling persistent storage for your app

## What I Learned


## Notes
#### Database Setup
Create a SQLite file and run this SQL manually or via migration:


```
-- posts.sql
CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL
);
```

#### Apply it via CLI:

`sqlite3 blog.db < posts.sql
`


#### Run the App
```
cargo run
```

#### Test with curl:
1. Add a post:
```
curl -X POST -H "Content-Type: application/json" \
-d '{"title":"SQLite Test", "content":"Rust + SQLx is cool"}' \
http://127.0.0.1:8080/posts
```

2. List posts:
```
curl http://127.0.0.1:8080/posts
```

