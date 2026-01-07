# Result: did it work?

> Note that error handling has its own section!

`Result<T, E>` is just another enumeration (generic) defined in the standard library like this (again, with a bunch of helpers):

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

It is used to allow a function to return *either* a success value of type `T` *or* an error value of type `E`. This is how Rust handles errors instead of exceptions.

Here's an example of using `Result`:

```rust
fn main() {
    match "123".parse::<i32>() {
        Ok(value) => println!("Parsed value: {}", value),
        Err(e) => println!("Failed to parse integer: {}", e),
    }
}
```