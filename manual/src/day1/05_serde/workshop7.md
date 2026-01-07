# CLAP Skeleton

CLAP is "Command Line Argument Parser", a popular crate for building CLI applications in Rust. It's used by a number of the Rust tools you may already be familiar with, such as `cargo` itself.

*Hint: make sure you added the "derive" feature when adding CLAP as a dependency!*

Let's set up a basic skeleton for our CLI application using CLAP. We'll define the commands and arguments we want to support.

Replace the contents of `src/main.rs` with the following code:

```rust
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List all users
    List,
}

fn main() {
    let cli = Cli::parse();
    let mut user_manager = login_lib::UserManager::new();

    match cli.command {
        Commands::List => {
            for user in user_manager.get_all_users() {
                println!("Username: {}, Role: {:?}", user.username, user.user_type);
            }
        }
    }
}
```

And try it. I actually think its a little scary, because `clap` uses procedural macros to read the token stream for your program - including the comments!

```bash
cargo run
```

Will show you a help message - including text derived from our comments!

```
Usage: login_cli <COMMAND>

Commands:
  list    List all users
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

**Let me know when it's working for you - we'll move on.**