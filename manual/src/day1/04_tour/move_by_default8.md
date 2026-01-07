# The Golden Rule

Remember this:

**At any given time, you can have either (but not both of)**
* One mutable reference
* Any number of immutable references

This is the core of Rust's memory safety guarantees. It prevents data races (we'll see more about that later), makes reasoning about your code easier, and helps the compiler optimize your code better.