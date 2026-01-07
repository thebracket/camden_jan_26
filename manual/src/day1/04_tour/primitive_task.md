# Task: Convert between types

To make sure that sticks, let's take 5 minutes for you guys to write a little code.

1. Create a new binary project with Cargo, called `primitives`.
2. In `main.rs`, create a variable of type `u16` and assign it the value `500`.
3. Create a variable of type `u32` and convert the `u16` variable into it using `into()`.
4. Print both variables.
5. Now, try to convert the `u32` variable back into a `u16` using `try_into()`. Use `unwrap()` to get the value.
6. Replace `unwrap()` with `expect("Conversion failed!")`, and run the program again. Then run it with an impossible conversion (e.g., assign `70000` to the `u32` variable before converting it back to `u16`).
7. Observe the panic message and understand why it happened - and why `expect` is nicer than `unwrap()` (and still not ideal).

When you're done, let me know! If you run into any issues, feel free to ask - that's why I'm here!