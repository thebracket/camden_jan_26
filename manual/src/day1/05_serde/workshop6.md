# WS Part 2: CLAP for a Login CLI

Now that we have a login library, let's see how easy the crates infrastructure makes it to build a management CLI around it - and to consume libraries we've built ourselves.

In a parallel directory to the `login_lib` crate, create a new binary crate called `login_cli`:

```bash
cargo new login_cli
```

Let's add `clap` as a dependency in the `Cargo.toml`:

```bash
cd login_cli
cargo add clap -F derive
```

And edit `Cargo.toml` to add our library as a dependency:

```toml
[package]
name = "login_cli"
version = "0.1.0"
edition = "2024"

[dependencies]
clap = { version = "4.5.54", features = ["derive"] }
login_lib = { path = "../login_lib" }
```

Setup a *really minimal* main.rs just to see that it works:

```rust
fn main() {
    let mut manager = login_lib::UserManager::new();
    manager.add_user("bob".to_string(), "password".to_string(), login_lib::Permission::Normal);
    manager.save();
    println!("Added user bob");
}
```

Now we know that it works!

**Let me know when it's working for you - we'll move on.**