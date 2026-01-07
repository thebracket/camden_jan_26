# Seeing what Cargo is Actually Doing

If you're integrating into other build systems, it can be helpful to understand what
Cargo is actually doing under the hood when it builds your project.

You can see this by running Cargo with the `-v` (verbose) flag. For example:

```sh
cargo build -v
```

This gives you the full commands that Cargo is executing, including calls to `rustc` (the Rust compiler):

```
/home/herbert/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc --crate-name hello_world --edition=2024 day1/hello_world/src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=86 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=3a6b7df1ebcf70a7 -C extra-filename=-4058bf4c2499b9fe --out-dir /home/herbert/Documents/Ardan/l3h_jan_2026/code/target/debug/deps -C incremental=/home/herbert/Documents/Ardan/l3h_jan_2026/code/target/debug/incremental -L dependency=/home/herbert/Documents/Ardan/l3h_jan_2026/code/target/debug/deps
```

> Note that the exact commands may vary based on your system and project configuration.

You won't use this often. The Linux kernel executes all of its builds via calls to `rustc` inside its `Makefile` system. `CMake` has Rust support as well. Most Rust projects use Cargo.