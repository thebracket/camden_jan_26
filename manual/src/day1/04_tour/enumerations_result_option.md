# Why Result&lt;Option&gt; Exists

I get asked this a lot: "Why do we have both `Result<T, E>` *and* `Option<T>`? Why not just have `Result<T, ()>` for everything?"

It's quite possible to have `Result<Option<T>, E>`. For example, you query a row in a database. The query might fail (an error), or it might succeed but return no rows (None), or it might succeed and return a row (Some).

Rust is all about being pedantic and explicit.