# Scope Return

You've probably noticed a pattern: we can return things from functions, loops, if statements, and so on. In Rust, *everything* is an expression that can return a value!

In fact, even functions that don't return actually return something:

```rust
fn foo() {}

fn main() {
    let x = foo(); // x is of type ()
    println!("The value of x is: {:?}", x); // Prints: The value of x is: ()
}
```

`()` is the "unit type" - or "empty tuple". It's like `void` in other languages, but it's still a type, and it has exactly one value: `()`.

You can even return directly from scopes:

```rust
fn main() {
    let x = {
        let a = 5;
        let b = 10;
        a + b // The value of the block is a + b
    };
    println!("The value of x is: {}", x); // Prints: The value of x is: 15
}
```

> This can be get messy, but it can also be useful for limiting variable scope in inner calculations.
