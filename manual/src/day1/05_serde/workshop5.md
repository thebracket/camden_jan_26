# Save and Load Functions

This one is a gimme - because we haven't covered much IO. Let's replace the constructor for the `UserManager` and add two functions:

```rust
impl UserManager {
    pub fn new() -> UserManager {
        let path = Path::new("users.json");
        if path.exists() {
            Self::load()
        } else {
            UserManager { users: HashMap::new() }
        }
    }

    fn load() -> UserManager {
        let raw = read_to_string("users.json").expect("Unable to read file");
        let deserialized = serde_json::from_str(&raw).expect("Unable to deserialize");
        deserialized
    }

    pub fn save(&self) {
        let json = serde_json::to_string_pretty(&self).expect("Unable to serialize myself");
        std::fs::write("users.json", &json).expect("Unable to write file");
    }

    // Etc
```

> Note that this matches the code in `code/day1/login_lib` now.