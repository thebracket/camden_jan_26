# no_std "Hello World" using libc and printf

> See `code/day3/no_std_printf` for the code.

This example is `no_std`, but still runs on your normal operating system. We're not using Rust's I/O at all; we're just calling C's `printf`.

At a high level:

* We opt out of the standard library with `#![no_std]`.
* We opt out of Rust's `main` wrapper with `#![no_main]`.
* We export a C-compatible `main` function.
* We call into libc.

If you open `src/main.rs`, it's basically just `printf` with a null-terminated string.

## Building

On stable Rust, `no_std` binaries can be a bit awkward because the precompiled `core` is usually built for unwinding panics. The simplest way to build these examples is to use nightly and rebuild the standard crates you need:

```bash
cargo +nightly build -Z build-std=core -Z build-std-features=panic_immediate_abort
```

Or just run the helper script:

```bash
./build.sh
```
