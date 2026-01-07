# Converting with Into

The safest way to convert between types is with `into()`. This method is part of the `Into` trait, which is implemented for many primitive types in Rust.

For example:

```rust
fn main() {
    let i: u8 = 64;
    let j: u16 = i.into();
    println!("{i} {j}");
}
```

Into is *only* defined for conversions that are guaranteed to be safe. For example, this will not compile:

```rust
fn main() {
    let i: u16 = 64;
    let j: u8 = i.into();
    println!("{i} {j}");
}
```
