# Enumerations can have functions, too!

You can also define functions on enumerations, just like you can with structs. Here's an example:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 55,
        }
    }
}

fn main() {
    let light = TrafficLight::Red;
    println!("Red light duration: {} seconds", light.duration());
}
```

You can also use associated functions, handy for constructor-like behavior:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn new_red() -> Self {
        TrafficLight::Red
    }
}
```