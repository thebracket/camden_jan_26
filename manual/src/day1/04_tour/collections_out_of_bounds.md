# Out-of-bounds Access

One of the more notorious ways to make C and C++ look bad is to access beyond the bounds of an array or vector:

```cpp
#include <vector>
#include <iostream>

int main() {
    std::vector<int> v = {1, 2, 3, 4, 5};

    for (int i=0; i<6; i++) {
        std::cout << v[i] << "\n";
    }
}
```

(You can of course use `.at()` in C++ to avoid this, check your bounds --- but far too many people don't.)

On `cpp.sh`, this code outputs:

```
1
2
3
4
5
6881
```

In Rust, this code would look like:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for i in 0..6 {
        println!("{}", v[i]);
    }
}
```

It still compiles, but when you run it, you get:

```
thread 'main' (13) panicked at src/main.rs:4:25:
index out of bounds: the len is 5 but the index is 5
```

And that's a *good* thing. Instead of returning whatever happens to be next to the vector in memory, Rust checks the bounds of the vector at runtime and panics if you try to access an invalid index.

Of course, it's better not to panic at all - so you can use the `get` method, which returns an `Option`:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for i in 0..6 {
        match v.get(i) {
            Some(value) => println!("{}", value),
            None => println!("Index {} is out of bounds", i),
        }
    }
}
```

> Strousrup famously quipped that Rust is just like C++ with opposite defaults. Here, Rust defaults to safety (checking bounds) while C++ defaults to performance (no bounds checking). Rust has `get_unchecked` to skip the bounds-check in `unsafe` code (as well as pointer access) - but it's not a good idea.
