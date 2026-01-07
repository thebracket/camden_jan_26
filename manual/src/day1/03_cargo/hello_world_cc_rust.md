# Hello World C/C++ vs Rust - very similar. CMake is "fun".

So let's do a quick compare and contrast.

## Rust

Rust Hello World:

`Cargo.toml`

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2024"

[dependencies]
```

```rust
fn main() {
    println!("Hello, world!");
}
```

And you can build and run with:

```bash
cargo run
```

## C/C++

> See `day1/hello_world_c` for the code.

<table><tr><td>
A basic C version:

```c
#include <stdio.h>

int main() {
    printf("Hello World\n");
    return 0;
}
```
</td><td>
Or in C++:

```cpp
#include <iostream>

int main() {
    std::cout << "Hello World\n";
    return 0;
}
```
</td></tr></table>

And you can build it with something like this:

```bash
#!/bin/bash
cc -o hi_c hello.c
c++ -o hi_cpp hello.cpp
```

Or this (Makefile):

```makefile
CC ?= cc
CXX ?= c++

CFLAGS ?= -O2 -Wall -Wextra -pedantic -std=c11
CXXFLAGS ?= -O2 -Wall -Wextra -pedantic -std=c++17

.PHONY: all clean

all: hello_c hello_cpp

hello_c: hello.c
	$(CC) $(CFLAGS) -o $@ $<

hello_cpp: hello.cpp
	$(CXX) $(CXXFLAGS) -o $@ $<

clean:
	rm -f hello_c hello_cpp
```

Or even this (CMake):

```cmake
cmake_minimum_required(VERSION 3.16)

project(hello_world_c LANGUAGES C CXX)

add_executable(hello_c hello.c)
target_compile_features(hello_c PRIVATE c_std_11)
target_compile_options(hello_c PRIVATE -Wall -Wextra -pedantic)

add_executable(hello_cpp hello.cpp)
target_compile_features(hello_cpp PRIVATE cxx_std_17)
target_compile_options(hello_cpp PRIVATE -Wall -Wextra -pedantic)
```

You might even have Bazel, Meson, SCons, or some other build system.

## Differences

### Build System

Rust encourages you to use `Cargo`. C and C++ have *many* different build systems. This becomes particularly pronounced when you want dependencies --- you might need Conan, VCPKG, or something else to manage those for you. Rust bakes it in.

### Language Features

Compare:

<table><tr><td>

**Rust**

```rust
fn main() {
    println!("Hello, world!");
}
```

</td><td>

**C**

```c
#include <iostream>

int main() {
    std::cout << "Hello World\n";
    return 0;
}
```
</td></tr></table>

* The C version imports `iostream`. Rust is importing its `std` library implicitly, which includes some automatic features like `println!`.
* Rust doesn't support functions with variable number of arguments (like `printf` in C). Instead, it uses macros (the `!` indicates a macro) that can handle variable arguments in a type-safe way.
* Rust's `main` function has no return type specified. The Rust program returns status code 0 implicitly. If you want to exit with a different status code, you can use `std::process::exit(code)`.

### Binary Size

On my Linux box, `hello_world` from Rust is 3.8 Mb! The C and C++ versions are about 16 kb each.

This is because Rust is statically linked, and the entire standard library is included in the executable. C and C++ are dynamically linked by default, and are dynamically linking to `libc` and `libstdc++` respectively. So the extra weight is there, but not in the binary.

> You can make Rust "hello_world" pretty tiny with some effort.
