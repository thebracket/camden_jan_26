# Iterator Invalidation vs The Borrow Checker

This C++ is technically undefined behaviour, but it often "works" (until it doesn't):

```cpp
#include <vector>
#include <iostream>

int main() {
    std::vector<int> v = {1, 2, 3, 4, 5};

    for (auto it = v.begin(); it != v.end(); ++it) {
        if (*it % 2 == 0) {
            v.erase(it);  // iterator invalidated
        }
    }

    for (int x : v) {
        std::cout << x << " ";
    }
}
```

It *probably* prints `1 3 5`, but it might crash, or print garbage, or enter an infinite loop. This has been the cause of many subtle bugs in C++ programs.

Rust won't compile something similar:

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    for x in &v {
        if *x % 2 == 0 {
            v.remove(0); // try to mutate while iterating
        }
    }
}
```

The error "cannot borrow `v` as mutable because it is also borrowed as immutable" goes back to the "golden rule" of Rust: you can have either one mutable reference or any number of immutable references, but not both at the same time. So once you are iterating over `&v`, you cannot mutate `v`.

Instead, you should use `retain`, which is designed for this purpose:

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.retain(|&x| x % 2 != 0); // keep only odd numbers

    for x in &v {
        println!("{}", x);
    }
}
```

If you're used to iterating arrays and vectors, and mutating them during the iteration, you need to be especially careful in Rust. The borrow checker will prevent you from doing this unsafely, and sometimes you'll need to rethink your approach to avoid iterator invalidation issues/the borrow checker.