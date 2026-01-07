# Mutability and Borrowing

When you lend (borrow) something to a function, you're doing so immutably by default. It can't alter the thing you lent. Sometimes, you want to let the function change what you lent it.

```rust
fn do_something(s: &mut String) {
    s.push_str(" World"); // Modify the string by appending to it.
}

fn main() {
    let mut my_string = String::from("Hello, Rust!"); // Note: my_string must be mutable.
    do_something(&mut my_string); // Borrow it mutably by passing a mutable reference.
    println!("Back in main: {}", my_string); // Now modified!
}
```

Notice that you have to declare mutability:
* At the *variable declaration* (the variable can be changed).
* At the *call site* (you're lending it mutably).
* In the *function signature* (the function can change what it borrowed).

While that's an awful lot of "muts", it makes it very clear what's going on, and avoids accidental mutations.

While that's pedantic, it protects against library authors changing function signatures and breaking your code in surprising ways.

<table><tr><td>

```cpp
#include <string>
#include <iostream>

void do_something(std::string &s) {
    std::cout << s << "\n";
}

int main() {
    auto s = std::string("Hello World");
    do_something(s);
    std::cout << s << "\n";
    return 0;
}
```

</td><td>

```cpp
#include <string>
#include <iostream>

void do_something(std::string &s) {
    std::cout << s << "\n";
    s += " - I Am Evil!";
}

int main() {
    auto s = std::string("Hello World");
    do_something(s);
    std::cout << s << "\n";
    return 0;
}
```

</td></tr></table>