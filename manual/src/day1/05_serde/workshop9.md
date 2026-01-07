# Delete a User

Now add a command that accepts a username as a parameter and deletes that user from the system.

![](../../images/ScrollTime.png)

```rust
        Commands::Delete { username } => {
            user_manager.delete_user(&username);
            user_manager.save();
            println!("User deleted successfully.");
        }
```