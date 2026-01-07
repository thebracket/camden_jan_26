# Overflow/Underflow

Who'se run into something like this before? (It's in the `code/day1/overflow` directory.)

```c
#include <stdint.h>
#include <stdio.h>

int main() {
    int8_t n = 120;
    for (int i=0; i<10; i++) {
        n += 1;
        printf("%d\n", n);
    }
    return 0;
}
```

This will output:

```
121
122
123
124
125
126
127
-128
-127
-126
```

Hopefully, a lot of you are saying "of course it does". It's actually been a significant source of errors over the years when you *don't* do it deliberately.

Let's try that in Rust (see `code/day1/overflow_rs`):

```rust
fn main() {
    let mut n: i8 = 120;
    for _ in 0..10 {
        n += 1;
        println!("{n}");
    }
}
```

If you run this with `cargo run`, you'll see:

```
121
122
123
124
125
126
127

thread 'main' (2509171) panicked at day1/overflow_rs/src/main.rs:4:9:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Interestingly, if you run `cargo run --release`, you'll see:

```
121
122
123
124
125
126
127
-128
-127
-126
```

Rust checks for overflow at runtime, *in debug builds*. There's a a runtime cost to the check, so it's disabled in release builds.

> I once completely forgot to run debug builds on a game I was developing, and it took a while to notice that world generation was oddly wonky because of overflows! The moral: run debug builds sometimes.