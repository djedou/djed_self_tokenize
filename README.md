# djed_self_tokenize  

A procedural macro derive allowing for self tokenizing Rust data structures

## Purpose
Treat data as code and code as data.

The entry and exit points for these procedural macros are `proc_macro::TokenStream`s, therefore any instance of a data structure created as part of the compilation process cannot be integrated in the final code unless tokenized. The [quote](https://github.com/dtolnay/quote) library exposes a `quote::ToTokens` trait which can be then directly translated into a `proc_macro::TokenStream`. However, implementing this trait on all of your data structures is tedious and can be automated. This crate offers a custom derive which implements the `quote::ToTokens` trait, allowing arbitrary values to be tokenized into rust tokens.

## How to use

Therefore, to create procedural macros and use this crate, you need Nightly:
```sh
rustup default nightly
```

Add this to your `Cargo.toml` file:

```toml
[dependencies]
djed_self_tokenize_macro = { git = "https://github.com/djedou/djed_self_tokenize" }
djed_self_tokenize_trait = { git = "https://github.com/djedou/djed_self_tokenize" }
```

Then, simply import the library into your code and derive the `SelfTokenize` trait on your data structures.

```rust
#![feature(proc_macro)]
extern crate djed_self_tokenize_macro;
extern crate djed_self_tokenize_trait;

use djed_self_tokenize_macro::SelfTokenize;
use djed_self_tokenize_trait::ToCustomTokens;

#[derive(SelfTokenize)]
struct MyExampleUnit;

#[derive(SelfTokenize)]
struct MyExampleStruct {
  foo: String,
  bar: MyExampleTupleStruct,
  baz: Vec<MyExampleEnum>
}

#[derive(SelfTokenize)]
struct MyExampleTupleStruct(String);

#[derive(SelfTokenize)]
enum MyExampleEnum {
  FooVariant(MyExampleUnit),
  BarVariant {
    unit: MyExampleUnit
  }
}
```

Then you can serialize the above structs to tokens (without consuming them).

```rust
extern crate quote;
use quote::ToTokens;

let value = MyExampleStruct {
  foo: "Hello".to_string(),
  bar: MyExampleTupleStruct("world!".to_string()),
  baz: vec![
    MyExampleEnum::FooVariant(MyExampleUnit),
    MyExampleEnum::BarVariant {
      unit: MyExampleUnit
    },
  ]
};

let mut tokens = quote::Tokens::new();
value.to_custom_tokens(&mut tokens);

// If needed, tokens can also be stringified
let stringified = tokens.to_string();

// ...and also parsed as valid rust code into a token stream
let parsed = tokens.parse().unwrap();
```

When writing a compiler plugin as a procedural macro, you can tokenize the values generated as part of the compilation process to inject them into the generated code as if they were written there in the first place, by serializing to `proc_macro::TokenStream` instead of strings:

```rust
#![feature(proc_macro)]

#[macro_use]
extern crate quote;
extern crate proc_macro;

use quote::ToTokens;

#[proc_macro]
pub fn boring_compiler_plugin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let value = compute_value(&input);
  let tokens = quote! { #value };
  tokens.parse().unwrap()
}
```

In the above example, we're serialize the `value` to generate a `quote::Tokens` which can be then directly translated into a `proc_macro::TokenStream`, used by the rust compiler. See [procedural macros](https://doc.rust-lang.org/book/first-edition/procedural-macros.html), [syn](https://github.com/dtolnay/syn) and [quote](https://github.com/dtolnay/quote) crates for more information.

This turns

```rust
let value = boring_compiler_plugin!();
```

into

```rust
let value = MyExampleStruct {
  foo: "Hello".to_string(),
  bar: MyExampleTupleStruct("world!".to_string()),
  baz: vec![
    MyExampleEnum::FooVariant(MyExampleUnit),
    MyExampleEnum::BarVariant {
      unit: MyExampleUnit
    },
  ]
};
```