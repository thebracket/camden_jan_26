# Inline modules

> See `code/day2/modules` for the code.

Rust modules are primarily a *namespace* tool. The crate has a module tree, rooted at `main.rs` (binary crate) or `lib.rs` (library crate).

You can define a module inline with a `mod` block:

```rust
mod inline_module {
    pub fn greet() {
        println!("Hello from inline module!");
    }
}

fn main() {
    inline_module::greet();
}
```

Items inside a module are private by default. If you want to call a function from outside the module, mark it `pub`.

Paths are relative to where you are. From the crate root you can always be explicit:

```rust
crate::inline_module::greet();
```
