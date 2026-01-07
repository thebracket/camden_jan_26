# Logical Macros (Declarative)

Declarative macros are the `macro_rules!` system. They let you match token patterns and expand them into code at compile time.

The "why" is usually one of:

* Reduce repeated boilerplate.
* Provide a tiny bit of syntactic sugar when normal functions can't express what you want.

## When Rust Syntax isn't Enough

The canonical examples are things like `println!`, `vec!`, and `format!`. You can't write those as normal functions because they accept a variable number of arguments, and the compiler needs to type-check the result after expansion.

## Interactive walkthrough: a HashMap builder macro

> See `code/day3/macros_demo` for the code.

Here's a small macro that builds a `HashMap` with a familiar "map literal" feel:

```rust
use std::collections::HashMap;

macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = HashMap::new();
        $( map.insert($key, $value); )*
        map
    }};
}

fn main() {
    let ports = hashmap! {
        "http" => 80,
        "https" => 443,
    };

    println!("{ports:?}");
}
```

This is doing pattern matching on tokens. The `$( ... ),*` syntax says "repeat the pattern zero or more times, separated by commas", and `$(,)?` makes the trailing comma optional.

## Downside: Don't make LISP

It's *really* tempting to take the macros and turn Rust into something not-very-Rust-like at all. This is the LISP way (step 1 of writing a large LISP program is to build a language that suits your problem)---and it's not a great idea. You'll have a terrible time onboarding newcomers to your "here's the language we built, and here's the program to work on" approach.

Use macros where you *need* them to extend the language, or make something easier.
