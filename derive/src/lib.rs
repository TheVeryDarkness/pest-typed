// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

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
//! ### Generation
//!
//! #### Enumeration of Rules
//!
//! #### Structs that are Generated for Rules
//!
//! ##### Fields
//!
//! There are three cases related to fields of a generated `struct`:
//!
//! - Emit inner nodes and a span (normal rule, non-atomic rule and compound atomic rule in **pest**).
//! - Emit a span (atomic rule in **pest**).
//! - Emit nothing (silent rule in **pest**). Just parse and drop.

#![doc = include_str!("../example-part1.md")]
#![doc = include_str!("../example-part2.md")]
#![cfg_attr(feature = "grammar-extras", doc = include_str!("../example-part3.md"))]
#![doc = include_str!("../example-part4.md")]

use proc_macro::TokenStream;

/// The main method that's called by the proc macro [`pest_typed_generator::derive_typed_parser`].
#[doc = include_str!("../usage-part1.md")]
#[doc = include_str!("../usage-part2.md")]
#[proc_macro_derive(
    TypedParser,
    attributes(
        grammar,
        grammar_inline,
        emit_rule_reference,
        emit_tagged_node_reference,
        do_not_emit_span,
    )
)]
pub fn derive_typed_parser(input: TokenStream) -> TokenStream {
    pest_typed_generator::derive_typed_parser(input.into(), true).into()
}
