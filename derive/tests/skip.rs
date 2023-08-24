// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use pest_typed::{error::Error, ParsableTypedNode as _};
use pest_typed_derive::TypedParser;

#[derive(TypedParser)]
#[grammar_inline = r#"
WHITESPACE = { " " }
COMMENT    = { "/*" ~ (!"*/" ~ ANY)* ~ "*/" }
main       = { "x"* }
program    = { SOI ~ main ~ EOI }
"#]
struct Parser;

#[test]
fn comment() -> Result<(), Error<Rule>> {
    let vec = pairs::main::parse("x x x /*x*/")?;
    assert_eq!(vec.iter_matched().len(), 3);
    Ok(())
}

#[test]
fn skip_on_two_end() {
    pairs::main::parse(" x x").unwrap_err();
    pairs::main::parse("x x ").unwrap();
}

#[test]
fn post_skip() -> Result<(), Error<Rule>> {
    let program = pairs::program::parse("x x /*x*/")?;
    let (_soi, main, _eoi) = program.get_matched();
    assert_eq!(main.iter_matched().len(), 2);
    Ok(())
}

#[test]
fn pre_skip() -> Result<(), Error<Rule>> {
    let program = pairs::program::parse("/* x x */ x x")?;
    let (_soi, main, _eoi) = program.get_matched();
    assert_eq!(main.iter_matched().len(), 2);
    Ok(())
}
