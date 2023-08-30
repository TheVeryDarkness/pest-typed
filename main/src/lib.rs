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
//! Features:
//!
//! - `std`: include [`std`] support.
//! - ~`verbose`~ (TODO): provide verbose error messages.

#![no_std]
#![warn(
    missing_docs,
    rust_2018_idioms,
    rustdoc::all,
    unused_qualifications,
    future_incompatible
)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub use pest::RuleType;
pub use typed_node::{NeverFailedTypedNode, ParsableTypedNode, RuleStruct, TypedNode};
pub use wrapper::{
    BoundWrapper, ConstantStorage, RuleWrapper, Storage, StringArrayWrapper, StringWrapper,
    TypeWrapper,
};

pub mod choices;
pub mod iterators;
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

pub use pest::error;
pub use position::Position;
pub use span::{merge_spans, Span};
pub use pest::Stack;

// Re-export unicode.
pub use pest::unicode;

/// A trait with a single method that parses strings into typed concrete syntax tree.
pub trait TypedParser<R: RuleType> {
    /// Parses a `&str` into a tree starting from T.
    #[allow(clippy::perf)]
    fn parse<'i, T: ParsableTypedNode<'i, R>>(input: &'i str) -> Result<T, error::Error<R>>;
}
