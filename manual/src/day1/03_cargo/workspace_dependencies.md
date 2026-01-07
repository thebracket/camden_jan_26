# Workspace Dependencies

The setup we have right now works - but what if multiple crates all want to rely on `colored`? Here's what we have now:

```toml
[dependencies]
colored = "3.0.0"
test_library = { path = "test_library" }
```

The `[dependencies]` section *only* applies to the current crate - in this case, "hello world". That's fine with two crates---but once your complexity starts to tick up, you probably don't want 30 versions of the same library hanging around. You definitely don't want to have to update 30 `Cargo.toml` files when you need to upgrade!

In the *top-level* `Cargo.toml`, you can add:

```toml
[workspace.dependencies]
colored = "3.0.0"
```

Now *any* crate in the workspace can opt-in to using the workspace dependency as follows:

```toml
[dependencies]
colored.workspace = true
```

It's not quite as easy as `cargo add`, but you gain a lot of advantages:

* Shared build artifacts.
* Improved compile times.
* You *know* that your versions are the same throughout your project.
* Lower disk-space usage.

## When To Use Workspace Dependencies

Workspace dependencies are a powerful tool, but they can be a premature optimization. They work best when your project is large, and you need to start imposing some order on your dependencies. It's not a great idea when you're starting out, writing exploratory code, or working on a small project.

Workspace dependencies are a great fit when:

* You have a large project with multiple crates in your workspace.
* You want to ensure consistent versions of dependencies across all crates.
* You want to reduce build times and disk space usage by sharing compiled artifacts.

## When Not To Use Workspace Dependencies

* In this class! The examples in the repo use them to save space - but you'll have a much easier time if you treat each crate as its own project while we work through the examples.
* When you have a small project with only one or two crates.

> **WARNING**: Workspace dependencies have been the single largest cause of confusion when I'm teaching. Let's not use them today - but be aware that they exist. It's really handy in real projects where you need strict version control of dependencies.
