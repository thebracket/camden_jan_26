# Functions & Closures Workshop

In this workshop, you'll practice writing functions and closures in Rust. Follow the tasks below to get hands-on experience.

1. Create a new Rust project using `cargo new`.
2. Write a function that accepts an integer parameter and returns its square.
3. Call the function and print the result, to make sure it works.
4. Next, create a function that takes a function pointer (to a function that takes an integer and returns an integer), and an integer parameter. Inside this function, call the passed-in function pointer with the integer parameter and return the result.
5. Test this higher-order function by passing in the square function you created earlier.
6. Now define a variable to hold `vec![1, 2, 3, 4, 5]`.
7. Use the `map` method with a closure to create a new vector containing the squares of the original vector's elements.
8. Print the new vector to verify the results.
9. Finally, create a closure that captures a variable from its surrounding scope. For example, define a variable `multiplier` and create a closure that multiplies its input by `multiplier`. Test this closure with different inputs and print the results.

![](../../images/ScrollTime.png)

Example program (`src/main.rs`):

```rust
fn square(x: i32) -> i32 {
    x * x
}

fn apply(func: fn(i32) -> i32, x: i32) -> i32 {
    func(x)
}

fn main() {
    // 2-3) Square a number with a function.
    let value = 12;
    let squared = square(value);
    println!("square({}) = {}", value, squared);

    // 4-5) Pass a function pointer to another function.
    let via_apply = apply(square, value);
    println!("apply(square, {}) = {}", value, via_apply);

    // 6-8) Use `map` with a closure to transform a vector.
    let values = vec![1, 2, 3, 4, 5];
    let squares: Vec<i32> = values.iter().map(|&n| square(n)).collect();
    println!("squares = {:?}", squares);

    // 9) Closure capturing from surrounding scope.
    let multiplier = 3;
    let multiply = |x: i32| x * multiplier;

    println!("multiply(2) = {}", multiply(2));
    println!("multiply(5) = {}", multiply(5));
}
```
