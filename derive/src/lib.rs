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
//! - Refer to <https://pest.rs> for pest's syntax and built-in rules.
//! - Refer to [`pest_typed_generator`] for how it generates codes.
//! - Refer to [`derive_typed_parser`] to see how to use the macro.

use proc_macro::TokenStream;

/// The main method that's called by the proc macro [`pest_typed_generator::derive_typed_parser`].
///
/// ## Basic usage
///
/// ```
/// extern crate alloc;
/// use core::{iter, result::Result};
/// use alloc::vec::Vec;
/// use pest_typed_derive::TypedParser;
/// use pest_typed::{ParsableTypedNode as _, TypedParser as _, error::Error};
///
/// /// See https://datatracker.ietf.org/doc/html/rfc4180.html for CSV's format.
/// #[derive(TypedParser)]
/// #[grammar = "../tests/csv.pest"]
/// #[emit_rule_reference]
/// struct Parser;
///
/// fn main() -> Result<(), Error<Rule>> {
///     let input = "name,age\nTom,10\nJerry,20";
///     let file = pairs::file::parse(input)?;
///     let (first_row, following_rows) = file.row();
///     let rows = iter::once(first_row).chain(following_rows.into_iter());
///     let columns = rows.map(
///         |row| {
///             let (first_column, following_columns) = row.item();
///             let columns = iter::once(first_column).chain(following_columns.into_iter());
///             let line = columns.map(|column| column.span.as_str()).collect::<Vec<_>>().join(",");
///             line
///         }
///     );
///     let columns = columns.collect::<Vec<_>>().join("\n");
///     assert_eq!(columns, input);
///     Ok(())
/// }
/// ```
///
/// ## Generated contents
///
/// This derive macro will:
/// - Implement [`pest_typed::TypedParser`] for dervied struct.
/// - Generate several modules.
///
/// ## Attributes
///
/// Attributes (see [pest](https://pest.rs) for more information):
/// - `grammar`: specify grammar file path.
/// - `grammar`: provide grammars in an inline string.
/// - `emit_rule_reference`: emit [accesser functions](#accesser-functions) for those rules referenced by current rule.
/// - `emit_tagged_node_reference`: emit [accesser functions](#accesser-functions) for those tagged nodes referenced by current rule.
///   Only takes effect when node tags are enabled (currently controlled by feature **grammar-extras**.).
///
/// ## Accesser functions
///
/// An accesser function is
///
#[proc_macro_derive(
    TypedParser,
    attributes(
        grammar,
        grammar_inline,
        emit_rule_reference,
        emit_tagged_node_reference
    )
)]
pub fn derive_typed_parser(input: TokenStream) -> TokenStream {
    pest_typed_generator::derive_typed_parser(input.into(), true).into()
}
