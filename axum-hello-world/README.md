# Notes

## Get Started
```
>> cargo install cargo-watch

>> cargo new axum-hello-world
>> cd axum-hello-world
>> cargo add axum
>> cargo add tokio -F macros -F rt-multi-thread
>> cargo add serde -F derive
>> cargo add axum -F headers
>> cargo add tower-http
>> cargo add tower-http -F cors
```
Side note: cargo-install is a global intallation, it is like nodemon for NodeJS, air for Golang.

## Documentation
```
>> cargo doc --open
```

## Run
```
>> cargo watch -h
>> cargo watch -x run
```

## Axum Notes
### Router
Router is passing from bottom to up.
```
pub fn create_routes() -> Router<(), Body> {
    Router::new()
        .route("/read_middleware_custom_header", get(read_middleware_custom_header))
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(Extension(shared_data))
        .layer(cors)
}
```
Take above code as an example, `cors` will be applied first because it is at the top of the chain, then `Extension` will be applied.

### Entry
```
pub async fn run() {
    let app = create_routes();

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### Execute
```
#[tokio::main]
async fn main() {
    run().await
}
```

## Rust Nodes
### Rust Nightly
```
>> rustup toolchain list
>> rustup self update
>> rustup update nightly
>> rustc nightly --version
>> rustup default nightly
```

### Trait ToOwned
