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
//! Here is a basic example on how to access and process referenced rules in a rule using Accesser API:
//!
//! ```rust
//! extern crate alloc;
//! use alloc::vec::Vec;
//! use core::{iter, result::Result};
//! use pest_typed_derive::TypedParser;
//! use pest_typed::{ParsableTypedNode as _, TypedParser as _, error::Error};
//!
//! /// See https://datatracker.ietf.org/doc/html/rfc4180.html for CSV's format.
//! #[derive(TypedParser)]
//! #[grammar_inline = r#"
#![doc = include_str!("../tests/csv.pest")]
//! "#]
//! #[emit_rule_reference]
//! struct Parser;
//!
//! fn main() -> Result<(), Error<Rule>> {
//!     let input = "name,age\nTom,10\nJerry,20";
//!     let file = pairs::file::parse(input)?;
//!     let (first_row, following_rows) = file.row();
//!     let rows = iter::once(first_row).chain(following_rows.into_iter());
//!     let columns = rows.map(
//!         |row| {
//!             let (first_column, following_columns) = row.item();
//!             let columns = iter::once(first_column).chain(following_columns.into_iter());
//!             let line = columns.map(|column| column.span.as_str()).collect::<Vec<_>>().join(",");
//!             line
//!         }
//!     );
//!     let columns = columns.collect::<Vec<_>>().join("\n");
//!     assert_eq!(columns, input);
//!     Ok(())
//! }
//! ```
//!

#![doc = include_str!("../example-part2.md")]
#![cfg_attr(feature = "grammar-extras", doc = include_str!("../example-part3.md"))]
#![doc = include_str!("../example-part4.md")]
#![doc = include_str!("../example-part5.md")]

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
        no_warnigs,
    )
)]
pub fn derive_typed_parser(input: TokenStream) -> TokenStream {
    pest_typed_generator::derive_typed_parser(input.into(), true).into()
}
