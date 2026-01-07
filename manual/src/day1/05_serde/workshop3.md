# User Manager Struct

Now we want to create a `UserManager` struct that will hold a list of users and provide methods to add, remove, and list users. Let's also include an `authenticate` function to pretend we're actually verifying user credentials.

We want:

* The `UserManager` type to contain a `HashMap<String, User>` - keyed on username (username lookup is the most common operation).
* A method `new` that creates an empty `UserManager`.
* A method `add_user` that takes a username, password, and role, and adds a new user to the manager.
* A method `delete_user` that takes a username and removes that user from the manager.
* A method `get_all_users` that returns a list of all users (vector of references to users).
* A method `authenticate` that takes a username and password, and returns an `Option<&User>` - `Some(&User)` if the credentials match, or `None` if they don't.

**(Please don't skip ahead - we want to keep everyone together)**

![](../../images/ScrollTime.png)

```rust
use std::{collections::HashMap, fs::read_to_string, path::Path};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    password: String,
    pub user_type: Permission
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Permission {
    Normal, Admin
}

#[derive(Serialize, Deserialize)]

pub struct UserManager {
    users: HashMap<String, User>,
}

impl UserManager {
    pub fn new() -> UserManager {
        UserManager { users: HashMap::new() }
    }

    pub fn add_user(&mut self, username: String, password: String, role: Permission) {
        self.users.insert(username.clone(), User {
            username,
            password,
            user_type: role,
        });
    }

    pub fn delete_user(&mut self, username: &str) {
        self.users.remove(username);
    }

    pub fn get_all_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Option<&User> {
        match self.users.get(username) {
            Some(user) if user.password == password => Some(user),
            _ => None,
        }
    }
}
```