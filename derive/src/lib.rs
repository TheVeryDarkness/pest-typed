// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Derive statically typed nodes and parser from pest grammar.
//! Aimed to add enhancement to pest for those in need.
//!
//! When using this crate, remember to add **pest_typed** as a dependency.
//!
//! - Refer to [pest](https://pest.rs) for pest's syntax and built-in rules.
//! - Refer to underlying crate [`pest_typed_generator`] for how it generates codes.
//! - Refer to the derive macro [`fn@derive_typed_parser`] to see what it will generates and how to use the macro.
//!
//! ## Pest Grammars Related
//!
//! ### Auto-skipped Rules
//!
//! When a rule is not atomic, inner contents that match `COMMENT` or `WHITESPACE` will be skipped automatically, and `COMMENT` is prior to `WHITESPACE`.
//!
//! ## Generation
//!
//! We generate documents for automatically generated types, just hover on those types or view them in documents of your project to see them!
//!
//! ### Enumeration of Rules
//!
//! The same with [pest](https://docs.rs/pest).
//!
//! It implement `Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd`.
//!
//! ### Structs that are Generated for Rules
//!
//! We generate a **Rule Struct** for each rule.
//!
//! #### Fields
//!
//! There are three cases related to fields of a generated `struct`:
//!
//! - Emit inner nodes and a span (normal rule, non-atomic rule and compound atomic rule in **pest**).
//! - Emit a span (atomic rule in **pest**).
//! - Emit nothing (silent rule in **pest**, but note that this is not like what `Pair`/`Pairs` API does in **pest**). Just parse and drop.
//!
//! ## Examples
//!
//! Note: to use **pest_typed_derive** as a dependency, **pest_typed** is also needed.
//!
//! ### Accesser API
//!
//! Accesser API is a group of functions, called **Accesser Functions**, to access referenced rules (or tags, if enabled).
//!
//! Accesser function is named with the same name of the rule that it's accessing.
//!
//! See [`fn@derive_typed_parser`] for how to enable Accesser API.
//!
//! Given the pest grammar being:
//!
//! ```pest
#![doc = include_str!("../examples/csv.pest")]
//! ```
//!
//! Here is a basic example on how to access and process referenced rules in a rule using Accesser API:
//!
//! ```rust
#![doc = include_str!("../examples/accesser_api.rs")]
//! ```
//!
//! ### Emitted Fields of Rule Structs
//!
//! A rule structs is a struct that corresponds to a rule.
//!
//! ```rust
#![doc = include_str!("../examples/rule_structs.rs")]
//! ```
//!
//! ### Emitted Fields of Tag Structs
//!
//! An example using node tags.
//!
//! ```rust
#![doc = include_str!("../examples/node_tags.rs")]
//! ```
//!
//! An example using nested node tags.
//!
//! ```rust
#![doc = include_str!("../examples/nested_node_tags.rs")]
//! ```
//!
//! ### Lifetime
//!
//! Structs have fields that are contains references borrowed from the input, so each of them has a lifetime argument `'i`.
//!
//! Sometimes, you may encounter a lifetime error. Do not panic, just consider them seriously.
//!
//! ```rust
#![doc = include_str!("../examples/lifetime.rs")]
//! ```
//!
//! ### Emitted Fields and Functions of Nodes
//!
//! We can handle complexer problems with lower-level API (also named **Structual API**).
//!
//! But note that the structure of a **Rule Struct** depends on the optimizer in **pest**, so it may change in the future.
//!
//! Maybe we can use [`pest_meta::ast::Expr`](https://docs.rs/pest_meta/latest/pest_meta/ast/enum.Expr.html) by default in the future.
//!
//! |            Node Type            |                                      Fields                                      |                                              Functions                                               |
//! | :-----------------------------: | :------------------------------------------------------------------------------: | :--------------------------------------------------------------------------------------------------: |
//! |         Non-silent rule         | Matched `content`, which can be used to access match expression; matched `span`. |                                  See [Accesser API](#accesser-api)                                   |
//! |  Exact string (case-sensitive)  |                                                                                  | Original string to match, `const fn get_content(&self)`, which requires trait `pest_typed::Storage`. |
//! | Exact string (case-insensitive) |                        Matched `content` (an `&'i str`).                         | Original string to match, `const fn get_content(&self)`, which requires trait `pest_typed::Storage`. |
//! |      Sequence `T, Res...`       |                          matched `content` as a tuple.                           |               `as_ref(&self)`, which returns referencs of all elemnets `(&elemnts...)`               |
//! |       Choices `T, Res...`       |                    Variants, each of which is `(content, _)`.                    |                                           `if_then(&self)`                                           |
//! |            Optional             |                    Matched `content` wrapped in a [`Option`].                    |
//! |        Repetition of `T`        |                         Matched `content` (an `Vec<T>`).                         |
//!
//! For multi-elements sequence and multi-branches choices, its underlying implementation is like a list in functional programing. Those fields or variants are not so easy to read and use, and it's recommended to use function API.
//!
//! #### Sequence
//!
//! One can use `as_ref` to access elements within a sequence directly.
//!
//! #### Choices
//!
//! We provide several functions that simulate control structure like `if` (`if_then(f)`), `else-if` (`else_if(f)`) and `else` (`else_then(f)`).
//!
//! Each of those functions accept a function `f` as argument, if and only if the branch is the actual case, `f` is called.
//!
//! The structure must start with `if_then(f)`. And `else_if` is only available when there are at least two cases that haven't been handled, so if it's the last case, use `else_then(f)` instead.
//!
//! Except that `else_then(f)` returns the final result, `if_then(f)` and `else_if(f)` will return a temporary helper object.
//!
//! Using these functions, one can handle those cases one by one in order.
//!
//! #### Example
//!
//! ```rust
#![doc = include_str!("../examples/structural_api.rs")]
//! ```

#![warn(
    missing_docs,
    rust_2018_idioms,
    rustdoc::all,
    unused_qualifications,
    future_incompatible
)]

use proc_macro::TokenStream;

/// The main method that's called by the proc macro [`pest_typed_generator::derive_typed_parser`].
///
/// See [`pest_typed_generator`] for more information.
#[proc_macro_derive(
    TypedParser,
    attributes(
        grammar,
        grammar_inline,
        emit_rule_reference,
        emit_tagged_node_reference,
        do_not_emit_span,
        simulate_pair_api,
        no_warnigs,
    )
)]
pub fn derive_typed_parser(input: TokenStream) -> TokenStream {
    pest_typed_generator::derive_typed_parser(input.into(), true).into()
}
