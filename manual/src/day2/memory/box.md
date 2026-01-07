# Box - Single Ownership

The simplest smart pointer in Rust is `Box<T>`. A `Box` is a pointer that owns a value of type `T` on the heap. When the `Box` goes out of scope, it automatically de-allocates the memory for the value it owns.

Box is a pointer to an object on the heap. It provides single ownership of that object: you can't copy/clone it, only move it. When the `Box` goes out of scope, the memory is automatically freed.

You've already used `Box` indirectly when you created a `String` or a `Vec`. Both of those types use `Box` internally to store their data on the heap.

Here's a simple example of using `Box`:

```rust
fn main() {
    // Create a Box that owns an integer on the heap
    let my_box = Box::new(42);

    // Access the value inside the Box
    assert_eq!(*my_box, 42);

    // my_box goes out of scope here, and the memory is automatically freed
}
```

## Using Box with Custom Types

You can also use `Box` with your own custom types. Here's an example:

```rust
struct MyStruct {
    data: String,
}

fn main() {
    // Create a Box that owns a MyStruct on the heap
    let my_box = Box::new(MyStruct {
        data: String::from("Hello, world!"),
    });

    // Access the data inside the MyStruct
    assert_eq!(my_box.data, "Hello, world!");

    // my_box goes out of scope here, and the memory is automatically freed
}
```

## Nullable Boxes

Rust doesn't have a `null` pointer like C or C++ (well it does, but it's `unsafe` and mostly used for FFI). Instead, you can use the `Option<Box<T>>` type to represent a nullable box. Here's an example:

```rust
fn main() {
    let some_box: Option<Box<i32>> = Some(Box::new(42));
    let none_box: Option<Box<i32>> = None;
    assert_eq!(*some_box.unwrap(), 42);
    assert!(none_box.is_none());
}
```

## Cleanup is Transitive

When a `Box` goes out of scope, Rust automatically calls the `drop` method for the value it owns. If that value is a struct that contains other values, Rust will recursively call `drop` for those inner values first, then for the outer struct.

Here's an example demonstrating this:

```rust
struct Outer {
    inner: Box<Inner>,
}

struct Inner {
}

impl Drop for Inner {
    fn drop(&mut self) {
        println!("Dropping Inner");
    }
}

impl Drop for Outer {
    fn drop(&mut self) {
        println!("Dropping Outer");
    }
}

fn main() {
    let outer = Outer {
        inner: Box::new(Inner {}),
    };
    // outer goes out of scope here
}
```

When you run this program, you'll see the following output:

```
Dropping Inner
Dropping Outer
```

## Boxed Slices

You can create a fixed-size array on the heap using `Box<[T]>`. Here's an example:

```rust
fn main() {
    let boxed_slice: Box<[i32]> = vec![1, 2, 3, 4, 5].into_boxed_slice();
    assert_eq!(&*boxed_slice, &[1, 2, 3, 4, 5]);
}
```

Sometimes, you don't *want* to allow your vector to grow or shrink. In that case, you can convert it into a boxed slice, which has a fixed size.

> Creating an array directly in a Box requires "placement new", which isn't in stable Rust currently.

## Other Box Features

`Box` is a very versatile type.

A few safe features for Box:

* You can dereference a `Box<T>` to get a `&T` using the `*` operator.
* You can borrow a `Box<T>` as a `&T` or `&mut T` using the `&` and `&mut` operators.
* You can use `Box<T>` in trait objects to enable dynamic dispatch. (We'll cover this more in the Traits section.)
* Serde actually supports boxes, so you can serialize and deserialize boxed values - and it works just like normal values!

There are also some `unsafe` features, that make interop/FFI easier:

* You can claim ownership of a value on the heap by using `Box::from_raw` (unsafe).
* You can convert a `Box<T>` into a raw pointer using `Box::into_raw` (unsafe).
* You can convert a raw pointer back into a `Box<T>` using `Box::from_raw` (unsafe).
* You can deliberately leak a `Box<T>` using `Box::leak`, which returns a mutable reference to the value inside the box, and prevents the memory from being freed when the box goes out of scope. (safe, but it probably shouldn't be!)