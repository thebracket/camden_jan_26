# Loop / While

Let's start with `loop`. It's the most basic loop, and you can use it to build other loops!

Loops run forever unless you tell them to stop. You can use `break` to exit a loop.

```rust
fn main() {
    let mut count = 0;
    loop {
        println!("Count is: {}", count);
        count += 1;
        if count >= 5 {
            break; // Exit the loop when count reaches 5.
        }
    }
}
```

Nothing surprising there. You can also use "labelled break" to break out of nested loops:

```rust
fn main() {
    let mut outer_count = 0;
    'outer: loop {
        let mut inner_count = 0;
        loop {
            println!("Outer: {}, Inner: {}", outer_count, inner_count);
            inner_count += 1;
            if inner_count >= 3 {
                break; // Breaks out of the inner loop.
            }
        }
        outer_count += 1;
        if outer_count >= 2 {
            break 'outer; // Breaks out of the outer loop.
        }
    }
}
```

> Who says Rust doesn't have `goto`? :-)

Now what *does* surprise people is that `loop` is an expression that can return a value!

```rust
fn main() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count >= 5 {
            break count * 2; // Return count * 2 when breaking.
        }
    };
    println!("The result is: {}", result); // Prints: The result is: 10
}
```

> This is a Rust idiom. Pretty much everything can be an expression that returns a value.

## While Loops

While works as you'd expect:

```rust
fn main() {
    let mut count = 0;
    while count < 5 {
        println!("Count is: {}", count);
        count += 1;
    }
}
```
