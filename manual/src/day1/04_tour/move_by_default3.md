# Rust moved the string.

What happened there? `my_string` was *moved* into the function `do_something` (and automatically cleaned up when it left scope at the end of the function). It no longer exists in the main function's scope.

What you actually wrote is equivalent to the following C++ code:

```cpp
#include <string>
#include <iostream>

void do_something(std::string s) {
    std::cout << s << "\n";
}

int main() {
    auto s = std::string("Hello World");
    do_something(std::move(s));
    std::cout << s << "\n";
    return 0;
}
```

> Pop Quiz: Who knows what `std::move` in C++ actually does?

![](../../images/ScrollTime.png)

`std::move` is a *cast* that enables moving a variable (via the move constructor), and leaves the original variable in a "valid but unspecified state". It might be an empty string, it might be surprising. You can't safely use it anymore, because you don't know what it is. It does remain sufficiently valid that the destructor can run without crashing.

Rust *explicitly* tracks ownership - so when you *move out* of a variable, you can't use it anymore. This is a big part of how Rust guarantees memory safety without a garbage collector.

So, no more "use after move" bugs (which tend to be quite subtle) - but you have to think about ownership before you can really do *anything*.