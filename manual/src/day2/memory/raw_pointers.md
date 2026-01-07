# Raw Pointers are Still Here

Underneath all of the safe abstractions, Rust still has raw pointers. They are similar to C/C++ pointers, and are marked with `*const T` for immutable pointers and `*mut T` for mutable pointers.

It's only unsafe when you *derereference* them or do pointer arithmetic. Creating raw pointers is safe. Here's an example:

```rust
fn main() {
    let mut num = 5;

    // Create raw pointers
    let r1: *const i32 = &num; // immutable raw pointer
    let r2: *mut i32 = &mut num; // mutable raw pointer

    unsafe {
        // Dereference raw pointers
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        // Modify the value through the mutable raw pointer
        *r2 = 10;
        println!("After modification, num is: {}", num);
    }
}
```

Pointer arithmetic is also done in an `unsafe` block:

```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];
    let ptr: *const i32 = arr.as_ptr();

    unsafe {
        for i in 0..arr.len() {
            // Perform pointer arithmetic
            let current_ptr = ptr.add(i);
            println!("Element {} is: {}", i, *current_ptr);
        }
    }
}
```

So everything you know about raw pointers from C/C++ still applies in Rust - but you should absolutely use safe abstractions like `Box`, `Rc`, and `Arc` whenever possible!

