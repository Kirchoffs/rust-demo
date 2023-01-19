# Notes
## Run
```
>> cargo run --quiet -- post https://www.hyperboloid.com x=1 y=1

or

>> cargo build --quiet && target/debug/httpie post https://www.hyperboloid.com x=1 y=1
```

## Rust Details
In `anyhow` module:
```
pub type Result<T, E = Error> = core::result::Result<T, E>;
```