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
//! See [pest_typed](https://docs.rs/pest_typed/latest/pest_typed/) for related traits and types.

#![warn(
    missing_docs,
    rust_2018_idioms,
    rustdoc::all,
    unused_qualifications,
    future_incompatible
)]

mod config;
mod graph;
mod typed;
mod match_choices;
pub use typed::derive_typed_parser;
pub use match_choices::match_choices;

// Below modules are copied from pest and modified.
mod docs;
mod generator;
mod helper;
mod types;
