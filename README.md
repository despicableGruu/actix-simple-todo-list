# Simple TODO list API made in Rust.

## Requirements

- Rust
- Docker
- docker-compose

## Usage

```bash
# Copy example .env file
cp .env.example .env

# Run postgres
docker-compose up -d postgres

# Install diesel
cargo install diesel_cli --no-default-features --features postgres

# Run db migrations
DATABASE_URL=postgres://actix:actix@localhost:5432/actix diesel migration run

# Run unit tests
cargo test

# Run integration tests
cargo test --features "integration"

# Run the server (Add --release for an optimized build)
cargo run 
```

To test the API, you can use `curl`:

```bash
curl -s http://localhost:8080/
```


## Routes

### Status

- **Method:** `GET`
- **Endpoint:** `/`
- **Response:**
  ```json
  {
    "status": "Up"
  }
  ```

### Todo Lists

- **Method:** `GET`
- **Endpoint:** `/todos`
- **Response:**
  ```json
  [
    {
      "id": 1,
      "title": "Grocery list"
    },
    ...
  ]
  ```

- **Method:** `GET`
- **Endpoint:** `/todos/{id}` (e.g., `/todos/1`)
- **Response:**
  ```json
  {
    "id": 1,
    "title": "Grocery list"
  }
  ```

- **Method:** `POST`
- **Endpoint:** `/todos`
- **Request Header:**
  ```
  Content-Type: application/json
  ```
- **Request Body:**
  ```json
  {
    "title": "List title"    
  }
  ```
- **Response:**
  ```json
  {
    "id": 1,
    "title": "Grocery list"
  }
  ```


### Todo List Items

- **Method:** `GET`
- **Endpoint:** `/todos/{todo_id}/items` (e.g., `/todos/1/items`)
- **Response:**
  ```json
  [
    {
      "id": 1,
      "list_id": 1,
      "title": "Milk",
      "checked": true
    },
    {
      "id": 1,
      "list_id": 2,
      "title": "Bread",
      "checked": false
    }
  ]
  ```

- **Method:** `GET`
- **Endpoint:** `/todos/{todo_id}/items/{item_id}` (e.g., `/todos/1/items/1`)
- **Response:**
  ```json
  {
    "id": 1,
    "list_id": 1,
    "title": "Milk",
    "checked": true
  }
  ```

- **Method:** `POST`
- **Endpoint:** `/todos/{todo_id}/items` (e.g., `/todos/1/items`)
- **Request Header:**
  ```
  Content-Type: application/json
  ```
- **Request Body:**
  ```json
  {
    "title": "Eggs"    
  }
  ```
- **Response:**
  ```json
  {
    "id": 1,
    "list_id": 1,
    "title": "Eggs",
    "checked": false
  }
  ```

- **Method:** `PUT`
- **Endpoint:** `/todos/{todo_id}/items/{item_id}` (e.g., `/todos/1/items/1`)
- **Response:**
  ```json
  {
    "result": true
  }
  ```
  **Result:**
  - `true`: Item checked.
  - `false`: Item was already checked. Nothing to do. 
```