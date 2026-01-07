# Task: no_std library called from a normal binary

1. Create a new `no_std` library crate (`cargo new my_no_std_math --lib`).
2. Add `#![no_std]` to `lib.rs`.
3. Implement a few simple arithmetic functions, such as `add`, `sub`, and `mul`.
4. Create a normal binary crate (`cargo new my_no_std_consumer`).
5. Add a path dependency from the binary to the library in `Cargo.toml`.
6. Call the library functions from the binary, print the results, and verify it works.

![](../../images/ScrollTime.png)

> See `code/day3/no_std_math` and `code/day3/no_std_math_consumer` for one possible implementation.

