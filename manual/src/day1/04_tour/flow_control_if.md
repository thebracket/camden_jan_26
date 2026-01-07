# If

If statements work a lot like other languages, but without the brackets. Here's a simple example:

```rust
fn main() {
    let number = 7;
    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is equal to 5");
    } else {
        println!("The number is greater than 5");
    }
}
```

You can return values from `if` statements as well:

```rust
fn main() {
    let number = 7;
    let description = if number < 5 {
        "less than 5"
    } else if number == 5 {
        "equal to 5"
    } else {
        "greater than 5"
    };
    println!("The number is {}", description);
}
```