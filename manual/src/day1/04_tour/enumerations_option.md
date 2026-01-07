# Option: the null replacement

We've used `Option<T>` a few times already. Option is nothing special (beyond a bunch of helper functions!) - it's just an enum defined in the standard library like this:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

You use it where other languages might use "null" or a sentinel value to indicate the *absence* of a value. And because you *have* to destructure it in some fashion - you can't forget to handle the "no value" case!

So you can destructure it exactly like an enum - because it is one!

```rust
fn main() {
    match (12_i32).checked_div(0) {
        Some(result) => println!("Result is {}", result),
        None => println!("Division by zero is bad for you"),
    }
}