# Simple Enumerations & Simple Match

You can define simple enumerations and use them like enums in other languages. Here's an example:

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}
```

Instead of `switch`, we have `match` (with no fallthrough) - but otherwise it should look familiar.

If you add another color, you get a compile-time error if you don't handle it in the `match`:

```rust
enum Color {
    Red,
    Green,
    Blue,
    Purple,
}

fn main() {
    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}
```

This is really handy for ensuring you handle all possible cases! It's also a liability if you decide to match on an integer - so you can use the default case `_` to catch all other cases:

```rust
enum Color {
    Red,
    Green,
    Blue,
    Purple,
}

fn main() {
    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        _ => println!("Some other color"),
    }
}
```

> This is *really* helpful when you are refactoring, unless you use the default case!