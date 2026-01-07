# User Struct and Enum

Your `lib.rs` file has an example function and unit test in there. Delete it all!

We want to define a `User` type and an enumeration for their role.

The user should have:
* A username
* A password
* A role

The role should be an enumeration with three variants:
* Admin
* Normal

Don't forget to derive serialize, deserialize AND debug!

**(Please don't skip ahead - we want to keep everyone together)**

![](../../images/ScrollTime.png)

Here's my version:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    password: String,
    pub user_type: Permission
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Permission {
    Normal, Admin
}
```
