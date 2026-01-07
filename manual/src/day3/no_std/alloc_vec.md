# Using alloc: Vec in no_std

> See `code/day3/no_std_alloc_vec` for the code.

If you have an allocator available, you can use the `alloc` crate. That enables heap-backed types like:

* `Vec<T>`
* `String`
* `Box<T>`

This example sets up a global allocator, creates a `Vec`, pushes some data into it, and prints a tiny summary with `printf`.

## Building

This one needs `alloc` as well as `core`:

```bash
cargo +nightly build -Z build-std=core,alloc -Z build-std-features=panic_immediate_abort
```

Or:

```bash
./build.sh
```
