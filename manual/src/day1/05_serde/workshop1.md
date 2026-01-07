# WS: Login Library and Consumer

We're going to make a simple login library that can be used by a CLI application.

Let's start by making a new library project:

```bash
cargo new login_lib --lib
```

We're going to want Serde and Serde_json, so add these to `login_lib/Cargo.toml`:

```bash
cargo add serde -F derive
cargo add serde_json
```

Let me know when that's complete. 

**(Please don't skip ahead - we want to keep everyone together)**