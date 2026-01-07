# Private vs Public

By default, all struct fields are private. You can make them public by adding the `pub` keyword before the field name.

```rust
struct Person {
    pub name: String,
    age: u32,
}
```

Within a single file, you can access both public and private fields. However, from other modules, only the public fields are accessible.

