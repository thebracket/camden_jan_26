# Enumerations with Data

Enumerations can also carry data. (They are just like an std::variant in C++ or a tagged union in C, but with better ergonomics). You can also define them in "tuple style" or "struct style". Here's an example:

```rust
enum Shape {
    Circle(f64),               // tuple style
    Rectangle { width: f64, height: f64 }, // struct style
}
```

And then you can use `match` to both match the pattern and destructure the data:

```rust
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
}

fn main() {
    //let s = Shape::Circle(5.0);
    let s = Shape::Rectangle { width: 10.0, height: 5.0 };
    match s {
        Shape::Circle(radius) => println!("Circle with radius {}", radius),
        Shape::Rectangle { width, height } => {
            println!("Rectangle with width {} and height {}", width, height)
        }
    }
}
```

> How big is the enum? It is the size of its largest member.

