# Procedural Macros

> See `code/day3/procmacros` for the code.

I admit it. I saw "proc macros" as a question and thought "wow, I'll be here all month". So this is a *condensed* approach that is going to point you at resources and only skim the surface. Procedural macros are an *enormous* topic.

There's a really great tutorial here: [https://www.freecodecamp.org/news/procedural-macros-in-rust/](https://www.freecodecamp.org/news/procedural-macros-in-rust/)

## When to use Macros

Also, macros compile pretty slowly. You can really hurt your compilation times with a lot of macros.

So, use macros sparingly - and enjoy the power they provide.

## Types of Macro

Procedural macros receive the AST (Abstract Syntax Tree) tokens and modify it at compile time. This lets you modify pretty much anything. Proc Macros are either:

* `derive` macros - when you use `#[derive(Whatever)]` - you are invoking a derive procedural macro.
* `attribute` macros. When you see a function with `#[must_use]` - that's an attribute macro. The tracing crate uses `#[instrument]` to add in telemetry.
* `function-like` macros are macros disguised as functions. You don't "tag" these - but you call them like a function.

## Crates

Procedural macros only work for projects marked as such in `Cargo.toml`. Once you've added this mark, it's problematic to *also* have functionality. So you almost always end up with a crate for your procedural macros, and a crate for your library. You sometimes end up needing three (or more) crates to avoid circular dependencies.

That's why when you include `serde` with the `derive` feature, you see a `serde` crate *and* a `serde_derive` crate in your dependency tree: the procedural macros *must* be isolated.

## Helpful Libraries

Reading and writing an AST token stream isn't a lot of fun (well, maybe someone likes it). Pretty much everybody who isn't David Tolnay (author of many of these crates and Serde) uses some dependencies:

* `syn` parses token streams into Rust syntax.
* `quote` parses Rust syntax into token streams.
* `proc-macro2` exposes proc-macro features outside of the compiler's proc-macro context.
* `darling` makes handling proc macro arguments a lot less cumbersome.

## Let's make a derive macro

We'll build a derive macro named `HelloMacro` and use it to auto-generate a method named `hello_macro`.

### A macro stub

In your proc-macro crate:

```rust
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloMacro)]
pub fn derive_hello(item: TokenStream) -> TokenStream {
    todo!("We haven't written this yet");
}
```

### Start by scanning the structure

We'll parse the input with `syn`, and grab the struct identifier:

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::Data;

#[proc_macro_derive(HelloMacro)]
pub fn derive_hello(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let struct_id = &input.ident;

    match input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            todo!("next: emit Rust");
        }
        _ => unimplemented!(),
    }
}
```

### Emit some Rust

Once we've parsed the structure, we'll use `quote!` to generate an `impl` block:

```rust
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(HelloMacro)]
pub fn derive_hello(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let struct_id = &input.ident;

    match input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut implementation = quote! {
                println!("I am a struct of type [{}]", stringify!(#struct_id));
            };

            for field in fields {
                let field_name = field.ident.unwrap();
                implementation.extend(quote! {
                    println!("Field: [{}] = [{}]",
                        stringify!(#field_name),
                        self.#field_name,
                    );
                });
            }

            quote! {
                impl #struct_id {
                    fn hello_macro(&self) {
                        #implementation
                    }
                }
            }
            .into()
        }
        _ => unimplemented!(),
    }
}
```

And from a normal crate, you can use it like this:

```rust
use deriver_macros::HelloMacro;

#[derive(HelloMacro)]
struct MyData {
    name: String,
}

fn main() {
    let person = MyData {
        name: "Alice".to_string(),
    };
    person.hello_macro();
}
```

