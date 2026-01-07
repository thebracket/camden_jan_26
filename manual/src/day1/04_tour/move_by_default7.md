# What is Borrowing, Anyway?

As you saw, on a simple level when you "borrow" you are doing the equivalent of C++'s "taking a reference" (a pointer that can't be null --- not always true, but we'll pretend, and is guaranteed to be valid).

In C++, references are syntactic sugar for pointers. You can think of Rust references the same way.

Rust has a much-feared feature called the "borrow checker" that enforces rules about how references can be used.

Before we get into concurrency, let's look at some of the benefits of the borrow checker.

## Use After Free

C++ lets you do this:

```cpp
#include <string>
#include <iostream>

void do_something(std::string * s) {
    std::cout << *s << "\n";
}

int main() {
    auto s = new std::string("Hello World");
    delete s;
    do_something(s);
    return 0;
}
```

Interestingly, on `cpp.sh` this prints "Hello World". On my Linux box, it prints an empty string.

Let's try this in Rust:

```rust
fn do_something(s: &String) {
    println!("{s}");
}

fn main() {
    let s = String::from("Hello, Rust!");
    drop(s); // Explicitly drop the string, freeing the memory.
    do_something(&s); // Attempt to use the reference after the string is dropped.
}
```

This won't compile:

```
   Compiling playground v0.0.1 (/playground)
error[E0382]: borrow of moved value: `s`
 --> src/main.rs:8:18
  |
6 |     let s = String::from("Hello, Rust!");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
7 |     drop(s); // Explicitly drop the string, freeing the memory.
  |          - value moved here
8 |     do_something(&s); // Attempt to use the reference after the string is dropped.
  |                  ^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
7 |     drop(s.clone()); // Explicitly drop the string, freeing the memory.
  |           ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `playground` (bin "playground") due to 1 previous error
```

Not the friendliest of messages, and the suggestion is very silly (clone a string so you can drop the clone?) - but the borrow checker has:

* Tracked *ownership* of the string.
* Noted that `drop(s)` *moved* the string out of `s` (into the void, and freed the memory).
* So "use after free" is a borrowing violation.

So the borrow checker is tracking *ownership* of every variable in your program - in this case, the string ceases to be owned by `main` after the `drop(s)` call.

> "Use after free" won't compile! It's one of the biggest sources of bugs in C and C++ programs, and Rust eliminates it at compile time.