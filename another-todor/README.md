# Notes
## Run
```
>> cargo run -p core -- --title coding --status pending
>> cargo run -p core --release -- --title coding --status pending

>> cd todo/core
>> cargo run -- --title coding --status pending
```

## Miscellaneous
```
let args: Vec<String> = env::args().collect();

// or
let args = env::args().collect::<Vec<String>>();
```