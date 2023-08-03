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

#[derive(TypedParser)]
#[grammar_inline = r#"
main = { (pre? ~ val) ~ (mid ~ pre? ~ val)* }
pre  = { "-" }
val  = { ('0'..'9')+ }
mid  = { "+" | "-" | "*" | "/" }
"#]
struct Parser;

#[cfg(feature = "std")]
#[test]
fn tree() {
    /// Note that [`indoc!`] is used.
    macro_rules! test {
        ($input:literal, $tree:literal) => {
            let tree = pairs::main::parse($input).unwrap();
            let mut string = String::from("");
            tree.write_tree_to(&mut string).unwrap();
            assert_eq!(string, $tree);
        };
    }

    test!(
        "1*2",
        r#"main
    val "1"
    mid "*"
    val "2"
"#
    );
    test!(
        "-1*2",
        r#"main
    pre "-"
    val "1"
    mid "*"
    val "2"
"#
    );
    test!(
        "-1+2",
        r#"main
    pre "-"
    val "1"
    mid "+"
    val "2"
"#
    );
    test!(
        "4/2",
        r#"main
    val "4"
    mid "/"
    val "2"
"#
    );
    test!(
        "12/2",
        r#"main
    val "12"
    mid "/"
    val "2"
"#
    );
}
