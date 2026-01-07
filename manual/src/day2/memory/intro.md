# Memory

Have you noticed that we made it this far without mentioning memory? That's deliberate: Rust is a general purpose language, and you can do a *lot* without ever needing to think about lower-level details like memory management. So far, we've mostly used the stack (or allowed collections to worry about heap allocations for us).

Now let's dig in a bit.

## The Stack vs The Heap

The stack is small---64 kb by default on Linux. So you can't put everything in there. The stack is also very fast: allocating memory on the stack is just a matter of moving the stack pointer. De-allocating memory on the stack is just a matter of moving the stack pointer back.

The heap is much larger, and can grow as needed (up to the limits of your system). Allocating memory on the heap is slower than allocating memory on the stack, because the system has to find a block of memory that is large enough to satisfy the request. De-allocating memory on the heap is also slower than de-allocating memory on the stack, because the system has to keep track of which blocks of memory are in use and which are free.

> Not every platform has a heap, or it may be pre-allocated. That's especially true in embedded systems and high-reliability systems.

## Manually Allocating & De-allocating Memory

The "heap" is a region of memory that is shared by your program, and doesn't have the size-restrictions of the stack. It is *always* allocated and de-allocated. In "managed" languages, the language runtime is still allocating to the heap---but it uses a garbage collector of some sort to de-allocate memory that is no longer needed. This has the advantage that you don't need to worry about it, and the disadvantages:

* You don't know for sure when memory will be allocated. Is it allocated up-front? That's great for systems with a fixed memory size, but not so good for systems where you want to allocate memory on-demand. Is it allocated on first use? That's great for systems where you don't know how much memory you need up-front, but not so good for systems where you want to allocate memory up-front.
* You don't know for sure when the memory will be de-allocated.
* You get the infamous "GC pauses" where the program stops for a while to do garbage collection. The pauses might be very short, but it's still an insurmountable problem if you are trying to control the braking system on a sports car!
* You often have to jump through hoops to use an *exact* heap size, causing issues on embedded systems.

On some embedded platforms, you pretty much get to start out with a `libc` implementation (that may not be complete). On others, you get a platform definition file and have to do things the hard way --- we're not going that far!

You almost *never* have to directly allocate memory, except at the lowest level --- and when you do, it usually involves some `unsafe` tags.

### libc_malloc example

```rust
fn allocate_memory_with_libc() {
    unsafe {
        // Allocate memory with libc (one 32-bit integer)
        let my_num: *mut i32 = libc::malloc(std::mem::size_of::<i32>() as libc::size_t) as *mut i32;
        if my_num.is_null() {
            panic!("failed to allocate memory");
        }

        // Set the allocated variable - dereference the pointer and set to 42
        *my_num = 42;
        assert_eq!(42, *my_num);

        // Free the memory with libc - this is NOT automatic
        libc::free(my_num as *mut libc::c_void);
    }
}

fn main() {
    allocate_memory_with_libc();
}
```

So if you find yourself having to use `libc`, this is what you can expect: it looks a LOT like C! In your `unsafe` block, you are calling `malloc`, checking that it gave you the memory you requested, then setting the value of the memory and finally freeing it.

If you forget to call `free`, then just like a C program---you leaked memory.

### Using Rust's Allocator

Using `malloc` isn't always as simple as it sounds, you need to worry about memory alignment (lining up memory blocks with your platform's "word size"). Rust provides an allocator setup that you can use instead. It's similar, and still `unsafe`:

```rust
fn allocate_memory_with_rust() {
    use std::alloc::{alloc, dealloc, Layout};

    unsafe {
        // Allocate memory with Rust. It's safer to force alignment.
        let layout = Layout::new::<u16>();
        let ptr = alloc(layout);

        // Set the allocated variable - dereference the pointer and set to 42
        *ptr = 42;
        assert_eq!(42, *ptr);

        // Free the memory - this is not automatic
        dealloc(ptr, layout);
    }
}
```

You have pretty much everything you expect from C: pointer arithmetic, `null` pointers, forgetting to call `dealloc` and leaking memory. At this level, it's quite ugly.

> Again, you almost never have to do this. It's good to know it is there - and then use safe abstractions on top of it!