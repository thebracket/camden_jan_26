fn main() {
    let counter = std::sync::Mutex::new(0);

    std::thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
        }
    });

    println!("Counter: {}", counter.lock().unwrap());
}