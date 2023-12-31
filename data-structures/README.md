# Notes
## Get Started
```
>> cargo new data-structures --lib
```

## Rust Notes
### Iterator
#### Definition
```
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

#### Forms of Iterators
- iter(), which iterates over &T.
- iter_mut(), which iterates over &mut T.
- into_iter(), which iterates over T.
