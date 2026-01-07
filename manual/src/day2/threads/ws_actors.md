# WS: Actors

The "actor" model was popularized in Erlang (but dates all the way to the initial invention of Object Oriented Programming - objects were supposed to communicate with one another with messages).

The actor model is popular in Rust, because it allows you to isolate state inside of an "actor" (which is just a struct with some state and a thread), and have that actor communicate with other actors via message passing (channels). It avoids shared mutable state, and can avoid locking. It can also make reasoning about concurrency easier.

Let's make a simple actor that holds a counter, and can increment it or get its value.

1. Create a Rust project with `cargo new`.
2. Create a new module `actor.rs`.
    * Create a `Command` enum that has two variants: `Increment` and `GetValue`.
    * Create a `start_actor` function. It creates an MPSC channel (command type). It spawns a thread and moves the receiver into the thread.
    * The function returns the sender.
    * Inside the thread, create a loop that waits for commands. It holds a counter variable initialized to 0.
    * When it receives an `Increment` command, it increments the counter.
    * When it receives a `GetValue` command, it prints the current value of the counter.
3. In `main.rs`, import the `actor` module.
4. Start the actor, getting the sender.
5. Send it some `Increment` commands.
6. Send it a `GetValue` command to see the current value.

![](../../images/ScrollTime.png)

Here's an implementation:

`actor.rs`:

```rust
use std::sync::mpsc;
use std::thread;

pub enum Command {
    Increment,
    GetValue,
}

pub fn start_actor() -> mpsc::Sender<Command> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut counter = 0;
        for command in rx {
            match command {
                Command::Increment => {
                    counter += 1;
                }
                Command::GetValue => {
                    println!("Current counter value: {}", counter);
                }
            }
        }
    });

    tx
}
```

`main.rs`:

```rust
mod actor;

use actor::{start_actor, Command};
use std::thread;
use std::time::Duration;

fn main() {
    let actor_sender = start_actor();

    for _ in 0..5 {
        actor_sender.send(Command::Increment).unwrap();
    }

    actor_sender.send(Command::GetValue).unwrap();

    // Give the actor some time to process before the main thread exits
    thread::sleep(Duration::from_millis(100));
}
```

When you run this program, it will output:

```
Current counter value: 5
```