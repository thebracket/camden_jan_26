# Debugging

I personally use `println!` and `tracing` for debugging most of the time. But sometimes, you need a step-through debugger.

There are a few options for Rust debugging:
* [RustRover](https://www.jetbrains.com/rust/) - this is my personal favorite. It just works, and has great Rust support.
* [VS Code](https://code.visualstudio.com/) - with the Rust Analyzer extension you can get pretty good debugging support.
* [LLDB](https://lldb.llvm.org/) - the LLVM debugger, which works well with Rust since Rust uses LLVM as its backend.

I'll quickly show you how to set up debugging in RustRover and VS Code.

> The Rust step-through debugging story is not as great as other languages, but it's getting better all the time. If you can get by with `println!` and `tracing`, that's often the easiest path.