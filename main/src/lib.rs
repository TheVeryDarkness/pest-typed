// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Types for statically typed nodes and parser.

#![no_std]

extern crate alloc;
extern crate core;
#[cfg(feature = "std")]
extern crate std;

pub mod predefined_node;
pub mod tracker;
mod typed_node;
mod wrapper;
use pest;
use pest::RuleType;
pub use typed_node::{NeverFailedTypedNode, ParsableTypedNode, TypedNode};
pub use wrapper::{RuleWrapper, Storage, StringArrayWrapper, StringWrapper, TypeWrapper};

// Below modules are copied from pest.
pub mod error;
mod parser_state;
mod position;
mod span;
mod stack;

use error::Error;
pub use position::Position;
pub use span::{merge_spans, Span};
pub use stack::Stack;

// Re-export unicode.
pub use pest::unicode;

/// A trait with a single method that parses strings into typed concrete syntax tree.
pub trait TypedParser<R: RuleType> {
    /// Parses a `&str` into a tree starting from T.
    #[allow(clippy::perf)]
    fn parse<'i, T: ParsableTypedNode<'i, R>>(input: &'i str) -> Result<T, Error<R>>;
}
