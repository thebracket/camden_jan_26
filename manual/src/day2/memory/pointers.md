# Pointers (Box, Rc, Arc)

Hopefully, you can see what we're building up to here. Rust has no garbage collector, but does have RAII. Rust has low-level allocation and de-allocation - but it's tagged `unsafe`, and discouraged for everyday use.

## C/C++ History

Let's look at the evolution of pointer management in C and C++.

In C, you have `malloc` and `free`. You allocate memory on the heap with `malloc`, and when you're done with it, you call `free` to de-allocate it. If you forget to call `free`, you have a memory leak.

```c
#include <stdlib.h>

int main() {
    int* my_num = (int*)malloc(sizeof(int));
    *my_num = 42;
    // use my_num
    free(my_num); // if you forget this, you have a memory leak
    return 0;
}
```

That works, but it's easy to forget to call `free`, and it's easy to call `free` too many times (causing a double-free error).

C++ started out with `new` and `delete`. Very similar to `malloc` and `free`, but you can attach constructors and destructors to your types. So you can make a class that contains a pointer, and in the destructor, you can call `delete` on that pointer.

```cpp
#include <iostream>

class MyClass {
public:
    MyClass() {
        data = new int;
        *data = 42;
    }
    ~MyClass() {
        delete data;
    }
    void print() {
        std::cout << *data << std::endl;
    }
private:
    int* data;
};

int main() {
    MyClass* my_class = new MyClass();
    my_class->print();
    delete my_class; // calls destructor, which deletes data
    return 0;
}
```

That's better - you ony have to remember the `delete` for `my_class`. This opened up the door for RAII (Resource Acquisition Is Initialization) - where resources are tied to the lifetime of objects.

In modern C++ (C++11 and later), they added "smart pointers" - `std::unique_ptr` and `std::shared_ptr`.

* `std::unique_ptr` is a pointer that owns a resource exclusively. When the `unique_ptr` goes out of scope, it automatically deletes the resource. You can't copy a `unique_ptr`, only move it.
* `std::shared_ptr` is a pointer that can be shared among multiple owners. It keeps a reference count of how many `shared_ptr`s point to the same resource. When the last `shared_ptr` goes out of scope, it deletes the resource.

```cpp
#include <iostream>
#include <memory>

int main() {
    {
        std::unique_ptr<int> my_num = std::make_unique<int>(42);
        std::cout << *my_num << std::endl;
    } // my_num goes out of scope, memory is freed

    {
        std::shared_ptr<int> my_num1 = std::make_shared<int>(42);
        {
            std::shared_ptr<int> my_num2 = my_num1; // reference count increases
            std::cout << *my_num2 << std::endl;
        } // my_num2 goes out of scope, reference count decreases
        std::cout << *my_num1 << std::endl;
    } // my_num1 goes out of scope, memory is freed

    return 0;
}
```

This is the history that Rust is building on. RAII, combined with "smart pointers", makes memory management a lot easier and safer - it's hard to forget to free memory, and it's hard to double-free memory.