// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use pest_typed::ParsableTypedNode;
use pest_typed_derive::TypedParser;

#[allow(dead_code)]
#[derive(TypedParser)]
#[grammar_inline = r#"
x = { "a" ~ "b" ~ "c" ~ "d" ~ "e" ~ "f" ~ "g" ~ "h" ~ "i" ~ "j" ~ "k" ~ "l" ~ "m" ~ "n" ~ "o" ~ "p" }
y = { ("a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p"){16} }
"#]
struct Parser;

#[test]
fn parse() {
    pairs::x::try_parse("abcdefghijklmnop").unwrap();
    pairs::y::try_parse("abcdefghijklmnop").unwrap();
}
