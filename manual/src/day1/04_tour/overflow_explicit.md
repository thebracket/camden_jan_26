# Handling This Safely: Make intent clear, use checked function

Rust offers a bunch of functions to handle overflow/underflow explicitly. In addition to safety and correctness, there's a big benefit: you are making it *really obvious* to anyone reading your code what your intent is.

## Checked Arithmetic

```rust
fn main() {
    let a: u8 = 200;
    let b: u8 = 100;

    match a.checked_add(b) {
        Some(result) => println!("Result: {}", result),
        None => println!("Overflow occurred!"),
    }
}
```

There are `checked_add`, `checked_sub`, `checked_mul`, `checked_div`, and `checked_rem` functions for all integer types.

## Wrapping Arithmetic

If you want to allow overflow/underflow but want to make it explicit, you can use the `wrapping_*` functions:

```rust
fn main() {
    let a: u8 = 250;
    let b: u8 = 10;

    let result = a.wrapping_add(b);
    println!("Result with wrapping: {}", result); // Outputs: 4
}
```

## Saturating Arithmetic

If you want to ensure that values stay within bounds without wrapping around, you can use the `saturating_*` functions:

```rust
fn main() {
    let a: u8 = 250;
    let b: u8 = 10;
    let result = a.saturating_add(b);
    println!("Result with saturation: {}", result); // Outputs: 255
}
```
