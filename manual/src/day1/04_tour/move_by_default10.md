# Copy Types

After all this "wouldn't you expect this to work?", here's the funny exception (and why I've been using strings all of a sudden):

```rust
fn do_something(n: i32) {
    println!("Doing something with: {}", n);
}

fn main() {
    let my_number = 42;
    do_something(my_number); // Copy it by passing the value.
    println!("Back in main: {}", my_number); // Still valid!
}
```

It... compiles just fine.

## Why?

Some types in Rust implement the `Copy` trait. This means that when you assign them or pass them to functions, they are copied bit-for-bit, and the original remains valid.

Typically, simple scalar types like integers (`i32`, `u64`, etc.), floating-point numbers (`f32`, `f64`), and characters (`char`) implement the `Copy` trait. If a type is made up entirely of `Copy` types, it can also implement `Copy`.

This allows the compiler to use registers and stack space more efficiently, leading to better performance in many cases.