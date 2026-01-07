# Add User Command

Now that we have our CLAP skeleton set up, let's add a command to add a new user to our system.

Here's the enumeration with an "add" command added:

```rust
#[derive(Debug, Subcommand)]
enum Commands {
    /// List all users
    List,
    /// Add a new user
    /// Arguments: username, password, role (Normal/Admin)
    Add {username: String, password: String, role: String},
}
```

Go ahead and extend the `match` statement in `main` to handle the new command, rmembering to save the user after adding them.

![](../../images/ScrollTime.png)

```rust
        Commands::Add { username, password, role } => {
            let permission = match role.as_str() {
                "Admin" => login_lib::Permission::Admin,
                _ => login_lib::Permission::Normal,
            };
            user_manager.add_user(username, password, permission);
            user_manager.save();
            println!("User added successfully.");
        }
```
