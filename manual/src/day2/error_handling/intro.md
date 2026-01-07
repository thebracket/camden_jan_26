# Error Handling Walkthrough

We've already run into a bit of error handling - we even implemented some.

* `Result<T, E>` types. You used a `String` for the errort type.
* `map_err` to convert between error types.
* `?` operator to propagate errors.

We waited until after traits and `Box<dyn Trait>` to talk about error handling --- because it makes error handling make more sense.

Rust's error handling is a combination of reactions to different languages:
* C-style `errno` handling, where you check the return value of a function to see if it succeeded or failed.
* C++ exceptions, where you can throw an error and catch it later.
* Java-style checked exceptions, where you have to declare the exceptions a function can throw.

Exceptions are much maligned, so Rust instead opted to return *either* a success value or an error value.

## Error Handling in Other Languages

Language | Error Handling Mechanism
--- | ---
C | Return codes (e.g., `-1` for error, `0` for success)
C++ | Exceptions (e.g., `try`, `catch`, `throw`)
Java | Checked exceptions (e.g., `throws` declaration)
Python | Exceptions (e.g., `try`, `except`, `raise`)
JavaScript | Exceptions (e.g., `try`, `catch`, `throw`)
Go | Multiple return values (e.g., `value, err := function()`)
Haskell | `Either` type (e.g., `Left` for error, `Right` for success)
Swift | `Result` type (e.g., `Result.success` and `Result.failure`)
Ruby | Exceptions (e.g., `begin`, `rescue`, `raise`)
