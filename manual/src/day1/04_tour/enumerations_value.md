# Value Enumerations

Just like C/C++ enums, you can assign values to enumeration variants. Here's an example:

```rust
enum Number {
    One = 1,
    Two = 2,
    Three = 3,
}

fn main() {
    let n = Number::Two;
    let value = n as u32; // Cast enum to its underlying value
}
```

(Unfortunately, the reverse - getting an enum from a value - isn't automatic!)

You can even specify the type of the underlying values:

```rust
enum Number {
    One = 1u8,
}
```

So far - nothing magical.