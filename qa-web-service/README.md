# Notes
## Test
### CORS
#### Preflight Request
```
>> curl -i -X OPTIONS -H 'Origin: http://localhost' -H 'Access-Control-Request-Method: PUT' http://localhost:6174/questions
>> curl -i -X OPTIONS -H 'Origin: http://localhost' -H 'Access-Control-Request-Method: PUT' -H 'Access-Control-Request-Headers: X-Custom-Alpha-Header, X-Custom-Beta-Header' http://localhost:6174/questions
```

- -I, --head  
This fetches only the headers of the response
- -i  
This includes the headers in the output along with the response body
- -v, -verbose  
This option makes the curl command more talkative and will output information on the progress of the transfer
- -w "HTTP status code: %{http_code}\n"  
This option will output the HTTP status code of the response

#### Access-Control-Allow-Headers
This header is used in response to a preflight request to indicate which HTTP headers can be used when making the actual request.

By default, only the following headers are exposed:
- Accept
- Accept-Language
- Content-Language
- Content-Type (only for values application/x-www-form-urlencoded, multipart/form-data, and text/plain)

If you want to send other headers, you have to set them in the Access-Control-Allow-Headers header. For example:
```
Access-Control-Allow-Headers: X-Custom-Alpha-Header, X-Custom-Beta-Header
```

### Curl
```
>> curl localhost:6174/questions?x=1

>> curl 'localhost:6174/questions?start=0&end=1'

>> curl -X POST 'localhost:6174/questions' -H 'Content-Type: application/json' -d '{"id": "2", "title": "What is Rust?", "content": "Rust is a systems programming language", "tags": ["rust", "systems programming"]}'
```

The question mark (?) is a special character in the shell. The ? character is actually used as a wildcard in the shell to match any single character in filenames. If you want to use it as a part of the URL, you have to put the URL in double quotes.

### Content-Type
#### Difference between application/x-www-form-urlencoded and application/json
```
curl \
  --location \
  --request POST 'localhost:6174/questions' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data-urlencode 'id=5' \
  --data-urlencode 'title=First question' \
  --data-urlencode 'content=This is the question I had.'
```

```
curl \
  --location \
  --request POST 'localhost:6174/questions' \
  --header 'Content-Type: application/json' \
  --data-raw '{
    "id": "6",
    "title": "New question",
    "content": "How and why?"
  }'
```

### Complete Test
```
curl --location \
     --request POST 'localhost:6174/registration' \
     --header 'Content-Type: application/json' \
     --data-raw '{
       "email": "benjamin@email.com",
       "password": "passphrase"
     }'

curl --location \
     --request POST 'localhost:6174/login' \
     --header 'Content-Type: application/json' \
     --data-raw '{
       "email": "benjamin@email.com",
       "password": "passphrase"
     }'

USER_TOKEN=$(
  curl --location \
       --request POST 'localhost:6174/login' \
       --header 'Content-Type: application/json' \
       --data-raw '{
         "email": "benjamin@email.com",
         "password": "passphrase"
       }' | sed 's/^"\(.*\)"$/\1/' \
)

USER_TOKEN=$(
  curl --location \
       --request POST 'localhost:6174/login' \
       --header 'Content-Type: application/json' \
       --data-raw '{
         "email": "benjamin@email.com",
         "password": "passphrase"
       }' | jq -r '.' \
)

curl --location \
     --request POST 'localhost:6174/questions' \
     --header "Authorization: ${USER_TOKEN}" \
     --header 'Content-Type: application/json' \
     --data-raw '{
       "title": "How can I code better?",
       "content": "Any tips for a Junior developer?",
       "tags": ["programming", "junior"]
     }'

curl --location \
     --request PUT 'localhost:6174/questions/1' \
     --header "Authorization: ${USER_TOKEN}" \
     --header 'Content-Type: application/json' \
     --data-raw '{
       "id": 1,
       "title": "What can I do to improve my coding skills?",
       "content": "Any tips for a Junior developer?",
       "tags": ["programming", "junior"]
     }'

curl --location \
      --request GET 'localhost:6174/questions' \
      --header 'Content-Type: application/json'
```

Note:
Quotes for Authorization Header: Use double quotes (`"Authorization: ${USER_TOKEN}"`) instead of single quotes. Single quotes prevent variable substitution in most shells, so `${USER_TOKEN}` is treated literally as a string instead of being replaced with its value.

## Rust Notes
### Install Dependencies
```
>> cargo add sqlx --features runtime-tokio-rustls,migrate,postgres

>> cargo add platforms --build
```

### Applications & Libraries
- Applications
  - cargo new app_name
  - src/main.rs
  - executable
  - binary crate
- Libraries
  - cargo new --lib lib_name
  - src/lib.rs
  - library
  - library crate

### HashMap
For key of HashMap, the type must implement the Eq, PartialEq, and Hash traits.

### RwLock
#### Difference between std::sync::RwLock and tokio::sync::RwLock
- std::sync::RwLock
  - Blocking
  - Can be used in sync code

- tokio::sync::RwLock
  - Non-blocking
  - Can be used in async code

### Rust Doc
```
>> cargo doc --open
```

### Linting
#### Clippy
```
>> rustup component add clippy

>> cargo clean
>> cargo clippy
```

#### Rustfmt
```
>> rustup component add rustfmt

>> cargo fmt
```

### Logging
`Log` crate is the facade for logging libraries in Rust.
Other logging libraries like `env_logger` can use `Log` facade to log messages.

#### Use `env_logger` crate
- RUST_LOG
  - Sets the log level for all crates
  - Example: `RUST_LOG=info`

```
env_logger::init();
```

```
>> RUST_LOG=info cargo run
```

#### Use `log4rs` crate
```
log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
```

#### Filter log for Warp
```rust
let log = warp::log::custom(|info| {
    eprintln!(
        "{} {} {}",
        info.method(),
        info.path(),
        info.status(),
    );
});

let routes = warp::get()
    .and(warp::path("hello"))
    .and(warp::path::param())
    .and(warp::path::end())
    .and(log);
```

```rust
log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

log::error!("This is an error message");

let log = warp::log::custom(|info| {
    log::info!(
        "{} {} {} {:?} from {} with {:?}",
        info.method(),
        info.path(),
        info.status(),
        info.elapsed(),
        info.remote_addr().unwrap(),
        info.request_headers()
    );
});

let routes = warp::get()
    .and(warp::path("hello"))
    .and(warp::path::param())
    .and(warp::path::end())
    .and(log);

log::info!("Start querying questions");
```

### Debugging
#### Debugging with `lldb` on ARM-based Mac:
```
>> cargo build
>> lldb target/debug/qa-web-service

(lldb) >> b add_question
(lldb) >> b src/routes/question.rs:15
(lldb) >> breakpoint list
(lldb) >> r
(lldb) >> n
```

### Token
```rust
// Version 1
fn issue_token(account_id: AccountId) -> String {
    let state = serde_json::to_string(&account_id).expect("failed to serialize account id");
    local_paseto(&state, None, "RANDOM WORDS WINTER MACINTOSH PC".as_bytes()).expect("failed to issue token")
}

// Version 2
fn issue_token(account_id: AccountId) -> String {
    let current_date_time = Utc::now();
    let expiration_date_time = current_date_time + chrono::Duration::days(1);

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from("RANDOM WORDS WINTER MACINTOSH PC".as_bytes()))
        .set_expiration(&expiration_date_time)
        .set_not_before(&current_date_time)
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("failed to construct paseto token")
}
```

### Compilation
```
>> rustup target list
```

```
>> rustup target install x86_64-apple-darwin
>> cargo build --release --target x86_64-apple-darwin
```

```
>> rustup target install x86_64-unknown-linux-musl
```

The musl project provides an implementation of the standard C library optimized for static linking. Build systems can use musl to make system calls to the operating systems, and musl translates the calls to the operating system it is currently running on.

## Postgres
### Postgres in Docker
#### Install
```
>> docker pull postgres

or

>> docker pull postgres:17.2
```

#### Run by Docker
```
>> docker run -itd -e POSTGRES_USER=fudou -e POSTGRES_PASSWORD=6174 -p 5432:5432 -v ./data:/var/lib/postgresql/data --name postgresql postgres

>> docker exec -it postgresql bash
(postgresql) >> psql -U fudou

or
>> docker exec -it postgresql psql -U fudou -c "\l"
```

Note: Differentiate between `docker run`, `docker start`, and `docker exec`.

#### Run by PSQL
For MacOS:

```
>> brew doctor
>> brew update
>> brew install libpq
```

```
>> PGPASSWORD=6174 psql -h localhost -p 5432 -U fudou
>> PGPASSWORD=6174 psql -h localhost -p 5432 -U fudou -d qa
>> PGPASSWORD=6174 psql -h localhost -p 5432 -U fudou -c '\l'  
```

You can also use the CMD createdb outside of the psql shell.

```
>> PGPASSWORD=6174 createdb tmp -h localhost -p 5432 -U fudou
```

#### Run SQL
```
>> PGPASSWORD=6174 psql -h localhost -p 5432 -U fudou
(postgresql) >> \du
(postgresql) >> CREATE DATABASE qa;
(postgresql) >> \c qa fudou
(postgresql) >> \l
(postgresql) >> \h CREATE INDEX

>> PGPASSWORD=6174 psql -h localhost -p 5432 -U fudou -d qa
```

```
(postgresql) >>
CREATE TABLE IF NOT EXISTS questions (
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

(postgresql) >> 
CREATE TABLE IF NOT EXISTS answers (
    id serial PRIMARY KEY,
    content TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    corresponding_question integer REFERENCES questions
);

(postgresql) >> SELECT current_database();

# Display Table
(postgresql) >> \dt

# Display Index
(postgresql) >> \di

(postgresql) >> \d questions

(postgresql) >> INSERT INTO questions (title, content, tags) VALUES ('What is Rust?', 'Rust is a systems programming language.', '{rust, systems programming}');

(postgresql) >> DROP TABLE questions, answers;

(postgresql) >> \q
```

### Migration
```
>> cargo install sqlx-cli
```

```
>> sqlx migrate add -r questions_table
```

In {timestamp}_questions_table.up.sql:
```sql
CREATE TABLE IF NOT EXISTS questions (
    id serial PRIMARY KEY,      
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);
```

In {timestamp}_questions_table.down.sql:
```sql
DROP TABLE questions;
```

Log into Postgres (ensure the database is the one you want to use):
```
(postgresql) >> DROP TABLE answers, questions;
```

Run the following command to apply the migration:
```
>> export DATABASE_URL=postgres://fudou:6174@localhost:5432/qa
>> sqlx migrate run

or
>> sqlx migrate run --database-url postgres://fudou:6174@localhost:5432/qa
```

To revert the migration:
```
>> sqlx migrate revert
```

To check the current migration status:
```
>> sqlx migrate info
```

To redo the last migration (revert it and reapply):
```
>> sqlx migrate redo
```

Perform the same steps for the answers table.
```
>> sqlx migrate add -r answers_table
```

```sql
CREATE TABLE IF NOT EXISTS answers (
    id serial PRIMARY KEY,
    content TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    corresponding_question integer REFERENCES questions
);

DROP TABLE IF EXISTS answers;
```

Also for the accounts table:
```
>> sqlx migrate add -r accounts_table
```

```sql
CREATE TABLE IF NOT EXISTS accounts (
    id serial NOT NULL,
    email VARCHAR(255) NOT NULL PRIMARY KEY,
    password VARCHAR(255) NOT NULL
);

DROP TABLE IF EXISTS accounts;
```

Extend the questions & answers table:
```
>> sqlx migrate add -r extend_questions_table
>> sqlx migrate add -r extend_answers_table
```

```sql
ALTER TABLE questions
ADD COLUMN account_id serial;

ALTER TABLE questions
DROP COLUMN account_id;
```

## Miscellaneous
### Middleware
Middleware is placed between the incoming HTTP call and the handing-off to the route handler.

### Token
A token is a stateless way to do authentication. You can also choose to have a database table (like Redis) with all active tokens to be able to invalidate tokens in the future.

### Git Rev Parse
`git rev-parse` is a powerful Git command used to parse and retrieve specific information about Git revisions, objects, and references.

### Docker
#### Simplest Dockerfile for the Project
```dockerfile
FROM rust:latest
 
COPY ./ ./
 
RUN cargo build --release
 
CMD ["./target/release/rust-web-dev"]
```

#### Docker Compose
```
>> docker-compose build
>> docker-compose up
```

#### Clean Docker
```
>> docker builder prune
>> docker builder prune --all
```

```
>> docker system prune
```
It will remove all stopped containers, all dangling images, and all unused networks.
