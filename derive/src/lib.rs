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
//! Note that skipped items are taken into consideration when using [core::hash::Hash], [PartialEq] or [Eq].
//!
//! ## Generation
//!
//! We generate documents for automatically generated types, just hover on those types or view them in documents of your project to see them!
//!
//! ### Enumeration of Rules
//!
//! The same with [pest](https://docs.rs/pest).
//!
//! It implements `Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd`.
//!
//! ## APIs
//!
//! Note: to use **pest_typed_derive** as a dependency, **pest_typed** is also needed.
//!
//! ### Pairs API
//!
//! Note: the simulated [Pairs](#https://docs.rs/pest/latest/pest/iterators/struct.Pairs.html) API behaves a bit different from original version. An atomic rule will not contain inner pairs.
//!
//! ### Accesser API
//!
//! See [`fn@derive_typed_parser`] for how to enable Accesser API.
//!
//! Accesser API is a group of functions, called **Accesser Functions**, to access referenced rules (or tags, if enabled).
//!
//! Accesser function is named with the same name of the rule that it's accessing.
//!
//! For example, if you have
//!
//! ```pest
//! foo = { bar }
//! ```
//!
//! you can access `bar` from an instance `f` of `foo` by calling `f.bar()`.
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
//! ### Rule Structs
//!
//! We generate a **Rule Struct** for each rule.
//! The inner structure is generated from the grammar structure inside the rule (or parsing expression grammar).
//!
//! And the [pest](https://pest.rs) grammar is displayed in short in doc comments using [core::fmt::Display] so that you can view the structure without switching to `.pest` files.
//!
//! #### Emitted Fields for Rule Structs
//!
//! There are three cases related to fields of a generated `struct`:
//!
//! - Emit inner nodes and a span (normal rule, non-atomic rule and compound atomic rule in **pest**).
//! - Emit a span (atomic rule in **pest**).
//! - Emit inner expression (silent rule in **pest**).
//!
//! #### Example for Rule Structs
//!
//! ```rust
#![doc = include_str!("../examples/rule_structs.rs")]
//! ```
//!
//! ### Tag Structs
//!
//! We generate a **Rule Struct** for each tag.
//! The inner structure is generated from the grammar structure inside the tag.
//!
//! #### Emitted Fields for Tag Structs
//!
//! Fields:
//!
//! - Inner content.
//! - Span for matched input.
//!
//! #### Example for Tag Structs
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
//! ### Normal Nodes
//!
//! We can handle complexer problems with lower-level API (also named **Structual API**).
//!
//! But note that the structure of a **Rule Struct** indirectly depends on the optimizer in **pest**, so it may change in the future.
//!
//! Maybe we can use [`pest_meta::ast::Expr`](https://docs.rs/pest_meta/latest/pest_meta/ast/enum.Expr.html) by default in the future.
//!
//! |              Node Type              |                                                  Fields                                                   |                                             Functions                                             |
//! | :---------------------------------: | :-------------------------------------------------------------------------------------------------------: | :-----------------------------------------------------------------------------------------------: |
//! |           Non-silent rule           | Matched `content` (wrapped in a [`Box`]), which can be used to access matched expression; matched `span`. |                                See [Accesser API](#accesser-api).                                 |
//! |    Exact string (case-sensitive)    |                                                                                                           | `const fn get_content(&self)` to get original string, which requires trait `pest_typed::Storage`. |
//! |   Exact string (case-insensitive)   |                                     Matched `content` (an `&'i str`).                                     | `const fn get_content(&self)` to get original string, which requires trait `pest_typed::Storage`. |
//! |        Sequence `T, Res...`         |                                       Matched `content` as a tuple.                                       |          `get_matched(&self)`, which returns referencs of all elemnets `(&elemnts...)`.           |
//! |         Choices `T, Res...`         |                                   An enum, whose variants are choices.                                    |                       `if_then(&self)`, several functions `_0`, `_1`, etc.                        |
//! | Optional (wrapped in an [`Option`]) |                                                                                                           |                                                                                                   |
//! |          Repetition of `T`          |                                Matched `content` wrapped in a [`Vec<T>`].                                 | `iter_matched` and `iter_all` (by reference); `into_iter_matched` and `into_iter_all` (by value). |
//! |         Positive predicate          |                                     Matched `content` (not consumed).                                     |                                                                                                   |
//! |         Negative predicate          |                                                                                                           |                                                                                                   |
//! |          `PUSH` and `PEEK`          |                                            Matched `content`.                                             |                                                                                                   |
//! |         `POP` and `POP_ALL`         |                                              Popped `span`.                                               |                                                                                                   |
//! |               `DROP`                |                                                                                                           |                                                                                                   |
//!
//! #### Sequence
//!
//! One can use `get_matched` by reference (or `into_matched` by value) to access elements within a sequence directly.
//!
//! Both functions return a tuple.
//!
//! #### Choices
//!
//! Choices can be matched using `match`, as long as you find where its type is defined.
//! Auto-generated choices types are named as `Choice{n}` where `n` is the count of choices.
//! And every generics used can be found in mod `generics`.
//!
//! Similarly, we provide a proc macro `match_choices` to handle choices
//! with a bit simpler syntax.
//! Note that you need to import module `generics`.
//!
//! What's more, we provide several functions that simulate control structure
//! like `if` (`if_then(f)`), `else-if` (`else_if(f)`) and `else` (`else_then(f)`).
//!
//! Each of those functions accept a function `f` as argument,
//! if and only if the branch is the actual case, `f` is called.
//!
//! The structure must start with `if_then(f)`.
//! And `else_if` is only available when there are at least two cases that haven't been handled,
//! so if it's the last case, use `else_then(f)` instead.
//!
//! While `else_then(f)` returns the final result,
//! `if_then(f)` and `else_if(f)` return a temporary helper object.
//!
//! Using these functions, one can handle those cases one by one in order.
//!
//! #### Example
//!
//! ```rust
#![doc = include_str!("../examples/structural_api.rs")]
//! ```
//!
//! ### Lifetime
//!
//! Structs have fields that contains references borrowed from the input, so each of them has a lifetime argument `'i`.
//!
//! Sometimes, you may encounter a lifetime error. Do not panic, just consider them seriously. And we'll fix them if it's caused by bad API design.
//!
//! ```rust
#![doc = include_str!("../examples/lifetime.rs")]
//! ```

#![warn(
    missing_docs,
    rust_2018_idioms,
    rustdoc::all,
    unused_qualifications,
    future_incompatible
)]

use proc_macro::TokenStream;

/// The main method that's called by the proc macro is [`pest_typed_generator::derive_typed_parser()`].
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

/// The main method that's called by the proc macro is [`pest_typed_generator::match_choices()`].
///
/// Please imports generated module `generics` so that the macro can use it.
///
/// See [`pest_typed_generator`] for more information.
#[proc_macro]
pub fn match_choices(input: TokenStream) -> TokenStream {
    pest_typed_generator::match_choices(input.into()).into()
}
