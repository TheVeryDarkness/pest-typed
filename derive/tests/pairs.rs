// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

extern crate alloc;
use pest_typed::{error::Error, ParsableTypedNode};
use pest_typed_derive::TypedParser;

#[derive(TypedParser)]
#[grammar = "tests/csv.pest"]
#[emit_rule_reference]
struct CSV;

#[test]
fn pairs() -> Result<(), Error<Rule>> {
    let _res = pairs::file::parse("a,b,c\nd,e,f")?;
    // Not finished.
    Ok(())
}
