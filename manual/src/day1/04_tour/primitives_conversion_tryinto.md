# Converting with TryInto

> We're using a `Result` type here, which we haven't covered yet. Don't worry about it---we'll pretend we're Cloudflare and use `unwrap()`, and learn why that's a bad idea. We'll cover the *good* idea later!

When you want to narrow a type, but *not* risk losing data, you can use `try_into()`. This method is part of the `TryInto` trait, which is implemented for many primitive types in Rust.

For example:

```rust
fn main() {
    let i: u16 = 256;
    let j: u8 = i.try_into().unwrap();
    println!("{i} {j}");
}
```

When we run this, you see:

```
thread 'main' (13) panicked at src/main.rs:3:30:
called `Result::unwrap()` on an `Err` value: TryFromIntError(())
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

We explicitly panicked! That's what `unwrap()` does when it encounters an error. Sometimes, this is what you want - usually in production code you should handle the error properly.