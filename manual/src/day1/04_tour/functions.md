# Functions

You've seen the basic function syntax in Rust:

```rust
fn area_of_circle(radius: f64) -> f64 {
    3.14 * radius * radius
}

fn main() {
    let area = area_of_circle(5.0);
    println!("Area of circle with radius 5.0 is {}", area);
}
```

Nothing really surprising (although my friend Dave spent a week bemoaning all the parameters being the wrong away around).

Function pointers are also available in Rust, and you can pass them around like in C and C++:

```rust
fn area_square(side: f64) -> f64 {
    side * side
}

fn area_circle(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

fn main() {
    let mut area_fn: fn(f64) -> f64 = area_square;
    println!("Square area (side 5) = {}", area_fn(5.0));

    area_fn = area_circle;
    println!("Circle area (radius 5) = {}", area_fn(5.0));
}
```
