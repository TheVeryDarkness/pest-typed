// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

//! Derive typed nodes from pest grammar.

use proc_macro::TokenStream;

/// The main method that's called by the proc macro
/// (a wrapper around `pest_generator::derive_parser`)
#[proc_macro_derive(TypedParser, attributes(grammar, grammar_inline))]
pub fn derive_typed_parser(input: TokenStream) -> TokenStream {
    pest_typed_generator::derive_parser(input.into(), true).into()
}
