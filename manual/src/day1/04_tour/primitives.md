# Types - Primitives

In C (and C++), you may be used to types such as `bool`, `short`, `signed short`, `int`, `long`, `long long`, `float`, `double` or even `long double`. The exact size of these varies by platform (although modern chips outside of the embedded world have largely standardized on meanings). Many places have adopted `stdint.h` to have more precise definitions `int8_t`, `uint32_t`, etc.

> My favourite C type is `unsigned long long int`. `uint64_t` is a little easier to type.

Rust specifies the precision in types:

* `u8`, `u16`, `u32`, `u64`, `u128`
* `i8`, `i16`, `i32`, `i64`, `i128`
* `f32`, `f64`

There's also `usize` and `isize` (pointer-sized unsigned and signed integers, respectively).

## Specifying Types

If you ever want to start a fight in a C++ shop, just suggest that everyone use `auto`:

```cpp
#include <iostream>

int main() {
    auto x = 3;
    std::cout << x << "\n";
    return 0;
}
```

Rust is actually pretty similar to this by default:

```rust
fn main() {
    let x = 3;
    println!("{}", x);
}
```

Rust is inferring `x` to be an `i32`, unless you pass it to something that requires a type - in which case it will try to use that type.

A lot of the time, you don't need to specify types. However, if you're doing something where the type matters, you can (and should) specify it:

```rust
fn main() {
    let x: u8 = 3;
    println!("{}", x);
}
```