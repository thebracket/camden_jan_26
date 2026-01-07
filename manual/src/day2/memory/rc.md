# Rc - Reference Counted Single-Threaded Shared Ownership

C++'s `shared_ptr` has an interesting property that it has both a single-threaded and a multi-threaded version, but by default the *compiler* determines which one to use!

Rust takes a different approach: it has two different types, `Rc<T>` and `Arc<T>`. `Rc<T>` is for single-threaded use, and `Arc<T>` is for multi-threaded use.

Let's start with `Rc<T>`. It stands for "Reference Counted". Internally, it's a `usize` counter that keeps track of how many `Rc<T>` pointers point to the same value. When the last `Rc<T>` pointer goes out of scope, the value is de-allocated.

When you `clone()` an `Rc<T>`, it increments the reference count. When an `Rc<T>` goes out of scope, it decrements the reference count. When the reference count reaches zero, the value is de-allocated.

> The Rust community is working on adopting a new verb. `clone()` is being used by too many things. When you `clone()` a regular struct, a deep-copy is made. When you `clone()` an `Rc` - only the reference count is incremented and you get a pointer to the same object. Database connection pools are often cloneable, but they don't deep-copy the pool! It's a mess. The new solution may be to use `handle` as a name---but it isn't agreed upon yet.

## When to use Rc<T>

You use `Rc<T>` when you want to share ownership of a value between multiple parts of your program, but you know that all those parts will be running on the same thread. It alllows you to abstract the ownership model. So if exact ownership is unclear, or you have a graph structure where multiple nodes point to the same child node, `Rc<T>` is a good choice.

## Simple Example

```rust
use std::rc::Rc;

struct MyStruct {
    data: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct with data: {}", self.data);
    }
}

fn main() {
    let my_struct = Rc::new(MyStruct {
        data: String::from("Hello, world!"),
    });

    let my_struct_clone = Rc::clone(&my_struct);
    println!(
        "Reference count after clone: {}",
        Rc::strong_count(&my_struct)
    );

    {
        let another_clone = Rc::clone(&my_struct);
        println!(
            "Reference count after another clone: {}",
            Rc::strong_count(&my_struct)
        );
    } // another_clone goes out of scope here

    println!(
        "Reference count after another_clone goes out of scope: {}",
        Rc::strong_count(&my_struct)
    );
}
```

## Weak References

Sometimes you want to have a reference to a value, but you don't want to contribute to its reference count. This is where `Weak<T>` comes in. A `Weak<T>` is a non-owning reference to an `Rc<T>`. It doesn't increment the reference count, so it won't keep the value alive.

This allows you to create cyclic data structures without causing memory leaks. You can upgrade a `Weak<T>` to an `Rc<T>` using the `upgrade()` method, which returns an `Option<Rc<T>>`. If the value has already been de-allocated, it returns `None`.

```rust
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    next: Option<Rc<Node>>,
    prev: Option<Weak<Node>>,
}

impl Drop for Node {
    // NOTE: You DO NOT need to implement Drop for Node in real code. This is just for demonstration.
    fn drop(&mut self) {
        println!("Dropping Node with value: {}", self.value);
    }
}

fn main() {
    let first = Rc::new(Node {
        value: 1,
        next: None,
        prev: None,
    });

    let second = Rc::new(Node {
        value: 2,
        next: None,
        prev: Some(Rc::downgrade(&first)),
    });

    // Create a cycle
    if let Some(first_next) = Rc::get_mut(&mut Rc::clone(&first)) {
        first_next.next = Some(Rc::clone(&second));
    }

    println!(
        "First node reference count: {}",
        Rc::strong_count(&first)
    );
    println!(
        "Second node reference count: {}",
        Rc::strong_count(&second)
    );
}
```