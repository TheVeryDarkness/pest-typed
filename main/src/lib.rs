// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Types for statically typed nodes and parser.
//!
//! Based on [pest] grammar. See <https://pest.rs> for more information.
//!
//! Features:
//!
//! - `std`: include [`std`] support.
//!
//!   Without this feature, we'll use [core] and [alloc].
//!
//! It's suggested that you use [pest_typed_derive](https://crates.io/pest_typed_derive) to automatically generate types from your grammar.
//!
//! And though we have a lot of macros in this crate, only some of them are designed for usage outside the crate. They're listed below:
//!
//! - Choices: [choices!].
//! - Sequence: [seq!].
//! - Rules:
//!     - Atomic rule: [atomic_rule!].
//!     - Non-atomic rule: [non_atomic_rule!].
//!     - Compound atomic rule: [compound_atomic_rule!].
//!     - Normal rule: [normal_rule!].
//!     - Silent rule: [silent_rule!].
//!     - End-Of-Input rule: [rule_eoi!].
//!     - Tag: [tag!].

#![no_std]
#![warn(
    missing_docs,
    rust_2018_idioms,
    rustdoc::all,
    unused_qualifications,
    future_incompatible
)]
#![deny(unconditional_recursion)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use core::borrow::Borrow;

use line_indexer::LineIndexer;
use typed_node::NeverFailedParsableTypedNode;
pub use typed_node::{NeverFailedTypedNode, ParsableTypedNode, RuleStruct, Spanned, TypedNode};
pub use wrapper::{
    BoundWrapper, ConstantStorage, RuleWrapper, Storage, StringArrayWrapper, StringWrapper,
    TypeWrapper,
};

pub mod choices;
mod formatter;
mod input;
pub mod iterators;
pub mod line_indexer;
mod pratt;
pub mod predefined_node;
pub mod re_exported;
pub mod rule;
pub mod sequence;
pub mod tracker;
mod typed_node;
mod wrapper;

pub use alloc::boxed::Box;

// Below modules are copied from pest.
mod parser_state;
mod position;
mod span;

pub use input::{AsInput, Input};
pub use pest::{error, Stack};
pub use position::Position;
pub use span::{merge_spans, Span};

// Re-export unicode.
pub use pest::unicode;

/// A trait that is implemented for all pest rules. An extension of [`pest::RuleType`].
pub trait RuleType: pest::RuleType {
    /// The name of the rule.
    ///
    /// Could be a `const` `fn` if stabilized.
    fn name(&self) -> &'static str;
}

/// A trait with a single method that parses strings into typed concrete syntax tree.
pub trait TypedParser<R: RuleType> {
    /// Try to parse a `&str` into a tree starting from T.
    fn try_parse<'i, T: ParsableTypedNode<'i, R, str>>(
        input: &'i str,
    ) -> Result<T, Box<error::Error<R>>> {
        T::try_parse(input)
    }
    /// Parse a `&str` into a tree starting from T.
    fn parse<'i, T: NeverFailedParsableTypedNode<'i, R, str>>(input: &'i str) -> T {
        T::parse(input)
    }
    /// Check whether a `&str` can be parsed into a tree starting from T.
    fn try_check<'i, T: ParsableTypedNode<'i, R, str>>(
        input: &'i str,
    ) -> Result<(), Box<error::Error<R>>> {
        T::try_check(input)
    }
}
