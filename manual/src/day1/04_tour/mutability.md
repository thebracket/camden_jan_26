# Mutability

Coming from C, you'd reasonably expect this to work:

```rust
fn main() {
    let x = 5;
    x = x + 1; // Error: cannot assign twice to immutable variable `x`
    println!("x is now: {}", x);
}
```

In Rust, variables are immutable by default. To make a variable mutable, you need to use the `mut` keyword:

```rust
fn main() {
    let mut x = 5; // `x` is mutable
    x = x + 1;    // Now this works
    println!("x is now: {}", x);
}
```

> You *could* just put `mut` everywhere, but please don't!

## Shadowing

Rust allows you to "shadow" a variable by declaring a new variable with the same name. This can be useful for transforming data while keeping the same name:

```rust
fn main() {
    let x = 5;
    let x = x + 1; // Shadows the previous `x`
    let x = x * 2; // Shadows again
    println!("The final value of x is: {}", x); // Outputs: 12
}
```

Shadowing does *not* remove the original variable! It's still on the stack, it's just not assigned to the name `x` anymore. (The compiler can and will optimize this away in many cases, but conceptually it's still there.)

### Pop Quiz

What value gets printed here?

```rust
fn main() {
    let x = 3;
    {
        let x = 4;
    }
    println!("{x}");
}
```

![](../../images/ScrollTime.png)

The answer is `3`. The inner block creates a new variable `x` that shadows the outer `x`, but once the block ends, the inner `x` goes out of scope, and the outer `x` is still `3`.