# println! - there's nothing wrong with it

I just want to say that `println!` debugging is *not* a bad thing. It's great, I use it all the time for quick debugging. By the time your application reaches production, you should have a more robust logging solution in place, but for development and debugging, `println!` is perfectly fine.

It's worth noting that `println!` uses another macro called `format!` under the hood, which is used to format the string before printing it. This means you can use all the formatting features of `format!` with `println!`.

`format!` has a *lot* of features. The full documentation is [here](https://doc.rust-lang.org/std/fmt/index.html).

Most of the logging frameworks *also* use `format!`!

## The Hidden Mutex

One thing to be aware of is that `println!` uses a global lock (a mutex) to ensure that output from multiple threads doesn't get mixed together. This means that if you have a lot of threads all trying to print at the same time, they will be blocked waiting for the mutex. This can lead to performance issues in highly concurrent applications. It's also why `println` output is often the first suspect in performance investigations that involve printing a lot of output!

You can gain exclusive access to the standard output by locking it yourself:

```rust
use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock(); // Lock the stdout

    writeln!(handle, "This is a locked println!").unwrap();
}
```

Note that the `lock` uses an RAII pattern, so the lock will be released when the `handle` goes out of scope. Like any other lock, don't hold onto it for too long!