# Option 2: Borrow it

A much more idiomatic way to handle this is to *borrow* the string instead of taking ownership of it.

When you borrow something, you're just getting a reference to it. The original owner still owns it, and it's still valid after the function call.

```rust
fn do_something(s: &String) {
    println!("Doing something with: {}", s);
}

fn main() {
    let my_string = String::from("Hello, Rust!");
    do_something(&my_string); // Borrow it by passing a reference.
    println!("Back in main: {}", my_string); // Still valid!
}
```

Notice a few things here:

* In C++, you don't have to explicitly say "borrow this" on the caller side. In Rust, you *must* use the `&` operator to create a reference when calling the function. This both makes it very clear, and avoids the fun of someone changing a function signature and suddenly your performance explodes because you're copying huge structs instead of passing references.
* The function signature changed to accept a `&String` instead of a `String`. This means it takes a reference to a `String`, not ownership of the `String` itself.
    * Warning: you can also use `&str` here instead of `&String` - but let's not worry about that yet.

## Fun with C++

I've been bitten by this one in larger C++ projects a few times. Spot the difference!

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
    return 0;
}
```

</td><td>

```cpp
#include <string>
#include <iostream>

void do_something(std::string s) {
    std::cout << s << "\n";
}

int main() {
    auto s = std::string("Hello World");
    do_something(s);
    return 0;
}
```

</td></tr></table>

Both compile and run. And in the second version, the maintainer of the infamous "do_something" library changed the function signature to take a copy instead of a reference. Oops! Now every call to that function is making a copy of the string, which could be expensive.