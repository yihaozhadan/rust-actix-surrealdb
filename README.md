# Rust Actix SurrealDB sample program

Todo program based on Rust, Actix and SurrealDB.

Prerequisites:

Start SurrealDB server and enable scripting function by passing _--allow-scripting_ parameter. See SurrealDB [Capabilites](https://docs.surrealdb.com/docs/security/capabilities/).

## Get Start

Once the source code is downloaded, run

```
cargo build
```
or

```
cargo run
```

## Sample HTTP Requests and Responses

__Create a todo__

Request:

```sh
curl --location --request POST 'http://localhost:8080/todo' \
--header 'NS: todo' \
--header 'DB: todo' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--data-urlencode 'description=This is my first todo'
```

Response:

```json
{
    "id": {
        "tb": "task",
        "id": {
            "String": "iu5il1yie3wi9d6n1eub"
        }
    },
    "description": "This is my first todo",
    "completed": false
}
```

__Fetch todo list__

Request:

```sh
curl --location --request GET 'http://localhost:8080/todos'
```

Response:

```json
[
    {
        "id": {
            "tb": "task",
            "id": {
                "String": "iu5il1yie3wi9d6n1eub"
            }
        },
        "description": "This is my first todo",
        "completed": false
    }
]
```


__Fetch a todo__

Request:

```sh
curl --location --request GET 'http://localhost:8080/todo/iu5il1yie3wi9d6n1eub'
```

Response:

```json
{
    "id": {
        "tb": "task",
        "id": {
            "String": "iu5il1yie3wi9d6n1eub"
        }
    },
    "description": "This is my first todo",
    "completed": false
}
```

__Toggle a todo__

Request

```sh
curl --location --request PUT 'http://localhost:8080/todo/iu5il1yie3wi9d6n1eub'
```

Response:

```json
{
    "id": {
        "tb": "task",
        "id": {
            "String": "iu5il1yie3wi9d6n1eub"
        }
    },
    "description": "This is my first todo",
    "completed": true
}
```

__Delete a todo__

Request:

```sh
curl --location --request DELETE 'http://localhost:8080/todo/iu5il1yie3wi9d6n1eub'
```

Response:

```json
{
    "rows_affected": 1
}
```
