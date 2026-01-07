# Macros

Rust has macros for when "normal Rust syntax isn't enough".

There are two broad categories:

* Declarative macros (`macro_rules!`) - pattern matching / rewriting.
* Procedural macros - receive tokens, emit tokens, and effectively act like compiler plugins.

This is a whirlwind tour. The goal is to recognize what's going on, and know where to go next.

