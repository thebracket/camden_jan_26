# For

For loops don't work *anything* like they do in C and C++.

You're probably used to:

```c
#include <stdio.h>

int main() {
    for (int i = 0; i < 10; i++) {
        printf("The value of i is: %d\n", i);
    }
    return 0;
}
```

Equivalent *functionality* in Rust would be:

```rust
fn main() {
    for i in 0..10 {
        println!("The value of i is: {}", i);
    }
}
```

But it's not the same. There's no customizing the step action, and no end condition.