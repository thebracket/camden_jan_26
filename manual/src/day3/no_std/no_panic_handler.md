# Same thing, but no custom panic handler

> See `code/day3/no_std_printf_no_panic_handler` for the code.

In `no_std`, you normally have to provide a panic handler. That's not Rust being annoying; it's Rust being explicit about what happens when something panics.

If you're happy with a "halt/abort" strategy, you can pull in a tiny helper crate that provides a panic handler for you. Then you don't have to write one in your code.

This example does exactly the same `printf` call as before, but delegates the panic handler to a crate.

## Building

```bash
cargo +nightly build -Z build-std=core -Z build-std-features=panic_immediate_abort
```

Or:

```bash
./build.sh
```
