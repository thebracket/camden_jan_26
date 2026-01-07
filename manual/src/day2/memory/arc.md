# Arc - Atomic Reference Counted Multi-Threaded Shared Ownership

`Arc` is *just* like `Rc`, but the counter is implemented as an "atomic" (we'll cover those in threads). This means that multiple threads can safely increment and decrement the reference count without causing data races --- and the borrow checker will let you share `Arc<T>` between threads.

There is a *small* performance cost to using atomics, so only use `Arc<T>` when you need to share ownership between threads. If you're only sharing ownership within a single thread, use `Rc<T>` instead.

## Simple Example

```rust
use std::sync::Arc;
use std::thread;

struct MyStruct {
    data: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct with data: {}", self.data);
    }
}

fn main() {
    let my_struct = Arc::new(MyStruct {
        data: String::from("Hello, world!"),
    });

    let mut handles = vec![];

    for _ in 0..5 {
        let my_struct_clone = Arc::clone(&my_struct);
        let handle = thread::spawn(move || {
            println!("Thread got data: {}", my_struct_clone.data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Reference count after threads: {}",
        Arc::strong_count(&my_struct)
    );
}
```

`Arc` gets used a *lot*. You'll see it in web servers, databases, and other high-performance applications where multiple threads need to share access to the same data.

Rust isn't garbage collected --- but `Arc` gets you pretty close! Many garbage collected languages use reference counting under the hood (Python etc.).