# The use keyword and paths

> See `code/day2/modules` for the code.

Modules give you namespacing, but writing full paths everywhere gets old fast. The `use` keyword lets you bring names into scope.

## Absolute vs Relative Paths

From anywhere in your crate, you can refer to an item with an absolute path starting at the crate root:

```rust
crate::file_module::file_module_greet();
```

Inside a module, you also get a few helpful keywords for relative paths:

- `self::` means "this module"
- `super::` means "the parent module"
- `crate::` means "the crate root"

## Using `use` to Import Names

Instead of writing the full path, import what you want:

```rust
use crate::file_module::file_module_greet;

fn main() {
    file_module_greet();
}
```

You can also import a module and keep the module name:

```rust
use crate::file_module;

fn main() {
    file_module::file_module_greet();
}
```

Renaming imports is common for disambiguation:

```rust
use crate::file_module as fm;
```

## `super::` and `self::`

These are most useful inside nested modules. For example, from a child module you can reach back up to the parent:

```rust
use super::some_parent_function;
```

And you can explicitly refer to sibling items via the parent:

```rust
use super::sibling_module::some_function;
```

## Re-exporting with `pub use`

You can use `pub use` to re-export items and present a cleaner API.

In `code/day2/modules`, `file_module.rs` contains:

```rust
mod foo;
pub use foo::interior_greet;
```

That lets callers write:

```rust
file_module::interior_greet();
```

Instead of:

```rust
file_module::foo::interior_greet();
```

This pattern is common: keep the internal module layout flexible, and expose a small public surface via `pub use`.
