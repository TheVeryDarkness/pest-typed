// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! A code generator based on [`pest`].
//!
//! It can create corresponding type definitions from pest grammar files.
//!
//! See <https://docs.rs/pest_typed/latest/pest_typed/predefined_node/> for generated types.

mod graph;
mod typed;
pub use typed::derive_typed_parser;

// Below modules are copied from pest and modified.
mod docs;
mod generator;
mod helper;
pub mod types;
