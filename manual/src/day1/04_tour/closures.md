# Closures

Closures are basically lambdas in C++. They are anonymous functions you can save in a variable or pass as arguments to other functions. They can capture variables from their surrounding scope.

Here's a simple example of a closure that adds two numbers:

```rust
fn main() {
    let add = |a: i32, b: i32| -> i32 {
        a + b
    };
    let result = add(5, 3);
    println!("The sum is: {}", result);
}
```

Closures can also *capture* variables from their environment. Unlike C++, you don't get to specify a "capture list" (with method). Instead, if you use a variable from the surrounding scope, Rust automatically captures it by reference for you.

A simple example:

```rust
fn main() {
    let x = 10;
    let multiply_by_x = |y: i32| -> i32 {
        x * y
    };
    let result = multiply_by_x(5);
    println!("The result is: {}", result);
}
```

It's really common for closures to be used as arguments to other functions, especially for things like iterators. Here's an example using the `map` method on a vector:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squared_numbers: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squared numbers: {:?}", squared_numbers);
}
```

> `collect()` collects an iterator into a collection. C++ ranges have similar functionality.

Where this gets *messy* is that the closure can capture variables in three different ways: by reference, by mutable reference, or by moving ownership. Probably my biggest grumble is that I don't get to *specify* the list of captures like in C++.

Sometimes, you *want* to move ownership of a variable into the closure. You can do this with the `move` keyword:

```rust
fn main() {
    let s = String::from("Hello");
    let consume_string = move || {
        println!("{}", s);
    };
    consume_string();
    // println!("{}", s); // This would be an error because `s` has been moved.
}
```

> This is particularly prevalent in threaded code - we'll cover that later.