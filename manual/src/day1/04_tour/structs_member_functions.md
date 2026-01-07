# Member Functions

You can also define member functions that operate on an instance of a struct. These functions have access to the instance's data and are defined using the `impl` keyword, similar to associated functions.

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    let rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
}
```

Where people typically become confused is the `&self` parameter. It's similar to "this" or "self" in other languages - referring to this instance of the struct. However, it does't *have* to be a reference!

* `self` - takes ownership of the instance
* `&self` - takes a reference to the instance (does not take ownership)
* `&mut self` - takes a mutable reference to the instance (allows modifying the instance)
* `mut self` - takes ownership of the instance and allows modifying it

`&self` and `&mut self` are the most common forms you'll see. For the "builder pattern", you'll often see `self` used to take ownership and return a modified instance (avoiding cloning and ownership issues):

```rust
struct Builder {
    value: i32,
}

impl Builder {
    fn new() -> Builder {
        Builder { value: 0 }
    }

    fn set_value(mut self, value: i32) -> Builder {
        self.value = value;
        self
    }

    fn build(self) -> i32 {
        self.value
    }
}
```