# Task: Build a mini state machine

Here's a handy function:

```rust
pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
```

1. Create a new Rust project with `cargo new`.
2. Define a `Command` enum with variants for `Add`, `Subtract`, `Multiply`, `Divide`, and `Quit`. The operation variants should carry one `f64` value.
3. Implement an associated function `apply(&self, n: f64) -> Option<f64>` for the `Command` enum that applies the command to the given number and returns the result. If the command is `Quit`, it should return `None`.
4. Implement a constructor for the `Command` enum that gets a string from stdin "add", "subtract", "multiply", "divide", or "quit".
    * For the operation commands, it should also read a number from stdin. Hint: you can use `str::parse::<f64>()` to convert a string to an `f64`.
5. In the `main` function, start with an initial value of `0.0`.
6. Enter a loop where you:
    * Print the current value.
    * Read a command from stdin using your constructor.
    * If the command is `Quit`, break the loop.
    * Otherwise, apply the command to the current value and update it.
7. Test your program by entering various commands and numbers, ensuring it behaves as expected.

![](../../images/ScrollTime.png)

Here's a possible implementation of the task described:

> This is in `code/day1/enumerations_task`.

```rust
fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("read");
    input.trim().to_string()
}

enum Command {
    Add(f64),
    Subtract(f64),
    Multiply(f64),
    Divide(f64),
    Quit,
}

impl Command {
    fn read() -> Self {
        match read_line().as_str() {
            "add" => Self::Add(read_line().parse().unwrap()),
            "subtract" => Self::Subtract(read_line().parse().unwrap()),
            "multiply" => Self::Multiply(read_line().parse().unwrap()),
            "divide" => Self::Divide(read_line().parse().unwrap()),
            "quit" => Self::Quit,
            _ => Self::read(),
        }
    }

    fn apply(&self, n: f64) -> Option<f64> {
        match *self {
            Self::Add(v) => Some(n + v),
            Self::Subtract(v) => Some(n - v),
            Self::Multiply(v) => Some(n * v),
            Self::Divide(v) => Some(n / v),
            Self::Quit => None,
        }
    }
}

fn main() {
    let mut n = 0.0;
    loop {
        println!("{n}");
        let cmd = Command::read();
        n = match cmd.apply(n) {
            Some(next) => next,
            None => break,
        };
    }
}
```
