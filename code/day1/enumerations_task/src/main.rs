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