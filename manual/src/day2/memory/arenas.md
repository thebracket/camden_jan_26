# Arenas

An "Arena" is a pre-allocated area of memory that you use over and over to store data. A vector with a preset capacity that will never grow is the simplest form of arena. Arenas are used:

* When you need to avoid memory fragmentation.
* When allocation is expensive; you allocate your arena up-front and then use it.
* When you absolutely can't afford to fail to allocate memory. Arenas are pre-allocated, so you can't fail to allocate memory once they are started.
* On some platforms (real-time), you can't allocate memory after the program starts. You have to pre-allocate everything you need.
* Some arenas are used to store data that is never de-allocated. This is common in games, where you might have a "level" arena, a "game" arena, and a "global" arena. You can de-allocate the level arena when you move to a new level, but the game and global arenas are never de-allocated.
* In turn, this can allow you to fix the pain of "cyclic references"---references that refer to one another. If you have a cyclic reference, you can't de-allocate the memory. If you use an arena, you can de-allocate the entire arena at once, and the cyclic references are no longer a problem.

### Bump Style Arenas

A "bump" style arena is the simplest form of arena. You allocate a chunk of memory, and then you just keep allocating from it. You keep a pointer to the end of the allocated memory, and when you run out, you allocate another chunk. You can't de-allocate memory, but you can de-allocate the entire arena at once.

This allows you to solve cyclic references, and by pre-allocating the arena, you can avoid the problem of running out of memory.

> See `code/day2/arena_bump` for code.

We'll test out [Bumpalo](https://docs.rs/bumpalo/latest/bumpalo/). Bumpalo is pretty easy to use:

```rust
use bumpalo::Bump;

struct MyData {
    a: i32,
}

fn main() {
    let arena = Bump::new();
    arena.set_allocation_limit(Some(8192)); // Limit the size of the arena to 8 KiB
    let x = arena.alloc(MyData { a: 123 });
}
```

You can also enable the `collections` feature and use `BumpaloVec` and `BumpaloString` to store data in the arena:

```rust
use bumpalo::Bump;
use bumpalo::collections::String;
use bumpalo::collections::Vec;

struct MyData {
    a: i32,
}

fn main() {
    let arena = Bump::new();
    arena.set_allocation_limit(Some(8192)); // Limit the size of the arena to 8 KiB
    let x = arena.alloc(MyData { a: 123 });

    // With collections enabled
    let my_string = String::from_str_in("Hello, world!", &arena);
    let mut vec = Vec::new_in(&arena);
    vec.push(12);

    println!("{:?}", x);
    println!("{}", my_string);
    println!("{:?}", vec);
}
```

**Downside**: `Drop` will never be called in a `Bump` arena. You can enable unstable compiler features and make it work, but for now---you're not dropping anything in the arena!

Use a `bump` arena to allocate memory up-front (or in chunks) and store data inside the arena. You can't de-allocate individual items, but for something like a data-collector that *must not* suddenly fail to allocate memory or expand its heap, it's a great choice.

### Slab Arenas


A "slab arena" pre-allocates space for a uniform type, indexing each entry by key. This is similar to a pre-allocated `Vec`, but you don't have to keep `usize` around for entries---and the slab keeps track of vacant entries for you. It's also similar to a `HashMap`, but you don't have to hash keys. Slabs are great for pre-allocating a big pool of resources and then using them as needed.

> See `code/day2/arena_slab` for code.

```rust
use slab::Slab;

fn main() {
    let mut slab = Slab::with_capacity(10);
    let hello = slab.insert("hello");
    let world = slab.insert("world");

    assert_eq!(slab[hello], "hello");
    assert_eq!(slab[world], "world");

    slab.remove(hello);
}
```

Note that you *can* remove items! Slabs work like a "slot map" - entries are either `Vacant` or filled with your data type. Slabs won't ever fragment, and entries will be stored in contiguous memory. This makes them very fast to iterate over. If you can preallocate a slab of data, it's a great choice for high-performance and not fragmenting memory.
