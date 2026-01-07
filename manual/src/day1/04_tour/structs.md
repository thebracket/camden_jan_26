# Structs

You can define a struct as an empty type:

```rust
struct UnitStruct;
```

> This is more useful than it sounds, it shows up in generics as "marker" types a lot.

You can also define a struct as a "tuple struct", which is basically a named tuple:

```rust
struct Color(u8, u8, u8);
```

> Note that you have all of the problems with tuples here - it's not clear what each field means. You can access the fields with `.0`, `.1`, etc, or destructure them like tuples.

Most commonly, a struct is defined with named fields:

```rust
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Name: {}, Age: {}", alice.name, alice.age);
}
```