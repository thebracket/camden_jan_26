# Tuples & Destructuring

Tuples are really handy. You can group multiple values into a single compound value. They can hold values of different types. Here's a simple example:

```rust
fn main() {
    let person: (&str, i32, f64) = ("Alice", 30, 5.5);
    println!("Name: {}, Age: {}, Height: {}", person.0, person.1, person.2);
}
```

You can also destructure tuples to extract their values into separate variables. Here's how you can do that:

```rust
fn main() {
    let person: (&str, i32, f64) = ("Bob", 25, 6.0);
    
    // Destructuring the tuple
    let (name, age, height) = person;
    
    println!("Name: {}, Age: {}, Height: {}", name, age, height);
}
```

Tuples are often used when you want to return multiple values from a function. Here's an example of a function that returns a tuple:

```rust
fn calculate_stats(numbers: &[i32]) -> (i32, i32, f64) {
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len() as i32;
    let average = sum as f64 / count as f64;
    (sum, count, average)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let (sum, count, average) = calculate_stats(&numbers);
    
    println!("Sum: {}, Count: {}, Average: {}", sum, count, average);
}
```

## Beware of Tuple Abuse

Tuples are really tempting, but they don't convey a lot of meaning. If you find yourself using tuples with more than 3-4 elements, consider defining a struct instead. Structs give names to each field, making your code more readable and maintainable.

> I once saw an automated code generator that produced functions returning 128 tuple elements. Please, don't do that.

> The Bevy game engine actually supports tuples of generics, but only up to 16 elements. So they defined more generics to allow 16*16 = 256 element tuples. Yikes. If you need 256 parameters, you're probably doing something wrong.