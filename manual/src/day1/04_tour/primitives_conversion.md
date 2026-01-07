# Conversion and Narrowing

In the `/code/day1/narrowing` directory, there's a C program:

```c
#include <stdio.h>
#include <stdint.h>

int main() {
    int32_t x = 257;
    int8_t y = x;
    printf("%d, %d\n", x, y);
    return 0;
}
```

This outputs: `257, 1`. Unless you enabled warnings, used a linter or similar --- it compiles just fine. Notice that 257 doesn't fit int `int8_t`, so it wraps around and gets truncated to `1`. This is called *narrowing conversion*. It's legal in C, and has been the cause of *many* bugs over the years.

So let's try that in Rust:

```rust
fn main() {
    let x: i32 = 257;
    let y: i8 = x;
    println!("{}, {}", x, y);
}
```

It doesn't compile. Type conversions in Rust are *explicit*. And there are three ways to do it!

> This is a deliberately simple example. You'd never do this! However, once your code-base gets huge, it's *really easy* to do it by accident!