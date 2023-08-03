// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use pest_typed::{error::Error, ParsableTypedNode};
use pest_typed_derive::TypedParser;

#[derive(TypedParser)]
#[grammar_inline = r#"
s1  = { "a" }
s2  = { "a" ~ "b" }
s3  = { "a" ~ "b" ~ "c" }
s4  = { "a" ~ "b" ~ "c" ~ "d" }
s5  = { "a" ~ "b" ~ "c" ~ "d" ~ "e" }
s6  = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" }
s7  = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" }
s8  = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" ~ "h" }
s9  = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" ~ "h" ~ "i" }
s10 = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" ~ "h" ~ "i" ~ "j" }
s11 = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" ~ "h" ~ "i" ~ "j" ~ "k" }
s12 = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" ~ "h" ~ "i" ~ "j" ~ "k" ~ "l" }
"#]
struct Parser;

#[test]
fn main() -> Result<(), Error<Rule>> {
    pairs::s1::parse("a")?;
    pairs::s2::parse("ab")?;
    pairs::s3::parse("abc")?;
    pairs::s4::parse("abcd")?;
    pairs::s5::parse("abcde")?;
    pairs::s6::parse("abcdef")?;
    pairs::s7::parse("abcdefg")?;
    pairs::s8::parse("abcdefgh")?;
    pairs::s9::parse("abcdefghi")?;
    pairs::s10::parse("abcdefghij")?;
    pairs::s11::parse("abcdefghijk")?;
    pairs::s12::parse("abcdefghijkl")?;
    Ok(())
}
