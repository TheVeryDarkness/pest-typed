// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Types for typed parser.
#![no_std]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

extern crate alloc;
extern crate core;

mod pest;
pub mod predefined_node;
mod typed_node;
mod wrapper;
pub use crate::pest::{error::Error, lib::RuleType};
use crate::pest::{position, span, stack};
pub use typed_node::{NeverFailedTypedNode, ParsableTypedNode, TypedNode};
pub use wrapper::{RuleWrapper, StringStorage, StringWrapper};

/// A trait with a single method that parses strings into typed concrete syntax tree.
pub trait TypedParser<R: RuleType> {
    /// Parses a `&str` into a tree starting from T.
    #[allow(clippy::perf)]
    fn parse<'i, T: TypedNode<'i, R>>(input: &'i str) -> Result<T, Error<R>>;
}
