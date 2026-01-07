# Serialization with Serde

`Serde` (pronounced "sir-dee") is a framework for serializing and deserializing Rust data structures efficiently and generically. It is the most popular serialization library in the Rust ecosystem. It's one of DTolnay's *many* contributions to the Rust ecosystem, and is used by many other libraries. It's relatively easy to use, quite fast, and is format agnostic.

> I once pronounced it "surd" at RustConf. I was very politely corrected by a LOT of people!

We'll cover the basics of using Serde to serialize and deserialize data structures in Rust. We'll use JSON as our serialization format, but Serde supports many other formats as well.