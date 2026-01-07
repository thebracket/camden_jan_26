# Option 1: Move it right back (return it)

I think this is a pretty ugly option, but it's common in some functional programming languages.

You could simply move the string into the function, and then move it back out again:

```rust
fn do_something(s: String) -> String {
    println!("Doing something with: {}", s);
    s // Move it back out. Notice no "return" or semicolon. Rust returns the last expression.
}

fn main() {
    let my_string = String::from("Hello, Rust!");
    let my_string = do_something(my_string); // Move it back in.
    println!("Back in main: {}", my_string);
}
```

I *like* this option when I'm explicitly chaining a bunch of operations together, but otherwise it's a bit clunky.