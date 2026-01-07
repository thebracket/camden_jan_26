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
    /// Add a new user
    /// Arguments: username, password, role (Normal/Admin)
    Add {username: String, password: String, role: String},
    /// Delete a user
    /// Arguments: username
    Delete {username: String},
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
        Commands::Add { username, password, role } => {
            let permission = match role.as_str() {
                "Admin" => login_lib::Permission::Admin,
                _ => login_lib::Permission::Normal,
            };
            user_manager.add_user(username, password, permission);
            user_manager.save();
            println!("User added successfully.");
        }
        Commands::Delete { username } => {
            user_manager.delete_user(&username);
            user_manager.save();
            println!("User deleted successfully.");
        }
    }
}