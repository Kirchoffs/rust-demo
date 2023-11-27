# Notes
## Get Started
```
>> cargo add anyhow async-trait sqlparser polars reqwest tokio tracing

>> cargo add --dev tracing-subscriber tokio
```

```
>> cargo run --example dialect
```

## Command
```
>> tokei src/
```

## Rust Knowledge
### Cursor
In Rust, a Cursor is a type provided by the standard library in the std::io module. It wraps an in-memory buffer and provides it with a Read and/or Write implementation, depending on the nature of the buffer. This allows you to use in-memory data with APIs that expect something implementing the Read or Write trait, such as file or network IO operations.

A Cursor can be particularly useful for testing IO operations without the need to interact with real files or network services.

### Orphan Rule
The Orphan Rule in Rust is an important regulation regarding the implementation of traits, crucial for maintaining the consistency of the type system and preventing certain types of conflicts. This rule specifies the conditions under which you can implement a specific trait for a type.

The Orphan Rule states that you can only implement trait X for type T under one of the following two conditions:
- If the trait X is defined in your crate, you can implement it for any type T.
- If the type T is defined in your crate, you can implement any trait for this type.

Example:  
Suppose there is a type Foo defined in crate A and a trait Bar defined in crate B. In crate C, you cannot implement Bar for Foo because neither is defined in C. However, you can implement Bar for Foo in crate A, or implement Bar for Foo in crate B (if Foo is public).

Although the Orphan Rule restricts the implementation of traits, there are times when you might need to implement an external trait for a type not defined in your crate. In such cases, you can use one of the following methods:
- Wrapper Types: Create a new type that wraps the original type, and then implement the trait for this new type.
- New Trait: Create a new trait that contains the methods you want, and then implement this new trait for your type.

### Spectial Struct
#### Tuple Struct
```
struct Color(u8, u8, u8);

let red = Color(255, 0, 0);
```

```
pub struct Expression(pub(crate) Box<SqlExpr>);
```


#### Unit Struct
```
struct UnitStruct;

let unit_instance = UnitStruct;
```

### Transform
#### From String to Arc<str>
```
let my_string = String::from("Hello, Rust!");

// Convert String to &str
let my_str: &str = &my_string;

// Create Arc<str> from &str
let my_arc_str: Arc<str> = Arc::from(my_str);

println!("Original String: {}", my_string);
println!("Arc<str>: {}", my_arc_str);
```
