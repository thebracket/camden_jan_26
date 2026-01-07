# If Let

If you have a lot of invariants to check, nesting match statements can get unwieldy. Rust provides `if let` as a way to simplify this. For example, consider this nested match:

```rust
fn my_function(a: Option<i32>, b: Option<i32>, c: Result<i32, String>) {
    match a {
        Some(a_val) => {
            match b {
                Some(b_val) => {
                    match c {
                        Ok(c_val) => {
                            println!("All values are present: {}, {}, {}", a_val, b_val, c_val);
                        }
                        Err(e) => println!("Error in c: {}", e),
                    }
                }
                None => println!("b is None"),
            }
        }
        None => println!("a is None"),
    }
}
```

That's quite incredibly ugly. You can rewrite it using `if let` like this:

```rust
fn my_function(a: Option<i32>, b: Option<i32>, c: Result<i32, String>) {
    if let Some(a_val) = a {
        if let Some(b_val) = b {
            if let Ok(c_val) = c {
                println!("All values are present: {}, {}, {}", a_val, b_val, c_val);
            } else {
                println!("Error in c");
            }
        } else {
            println!("b is None");
        }
    } else {
        println!("a is None");
    }
}
```

"If let" is a "single arm" match statement that only matches one pattern, and if it doesn't match, it goes to the `else` block. This can make your code cleaner and easier to read when you're only interested in one specific case.

You can still end up with deep nesting, but it's better than the match version.