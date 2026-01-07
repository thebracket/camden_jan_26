# Unit Tests for UserManager

Handy hint: in VSCode and RustRover, you can type `tmod<tab>` and get the test boilerplate.

Otherwise, it looks like this:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_test() {
        assert_eq!(2 + 2, 4);        
    }
}
```

Your goals:

* Add a unit test to test the `authenticate` function.
* Add a unit test that serializes the whole `UserManager` after adding a user or two, and then deserializes it - and ensures the data is the same before and after.

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
        UserManager { users: HashMap::new()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_serialize_deserialize() {
        let mut manager = UserManager::new();
        manager.add_user("alice".to_string(), "password123".to_string(), Permission::Admin);
        manager.add_user("bob".to_string(), "securepass".to_string(), Permission::Normal);
        let serialized = serde_json::to_string(&manager).expect("Unable to serialize");
        let deserialized: UserManager = serde_json::from_str(&serialized).expect("Unable to deserialize");
        assert_eq!(manager.users.len(), deserialized.users.len());
        for (username, user) in &manager.users {
            let deserialized_user = deserialized.users.get(username).expect("User not found");
            assert_eq!(user.username, deserialized_user.username);
            assert_eq!(user.password, deserialized_user.password);
            assert_eq!(matches!(user.user_type, Permission::Admin), matches!(deserialized_user.user_type, Permission::Admin));
        }
    }

    #[test]
    fn test_authentication() {
        let mut manager = UserManager::new();
        manager.add_user("charlie".to_string(), "mypassword".to_string(), Permission::Normal);
        let user = manager.authenticate("charlie", "mypassword");
        assert!(user.is_some());
        let user = manager.authenticate("charlie", "wrongpassword");
        assert!(user.is_none());
        let user = manager.authenticate("nonexistent", "nopassword");
        assert!(user.is_none());
    }    
}
```