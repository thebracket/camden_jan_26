# For is really "for each"

`For` loops in Rust are really "for each" loops. They iterate over *iterators*.

When you type `0..10`, you're actually creating a `Range`. The range type implements an iterator that produces values from 0 up to (but not including) 10.

In C++20, you might write:

```cpp
#include <iostream>
#include <ranges> // Required for std::ranges::iota_view and views::iota

int main() {
    // Iterate from 0 up to (but not including) 10
    for (int i : std::views::iota(0, 10)) {
        std::cout << i << " ";
    }
    // Output: 0 1 2 3 4 5 6 7 8 9

    std::cout << std::endl;
}
```

In Rust, the equivalent code is:

```rust
fn main() {
    for i in 0..10 {
        print!("{} ", i);
    }
    // Output: 0 1 2 3 4 5 6 7 8 9

    println!();
}
```

## Iterators vs Destructive Iterators

> We'll look at vectors in detail in a bit, but this is a good time to introduce the concept of ownership with iterators.

This tripped me up *hard* when I first used Rust:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for i in v {
        println!("The value is: {}", i);
    }
    println!("Vector length is: {}", v.len());
}
```

It doesn't compile!

And then I remember - Rust is *move by default*, and that applies to for loops too! When the for loop runs, it *moves* each element out of the vector `v`. After the loop, `v` is no longer valid, so trying to access its length is a compile-time error.

> Honestly, I don't like this default!

To fix this, you can iterate over *references* to the elements in the vector:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for i in &v { // Note the '&' here
        println!("The value is: {}", i);
    }
    println!("Vector length is: {}", v.len()); // Now this works!
}
```

OR, you can use the long-form (the & is syntax sugar):

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for i in v.iter() { // Using the iter() method
        println!("The value is: {}", i);
    }
    println!("Vector length is: {}", v.len()); // Now this works!
}
```

> Curiously, the version without the `&` is also syntax sugar - it actually calls the `into_iter()` method on the vector, which consumes the vector and yields its elements by value.