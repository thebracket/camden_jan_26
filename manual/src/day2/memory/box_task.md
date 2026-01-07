# Box Task

Let's implement a very simple, *singly* linked list using `Box<T>`.

> Please don't try a doubly linked list. That gets *painful*, we'll talk about that soon!

1. Create a Rust project named `box_list`.
2. Create a struct named `Node` that has two fields:
    * `value: i32`
    * `next: Option<Box<Node>>`
3. Implement a method named `new` for `Node` that takes an `i32` and returns a `Node` with the given value and `next` set to `None`.
4. Implement a method named `append` for `Node` that takes an `i32` and appends a new `Node` with the given value to the end of the list.
5. Implement a method named `print` for `Node` that prints the values of the list.
6. In the `main` function, create a `Node` with the value `1`, append the values `2`, `3`, and `4`, and then print the list.

![](../../images/ScrollTime.png)

Here's one possible solution:

```rust
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }

    fn append(&mut self, value: i32) {
        match &mut self.next {
            Some(next_node) => next_node.append(value),
            None => self.next = Some(Box::new(Node::new(value))),
        }
    }

    fn print(&self) {
        print!("{} ", self.value);
        if let Some(next_node) = &self.next {
            next_node.print();
        }
    }
}

fn main() {
    let mut head = Node::new(1);
    head.append(2);
    head.append(3);
    head.append(4);
    head.print();
    println!();
}
```

When you run this program, it will output:

```
1 2 3 4 
```

> Linked Lists are a classic data structure, but they are rarely used in practice. Modern CPUs make vectors outperform them in almost every case. You can also use `std::collections::LinkedList` if you really need one!