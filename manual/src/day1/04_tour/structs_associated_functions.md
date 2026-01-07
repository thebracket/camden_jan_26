# Associated Functions

Associated functions use a type (struct or enum) as a namespace, and don't have access to an instance of that type. They are defined using the `impl` keyword.

```rust
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(radius: f64) -> f64 {
        std::f64::consts::PI * radius * radius
    }
}

fn main() {
    let radius = 5.0;
    let area = Circle::area(radius);
    println!("Area of circle with radius {}: {}", radius, area);
}
```

This is handy for utility functions that are related to a type but don't need to operate on an instance of that type.

## Constructors

You're probably used to C++ constructors:

```cpp
class Point {
public:
    Point(int x, int y) : x_(x), y_(y) {}
private:
    int x_;
    int y_;
};
```

In fact, you may be used to writing *five* different kinds of constructors (default, copy, move, parameterized, etc). Rust simplifies this by using associated functions as constructors.

In Rust, constructors are *nothing special at all*. They are just an associated function that returns an instance of the struct:

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn main() {
    let p = Point::new(10, 20);
    println!("Point: ({}, {})", p.x, p.y);
}
```

> Even the name `new` doesn't mean anything, it's just a convention. You can name your constructor function anything you want.

Under the hood, this actually simplifies things - but points to a requirement. Rust structs must be copyable with a `memcpy`, or movable.

## Destructors

Rust auto-implements the `Drop` trait (as a destructor) when you define a struct. When your struct goes out of scope, Rust automatically calls the `drop` function to clean up resources. This is transitive: if your struct contains other structs, containers, etc., their `drop` functions are called automatically as well.

> Rust doesn't guaranty that it won't leak memory, but RAII and ownership make it much less likely.

You can also implement the `Drop` trait manually if you need custom cleanup logic:

```rust
struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Releasing resource: {}", self.name);
    }
}

fn main() {
    let res = Resource {
        name: String::from("MyResource"),
    };
    println!("Using resource: {}", res.name);
} // `drop` is called automatically here
```

> The takeaway here is that Rust structures are a lot like structs in C++, but constructors are just associated functions, and destructors are implemented via the `Drop` trait. And you usually don't need to write a destructor.
