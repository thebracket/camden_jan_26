# Directory modules

> See `code/day2/modules` for the code.

When a module starts growing, it's common to give it a directory.

From the crate root (`src/main.rs`), declaring a module like this:

```rust
mod dir_module;
```

Can be satisfied by either:

- `src/dir_module.rs`
- `src/dir_module/mod.rs`

This example uses `src/dir_module/mod.rs`:

```rust
pub fn hello_from_dir_module() {
    println!("Hello from dir_module!");
}
```

Once you have a module directory, additional submodules can be added as more files/folders under `src/dir_module/` (and declared with `mod ...;` inside `src/dir_module/mod.rs`).

There is also a more "modern" alternative that avoids `mod.rs`: use `src/dir_module.rs` as the module root, and put submodules in `src/dir_module/` as needed. Both styles are widely seen in real code.
