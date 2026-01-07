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