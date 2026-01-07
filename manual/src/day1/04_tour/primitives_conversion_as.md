# Casting with "as"

You can *also* use `as`. `as` is dangerous, because it works just like casting in C and C++:

```rust
fn main() {
    let i: u16 = 257;
    let j: u8 = i as u8;
    println!("{i} {j}");
}
```

`as` acts *just* like a C cast, and will truncate values without warning. When you run this, you'll see:

```
257 1c
```

We've successfully achieved the same bug as the C version! That may not be a good thing, depending upon what you're trying to do.

> Let's open `code/day1/convert_as`.

If you `cargo run`, you see the bad output - with no warnings. That's not great. Running the linter with `cargo clippy` doesn't give a warning either!

> The assumption is that if you use `as`, you know what you're doing. So be careful with it!

You *may* want to create a warning. So let's turn it on:

```rust
#![warn(clippy::pedantic)] 
// #! in main.rs or lib.rs applies to the whole crate.
// Pedantic turns on extra warnings.

fn main() {
    let i: u16 = 257;
    let j: u8 = i as u8;
    println!("{i} {j}");
}
```

`cargo build` still doesn't give you a warning, but `cargo clippy` now does.

> It's a really good idea to integrate Clippy into your build process. You catch a *lot* of potential bugs that way!