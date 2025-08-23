# Project 070 – E-Commerce Backend with Actix-Web + PostgreSQL

## What I Built
A foundational E-commerce backend that supports products, orders, and customers, creating a RESTful API using Actix-Web + PostgreSQL with relations, data validation, and full CRUD for multiple entities.



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

-- Create user
createuser blog_user

-- Set password for the user
psql -c "ALTER USER blog_user WITH PASSWORD 'your_password';"


-- Create database
CREATE DATABASE ecommerce_db;

-- Grant privileges
psql -c "GRANT ALL PRIVILEGES ON DATABASE ecommerce_db TO ecommerce_user;"

-- Connect to the database
\c ecommerce_db;

-- Create the todos table
CREATE TABLE customers (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL
);
 
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    price DECIMAL NOT NULL
);
 
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER REFERENCES customers(id),
    total DECIMAL DEFAULT 0
);
 
CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id),
    product_id INTEGER REFERENCES products(id),
    quantity INTEGER NOT NULL
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
DATABASE_URL=postgres://username:password@localhost/ecommerce_db
```
Important: Replace username:password with your actual PostgreSQL credentials. If you don't have a password, use:
```
DATABASE_URL=postgres://username@localhost/ecommerce_db
 
```
#### Step 3: Build and Run the Application
```
# Build the project
cargo build

# Run the application
cargo run
You should see:

🛍️ E-Commerce API running at http://localhost:8080

```
Step 7: Test the API
Open a new terminal and test each endpoint:
```
# Create a product:

curl -X POST -H "Content-Type: application/json" \
-d '{"name":"Laptop","price":1499.99}' \
http://localhost:8080/products

# Create a customer:

curl -X POST -H "Content-Type: application/json" \
-d '{"name":"Alice","email":"alice@example.com"}' \
http://localhost:8080/customers

# List customers and products:

curl http://localhost:8080/customers
curl http://localhost:8080/products
```

## Notes