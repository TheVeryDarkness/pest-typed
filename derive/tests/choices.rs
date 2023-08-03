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
c1  = { ("a"){1} }
c2  = { ("a" | "b"){2} }
c3  = { ("a" | "b" | "c"){3} }
c4  = { ("a" | "b" | "c" | "d"){4} }
c5  = { ("a" | "b" | "c" | "d" | "e"){5} }
c6  = { ("a" | "b" | "c" | "d" | "e" | "f"){6} }
c7  = { ("a" | "b" | "c" | "d" | "e" | "f" | "g"){7} }
c8  = { ("a" | "b" | "c" | "d" | "e" | "f" | "g" | "h"){8} }
c9  = { ("a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i"){9} }
c10 = { ("a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j"){10} }
c11 = { ("a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k"){11} }
c12 = { ("a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l"){12} }
"#]
struct Parser;

#[test]
fn main() -> Result<(), Error<Rule>> {
    pairs::c1::parse("a")?;
    pairs::c2::parse("ab")?;
    pairs::c3::parse("abc")?;
    pairs::c4::parse("abcd")?;
    pairs::c5::parse("abcde")?;
    pairs::c6::parse("abcdef")?;
    pairs::c7::parse("abcdefg")?;
    pairs::c8::parse("abcdefgh")?;
    pairs::c9::parse("abcdefghi")?;
    pairs::c10::parse("abcdefghij")?;
    pairs::c11::parse("abcdefghijk")?;
    pairs::c12::parse("abcdefghijkl")?;
    Ok(())
}
