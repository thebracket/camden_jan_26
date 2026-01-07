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