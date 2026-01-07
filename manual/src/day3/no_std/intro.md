# Life with No STD

Most Rust programs use the standard library (`std`). That's where you get things like:

* `println!`
* Files, sockets, and threads
* Collections like `Vec` (via the allocator)

Sometimes you don't have `std`:

* Embedded systems
* Kernels / bare metal
* Tiny programs where you want full control of the runtime
* Highly constrained environments

Even without `std`, you still have `core`. `core` contains the language fundamentals: arithmetic, iterators, `Option`, `Result`, slices, and so on. It's the part of Rust that doesn't require an operating system.

If you also have an allocator, you can use `alloc` (which gives you things like `Vec` and `String`). We'll demo that, too.

## Building and toolchains

The `no_std` *library* example in this section builds on stable Rust.

The `no_std` *binary* demos are different: on stable Rust you'll usually run into:

`unwinding panics are not supported without std`

That's because the precompiled `core` you have installed is typically built with unwind support. For these demos, we rebuild `core`/`alloc` with an aborting panic strategy using nightly `-Z build-std`.

Each demo directory includes a `build.sh` that runs the right command.
