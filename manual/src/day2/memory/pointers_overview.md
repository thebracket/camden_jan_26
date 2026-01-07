# Smart Pointer Types Overview

Rust wanted to preserve the benefits of RAII and smart pointers from C++, while avoiding the pitfalls of manual memory management. So Rust provides several smart pointer types that manage memory automatically, while still allowing for fine-grained control over ownership and borrowing.

We'll go over the three main smart pointer types in Rust:
* `Box<T>` - Single Ownership
* `Rc<T>` - Reference Counted Single-Threaded Shared Ownership
* `Arc<T>` - Atomic Reference Counted Multi-Threaded Shared Ownership

Each of these types has its own use cases and trade-offs. Let's take a look at each one in more detail.