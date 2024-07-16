// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use pest_typed::{iterators::PairTree, ParsableTypedNode};
use pest_typed_derive::TypedParser;

#[allow(dead_code)]
#[derive(TypedParser)]
#[grammar_inline = r#"
x = _{ "x" ~ y }
y = _{ "y" | x }
z =  { x }
"#]
struct Parser;

#[test]
fn main() {
    let z = pairs::z::try_parse("xxxy").unwrap();
    let mut buf = String::new();
    z.write_tree_to(&mut buf).unwrap();
    assert_eq!(buf, "z \"xxxy\"\n");
}
