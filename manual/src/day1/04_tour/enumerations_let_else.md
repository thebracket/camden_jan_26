# Let Else

Our nesting is still a little out-of-hand. Rust added `let else` to help with this. Here's how it looks:

```rust
fn my_function(a: Option<i32>, b: Option<i32>, c: Result<i32, String>) {
    let Some(a_val) = a else {
        println!("a is None");
        return;
    };
    let Some(b_val) = b else {
        println!("b is None");
        return;
    };
    let Ok(c_val) = c else {
        println!("Error in c");
        return;
    };
    println!("All values are present: {}, {}, {}", a_val, b_val, c_val);
}
```

Much better. My rule of thumb is that functions check their inputs with `let else` at the top, and then the "happy path" of the function continues after that.