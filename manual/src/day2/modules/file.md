# File modules

> See `code/day2/modules` for the code.

Most modules live in separate files.

From the crate root (`src/main.rs`), declaring a module like this:

```rust
mod file_module;
```

Tells Rust to load `src/file_module.rs` and treat it as the `file_module` module.

Inside `src/file_module.rs`, you can declare submodules:

```rust
mod foo;
```

This loads `src/file_module/foo.rs` as `file_module::foo`.

This example also demonstrates a common pattern: re-exporting items so callers don't need to know your internal module layout.

```rust
pub use foo::interior_greet;
```

Now `main.rs` can call `file_module::interior_greet()` directly, even though the function is defined in `file_module::foo`.

Visibility is always relative to modules. If something isn't marked `pub`, it can't be used from outside the module it lives in. One common style is to keep submodules private and re-export a small public API from the parent module.
