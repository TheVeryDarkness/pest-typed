// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use pest_typed::{ParsableTypedNode, Storage};
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

macro_rules! test {
    ($name:ident, $input:literal) => {
        mod $name {
            use super::{pairs, Rule};
            use pest_typed::{
                error::Error,
                iterators::{Pair, Pairs},
                ParsableTypedNode,
            };
            #[test]
            fn success() -> Result<(), Error<Rule>> {
                let res = pairs::$name::parse($input)?;
                let span = res.span;
                assert_eq!(span, res.iter_pairs().next().unwrap().span());
                assert_eq!(span, res.clone().into_iter_pairs().next().unwrap().span());
                assert!(res.inner().next().is_none());
                assert!(res.clone().into_inner().next().is_none());
                assert_eq!(res, res.clone());

                Ok(())
            }
            #[test]
            #[should_panic]
            fn failed() {
                let mut buf = String::from($input);
                buf.pop();
                pairs::$name::parse(buf.as_str()).unwrap();
            }
        }
    };
}
test!(c1, "a");
test!(c2, "ab");
test!(c3, "abc");
test!(c4, "abcd");
test!(c5, "abcde");
test!(c6, "abcdef");
test!(c7, "abcdefg");
test!(c8, "abcdefgh");
test!(c9, "abcdefghi");
test!(c10, "abcdefghij");
test!(c11, "abcdefghijk");
test!(c12, "abcdefghijkl");

#[test]
fn choices() {
    let c4 = pairs::c4::parse("abcd").unwrap();
    let (_0, _1, _2, _3) = c4.as_ref();
    macro_rules! t {
        ($branch:ident) => {
            $branch
                .if_then(|_0| assert_eq!(_0.get_content(), "a"))
                .else_if(|_1| assert_eq!(_1.get_content(), "b"))
                .else_if(|_2| assert_eq!(_2.get_content(), "c"))
                .else_then(|_3| assert_eq!(_3.get_content(), "d"))
        };
    }
    t!(_0);
    t!(_1);
    t!(_2);
    t!(_3);
}
