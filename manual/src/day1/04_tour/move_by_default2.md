# Send a string to a function and then try to re-use it.

Let's pull up a keyboard and try this out.

Any reasonable language would let us do this!

```rust
fn do_something(s: String) {
    println!("Doing something with: {}", s);
}

fn main() {
    let my_string = String::from("Hello, Rust!");
    do_something(my_string);
    println!("Back in main: {}", my_string);
}
```

It doesn't compile! The error message is a bit long, too.

```
   Compiling playground v0.0.1 (/playground)
error[E0382]: borrow of moved value: `my_string`
 --> src/main.rs:8:34
  |
6 |     let my_string = String::from("Hello, Rust!");
  |         --------- move occurs because `my_string` has type `String`, which does not implement the `Copy` trait
7 |     do_something(my_string);
  |                  --------- value moved here
8 |     println!("Back in main: {}", my_string);
  |                                  ^^^^^^^^^ value borrowed here after move
  |
note: consider changing this parameter type in function `do_something` to borrow instead if owning the value isn't necessary
 --> src/main.rs:1:20
  |
1 | fn do_something(s: String) {
  |    ------------    ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
7 |     do_something(my_string.clone());
  |                           ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `playground` (bin "playground") due to 1 previous error
```

> Obviously, this is time to throw the keyboard out of the window.